use serenity::all::{
    CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
};

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;


use crate::embed::{dare_button, embed_text, truth_button};
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

        let embed = embed_text(&self, question_type, "PG-13").await;

        let row = CreateActionRow::Buttons(vec![truth_button(), dare_button()]);

        let builder = CreateMessage::new().embed(embed).components(vec![row]);

        let msg = msg.channel_id.send_message(&ctx.http, builder).await;

        if let Err(why) = msg {
            println!("Error sending message: {why:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(component_interaction) = interaction.message_component() {
            println!("{}", component_interaction.user.mention().to_string());

            let question_type: QuestionType = match component_interaction.data.custom_id.as_str() {
                "truth" => QuestionType::TRUTH,
                "dare" => QuestionType::DARE,
                _ => QuestionType::NONE,
            };

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed_text(&self, question_type, "PG-13").await)
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
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

impl Bot {
    pub async fn get_random_question(&self, question_type: QuestionType, question_rating: &str) -> Option<Question> {
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
}
