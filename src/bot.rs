use serenity::all::{
    Command, Context, CreateActionRow, CreateMessage, EventHandler, GuildId, Interaction, Message, Ready
};
use serenity::async_trait;

use crate::commands::{
    add_question, create_commands, list_custom_questions, list_questions, remove_question, set_question_permissions, set_rating
};
use crate::embed::{dare_button, embed_text, truth_button};
use crate::interactions::{next_page, previous_page, truth_or_dare};
use crate::other_impl::MessageMaker;
use crate::questions::{Question, QuestionType};

pub struct Bot {
    pub database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let question_type: QuestionType = match msg.content.as_str().trim() {
            "!truth" => QuestionType::TRUTH,
            "!dare" => QuestionType::DARE,
            _ => return,
        };

        let embed = embed_text(
            &self,
            question_type,
            self.get_guild_rating(msg.guild_id).await,
            msg.guild_id,
        )
        .await;

        let row = CreateActionRow::Buttons(vec![truth_button(), dare_button()]);

        let builder = CreateMessage::new().embed(embed).components(vec![row]);

        let msg = msg.channel_id.send_message(&ctx.http, builder).await;

        if let Err(why) = msg {
            eprintln!("Error sending message: {why:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(component_interaction) = interaction.clone().message_component() {
            let response = match component_interaction.data.custom_id.as_str() {
                "truth" | "dare" => {
                    truth_or_dare(
                        self,
                        &component_interaction.data.custom_id,
                        component_interaction.guild_id.unwrap(),
                    )
                    .await
                }
                // Next Page of Question List
                interaction if interaction.contains("next_page-") => {
                    // Delete the original message
                    if let Err(e) = component_interaction.message.delete(&ctx.http).await {
                        eprintln!("Failed to delete message: {e:?}");
                    }
                    next_page(&self, interaction, component_interaction.guild_id).await
                }
                // Previous Page of Question List
                interaction if interaction.contains("previous_page-") => {
                    // Delete the original message
                    if let Err(e) = component_interaction.message.delete(&ctx.http).await {
                        eprintln!("Failed to delete message: {e:?}");
                    }
                    previous_page(&self, interaction, component_interaction.guild_id).await
                }
                _ => "Uh, you shouldn't have seen this...".to_interaction_message(),
            };

            if let Err(why) = component_interaction
                .create_response(&ctx.http, response)
                .await
            {
                eprintln!("Failed to respond to interaction : {why:?}")
            }
        }

        // Command interactions
        if let Some(command) = interaction.command() {
            match command.data.name.as_str() {
                "set_rating" => {
                    command
                        .create_response(&ctx.http, set_rating(self, &command).await)
                        .await
                        .ok();
                }
                "add_question" => {
                    command
                        .create_response(&ctx.http, add_question(self, &command).await)
                        .await
                        .ok();
                }
                "list_questions" => {
                    command
                        .create_response(&ctx.http, list_questions(self, command.guild_id).await)
                        .await
                        .ok();
                }
                "list_custom_questions" => {
                    command
                        .create_response(
                            &ctx.http,
                            list_custom_questions(&self, command.guild_id).await,
                        )
                        .await
                        .ok();
                }
                "set_question_permissions" => {
                    command
                        .create_response(&ctx.http, set_question_permissions(self, &command).await)
                        .await
                        .ok();
                }
                "remove_question" => {
                    command.create_response(&ctx.http, remove_question(self, &command).await)
                    .await
                    .ok();
                }
                _ => {}
            }
        }
    }

    /// Runs when the bot is connected to Discord
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Lists all the commands and their ids
        // for command in Command::get_global_commands(&ctx.http).await.unwrap() {
        //     println!("{}-{}", command.name, command.id);
        // }

        for command in create_commands() {
            Command::create_global_command(&ctx.http, command).await.unwrap();
        }
    }
}

impl Bot {
    /// Retrieves a random question from the database based on the specified question type and rating.
    /// Returns `None` if no question is found.
    pub async fn get_random_question(
        &self,
        question_type: QuestionType,
        question_rating: &str,
        guild_id: i64,
    ) -> Result<Option<Question>, sqlx::Error> {
        // !TODO - guild specific
        let query = r#"
            SELECT * FROM questions
            WHERE question_type = ?1 AND rating = ?2 AND (guild_id = ?3 OR guild_id IS NULL)
            ORDER BY RANDOM()
            LIMIT 1
        "#;

        let question = sqlx::query_as::<sqlx::Sqlite, Question>(query)
            .bind(&question_type.to_string())
            .bind(question_rating)
            .bind(guild_id as i64)
            .fetch_optional(&self.database)
            .await?;
        
        Ok(question)
    }

    pub async fn set_guild_rating(&self, guild_id: i64, rating: &str) -> Result<(), sqlx::Error> {
        println!("{}", rating);

        sqlx::query(
            r#"
            INSERT INTO guild_settings (guild_id, rating)
            VALUES (?, ?)
            ON CONFLICT(guild_id) DO UPDATE SET rating = excluded.rating
            "#,
        )
        .bind(guild_id)
        .bind(rating)
        .execute(&self.database)
        .await?;

        Ok(())
    }

    pub async fn get_guild_rating(&self, guild_id: Option<GuildId>) -> String {
        // Default the guild_rating to PG
        if guild_id.is_none() {
            "PG".to_string()
        } else {
            // Query the rating for the guild_id
            let result = sqlx::query_scalar::<_, String>(
                r#"
            SELECT rating FROM guild_settings
            WHERE guild_id = ?
            "#,
            )
            .bind(guild_id.unwrap().get() as i64)
            .fetch_optional(&self.database)
            .await;

            match result.unwrap() {
                Some(rating) => rating,
                _ => "PG".to_string(),
            }
        }
    }

    /// Sets the question permissions for a guild.
    ///
    /// # Parameters
    /// * `guild_id: Option<GuildId>` - The guild to set permissions for
    /// * `can_add_questions: bool` - Whether the guild can add questions
    pub async fn set_guild_question_permissions(
        &self,
        guild_id: Option<GuildId>,
        admin: bool,
    ) -> Result<(), sqlx::Error> {
        let rating = self.get_guild_rating(guild_id).await;

        if let Some(guild_id) = guild_id {
            match sqlx::query(
                r#"
                INSERT INTO guild_settings (guild_id, rating, admin)
                VALUES (?, ?, ?)
                ON CONFLICT(guild_id) DO UPDATE SET admin = excluded.admin
                "#,
            )
            .bind(guild_id.get() as i64)
            .bind(rating)
            .bind(admin)
            .execute(&self.database)
            .await {
                Err(e) => eprint!("{e}"),
                _ => ()
            }
        }
        Ok(())
    }

    pub async fn get_guild_question_permissions(&self, guild_id: Option<GuildId>) -> bool {
        // Default the guild_rating to PG
        if guild_id.is_none() {
            false
        } else {
            // Query the rating for the guild_id
            let result = sqlx::query_scalar::<_, i64>(
                r#"
            SELECT admin FROM guild_settings
            WHERE guild_id = ?
            "#,
            )
            .bind(guild_id.unwrap().get() as i64)
            .fetch_optional(&self.database)
            .await;

            match result {
                Ok(val) => match val {
                    
                    Some(admin) => admin == 1,
                    _ => false,
                },
                Err(e) => {eprintln!("{e}"); false}
            }
        }
    }

    /// Gets all questions in the provided guild and in the default questions
    ///
    /// # Parameters
    /// * `guild_id: Option<GuildId>` - Wrapped guild id to check
    ///
    /// # Returns
    /// * `Vec<Questions>` - A list of all the questions
    pub async fn get_questions(&self, guild_id: Option<GuildId>) -> Vec<Question> {
        if guild_id.is_none() {
            Vec::new()
        } else {
            let query = r#"
                SELECT * FROM questions WHERE guild_id = ?1 OR guild_id IS NULL
                "#;

            let questions = sqlx::query_as::<_, Question>(query)
                .bind(guild_id.unwrap().get() as i64)
                .fetch_all(&self.database)
                .await
                .ok();

            questions.unwrap()
        }
    }

    /// Gets all questions in the provided guild
    ///
    /// # Parameters
    /// * `guild_id: Option<GuildId>` - Wrapped guild id to check
    ///
    /// # Returns
    /// * `Vec<Questions>` - A list of all the questions
    pub async fn get_custom_questions(&self, guild_id: Option<GuildId>) -> Vec<Question> {
        if guild_id.is_none() {
            Vec::new()
        } else {
            let query = r#"
                SELECT * FROM questions WHERE guild_id = ?1
                "#;

            let questions = sqlx::query_as::<_, Question>(query)
                .bind(guild_id.unwrap().get() as i64)
                .fetch_all(&self.database)
                .await
                .ok();

            questions.unwrap()
        }
    }

    pub async fn check_question_guild(&self, guild_id: Option<GuildId>, question_uid: &String) -> bool {
        if guild_id.is_none() {
            return false;
        } else {
            let query = r#"
                SELECT * FROM questions WHERE guild_id = ?1 AND uid = ?2 LIMIT 1
                "#;

            let question = sqlx::query_as::<_, Question>(query)
                .bind(guild_id.unwrap().get() as i64)
                .bind(question_uid)
                .fetch_optional(&self.database)
                .await;

            match question.unwrap() {
                Some(val) => {println!("Prompt: {}", val.prompt); true},
                None => false
            }
        }
    }
}
