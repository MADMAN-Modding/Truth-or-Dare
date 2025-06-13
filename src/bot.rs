use serenity::all::{
    Command, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId,
    Interaction,
};

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands::create_commands;
use crate::embed::{dare_button, embed_text, truth_button};
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
        )
        .await;

        let row = CreateActionRow::Buttons(vec![truth_button(), dare_button()]);

        let builder = CreateMessage::new().embed(embed).components(vec![row]);

        let msg = msg.channel_id.send_message(&ctx.http, builder).await;

        if let Err(why) = msg {
            println!("Error sending message: {why:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(component_interaction) = interaction.clone().message_component() {
            let question_type: QuestionType = match component_interaction.data.custom_id.as_str() {
                "truth" => QuestionType::TRUTH,
                "dare" => QuestionType::DARE,
                _ => QuestionType::NONE,
            };

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(
                        embed_text(
                            &self,
                            question_type,
                            self.get_guild_rating(component_interaction.guild_id).await,
                        )
                        .await,
                    )
                    .button(truth_button())
                    .button(dare_button()),
            );

            if let Err(why) = component_interaction
                .create_response(&ctx.http, response)
                .await
            {
                println!("Failed to respond to interaction : {why:?}")
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
                            .create_response(&ctx.http, "Failed to set rating.".to_message())
                            .await
                            .ok();
                        return;
                    }
                }

                command
                    .create_response(&ctx.http, format!("Rating set to {}.", rating).to_message())
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
    pub async fn get_random_question(
        &self,
        question_type: QuestionType,
        question_rating: &str,
    ) -> Option<Question> {
        let query = r#"
            SELECT * FROM questions
            WHERE question_type = ?1 AND rating = ?2
            ORDER BY RANDOM()
            LIMIT 1
        "#;

        sqlx::query_as::<_, Question>(query)
            .bind(question_type.to_string())
            .bind(question_rating.to_string())
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
        if guild_id.is_none() {
            return "PG".to_string()
        } else {
            let reseult = sqlx::query_scalar::<_, String>(
                r#"
            SELECT rating FROM guild_settings
            WHERE guild_id = ?
            "#,
            )
            .bind(guild_id.unwrap().get() as i64)
            .fetch_optional(&self.database)
            .await;

            match reseult.unwrap() {
                Some(rating) => rating,
                _ => "PG".to_string(),
            }
        }
    }
}
