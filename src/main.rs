use std::env;

use dotenv::dotenv;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateInteractionResponse,
    CreateInteractionResponseMessage, Interaction,
};
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

enum QuestionType {
    TRUTH,
    DARE,
    NONE
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let question_type: QuestionType = match msg.content.as_str().trim() {
            "!truth" => QuestionType::TRUTH,
            "!dare" => QuestionType::DARE,
            _ => return,
        };

        let embed = embed_text(question_type).await;

        let truth_button = CreateButton::new("truth")
            .label("Truth")
            .style(serenity::model::prelude::ButtonStyle::Primary);

        let dare_button = CreateButton::new("dare")
            .label("Dare")
            .style(serenity::model::prelude::ButtonStyle::Danger);

        let row = CreateActionRow::Buttons(vec![truth_button, dare_button]);

        let builder = CreateMessage::new()
            .embed(embed)
            .components(vec![row]);

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
                _ => QuestionType::NONE
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

async fn embed_text(question_type: QuestionType) -> CreateEmbed {
    let description = match question_type {
        QuestionType::TRUTH => "truth",
        QuestionType::DARE => "dare",
        QuestionType::NONE => "Error"
    };

    let embed = CreateEmbed::new()
        .title("Truth or Dare")
        .description(description)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        .timestamp(Timestamp::now());

    embed
}

/// Returns a `CreateButton` for Truths
fn truth_button() -> CreateButton {
    make_button("truth", "Truth", ButtonStyle::Primary)
}

/// Returns a `CreateButton` for Dares
fn dare_button() -> CreateButton {
    make_button("dare", "Dare", ButtonStyle::Danger)
}

/// Makes a button based on provided input
///
/// # Parameters
/// `id: &str` - ID of the button
/// `label: &str` - Text to be displayed on the button
/// `style: ButtonStyle` - Style of the button
fn make_button(id: &str, label: &str, style: ButtonStyle) -> CreateButton {
    CreateButton::new(id).label(label).style(style)
}
