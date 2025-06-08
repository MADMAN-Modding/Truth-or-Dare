use std::env;

use dotenv::dotenv;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction
};
use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateMessage};
use serenity::model::Timestamp;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

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
        let embed = embed_text().await;

        let truth_button = CreateButton::new("truth")
            .label("Truth")
            .style(serenity::model::prelude::ButtonStyle::Primary);

        let dare_button = CreateButton::new("dare")
            .label("Dare")
            .style(serenity::model::prelude::ButtonStyle::Danger);

        let row = CreateActionRow::Buttons(vec![truth_button, dare_button]);

        let builder = CreateMessage::new()
            .content("Hello, World!")
            .embed(embed)
            .components(vec![row])
            .add_file(CreateAttachment::path("./ferris_eyes.png").await.unwrap());

        if msg.content == "!hello" {
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("{}", interaction.id());
        
        if let Some(component_interaction) = interaction.message_component() {
            println!("{}", component_interaction.user.mention().to_string());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed_text().await).button(truth_button()).button(dare_button()),
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

async fn embed_text() -> CreateEmbed {
    // The create message builder allows you to easily create embeds and messages using a
    // builder syntax.
    // This example will create a message that says "Hello, World!", with an embed that has
    // a title, description, an image, three fields, and a footer.
    // let footer = CreateEmbedFooter::new("This is a footer");
    let embed = CreateEmbed::new()
        .title("Truth or Dare")
        .description("DARE_OR_TRUTH")
        .image("attachment://ferris_eyes.png")
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        .timestamp(Timestamp::now());

    embed
}

fn truth_button() -> CreateButton {
    make_button("truth", "Truth", ButtonStyle::Primary)
}

fn dare_button() -> CreateButton {
    make_button("dare", "Dare", ButtonStyle::Danger)
}

fn make_button(id: &str, label: &str, style: ButtonStyle) -> CreateButton {
    CreateButton::new(id)
        .label(label)
        .style(style)
}