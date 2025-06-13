use serenity::all::{
    CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
};

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::embed::{dare_button, embed_text, truth_button};
use crate::questions::QuestionType;

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

        let embed = embed_text(question_type).await;

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
                    .embed(embed_text(question_type).await)
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
