use serenity::all::{Command, Context, CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, EventHandler, GuildId, Interaction, Message, Ready};
use serenity::async_trait;

use crate::commands::create_commands;
use crate::embed::{dare_button, embed_text, send_page, truth_button};
use crate::interactions::{next_page, previous_page, truth_or_dare};
use crate::other_impl::MessageMaker;
use crate::questions::{Question, QuestionType};

use std::str::FromStr;

pub struct Bot {
    pub database: sqlx::SqlitePool
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
            msg.guild_id
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
                "truth" | "dare" => truth_or_dare(self, &component_interaction.data.custom_id, component_interaction.guild_id.unwrap()).await,
                // Next Page of Question List
                interaction if interaction.contains("next_page-") => {
                    next_page(&self, interaction, component_interaction.guild_id).await
                },
                // Previous Page of Question List
                interaction if interaction.contains("previous_page-") => {
                    previous_page(&self, interaction, component_interaction.guild_id).await
                }
                _ => CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().add_embed(CreateEmbed::new().description("Uh, you shouldn't have seen this..."))),
            };

            if let Err(why) = component_interaction
                .create_response(&ctx.http, response)
                .await
            {
                eprintln!("Failed to respond to interaction : {why:?}")
            }
        }

        if let Some(command) = interaction.command() {
            if command.data.name == "set_rating" {
                let rating = command
                    .data
                    .options
                    .iter()
                    .find(|option| option.name == "rating")
                    .and_then(|option| option.value.as_str())
                    .unwrap_or("PG");

                if let Some(guild_id) = command.guild_id {
                    let guild_id_i64 = guild_id.get() as i64;

                    if let Err(err) = self.set_guild_rating(guild_id_i64, rating).await {
                        eprintln!("Failed to set guild rating: {err}");
                        command
                            .create_response(&ctx.http, "Failed to set rating.".to_interaction_message())
                            .await
                            .ok();
                        return;
                    }
                }

                command
                    .create_response(&ctx.http, format!("Rating set to {}.", rating).to_interaction_message())
                    .await
                    .ok();
            } else if command.data.name == "add_question" {
                let get_option = |name| {
                    command
                        .data
                        .options
                        .iter()
                        .find(|o| o.name == name)
                        .and_then(|o| o.value.as_str())
                };
                // Sanitize input to remove potentially dangerous characters
                let question = get_option("question")
                    .unwrap_or("");

                let question_type = get_option("question_type")
                    .and_then(|s| QuestionType::from_str(s.to_uppercase().as_str()).ok())
                    .unwrap_or(QuestionType::NONE);
                let rating = get_option("rating").unwrap_or("PG");

                if question.is_empty() {
                    command
                        .create_response(&ctx.http, "Question cannot be empty.".to_interaction_message())
                        .await
                        .ok();
                } else {
                    sqlx::query(
                        r#"INSERT INTO questions (prompt, question_type, rating, guild_id) VALUES (?1, ?2, ?3, ?4)"#,
                    )
                    .bind(&question)
                    .bind(question_type.to_string())
                    .bind(rating)
                    .bind(command.guild_id.unwrap().get() as i64)
                    .execute(&self.database)
                    .await
                    .ok();

                    command
                        .create_response(
                            &ctx.http,
                            format!("Question added: {}", question).to_interaction_message(),
                        )
                        .await
                        .ok();
                }
            } else if command.data.name == "list_questions" {
                let questions = self.get_questions(command.guild_id).await;

                // Send the response
                let message = send_page(1, questions).await;
                command
                    .create_response(&ctx.http, message)
                    .await
                    .ok();
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for command in create_commands() {
            Command::create_global_command(&ctx, command).await.unwrap();
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
        guild_id: i64
    ) -> Option<Question> {
        // !TODO - guild specific
        let query = r#"
            SELECT * FROM questions
            WHERE question_type = ?1 AND rating = ?2 AND guild_id = ?3 OR guild_id IS NULL
            ORDER BY RANDOM()
            LIMIT 1
        "#;

        sqlx::query_as::<_, Question>(query)
            .bind(question_type.to_string())
            .bind(question_rating.to_string())
            .bind(guild_id)
            .fetch_optional(&self.database)
            .await
            .ok()
            .flatten()
    }

    pub async fn set_guild_rating(&self, guild_id: i64, rating: &str) -> Result<(), sqlx::Error> {
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

    pub async fn get_questions(&self, guild_id: Option<GuildId>) -> Vec<Question> {
        if guild_id.is_none() {
            Vec::new()
        } else {
            let query = r#"
                SELECT * FROM questions WHERE guild_id = ?1 OR guild_id IS NULL
                "#;

            let questions = sqlx::query_as::<_, Question>(query).bind(guild_id.unwrap().get() as i64).fetch_all(&self.database).await.ok();

            questions.unwrap()
        }
    }
}
