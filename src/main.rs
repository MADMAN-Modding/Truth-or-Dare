use std::env;

use dotenv::dotenv;
use serenity::all::{CommandInteraction, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, MessageComponentInteractionMetadata};
use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::Timestamp;
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
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            // The create message builder allows you to easily create embeds and messages using a
            // builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, an image, three fields, and a footer.
            // let footer = CreateEmbedFooter::new("This is a footer");
            let embed = CreateEmbed::new()
                .title("Truth or Dare")
                .description("DARE_OR_TRUTH")
                .image("attachment://ferris_eyes.png")
                // .fields(vec![
                //     ("This is the first field", "This is a field body", true),
                //     ("This is the second field", "Both fields are inline", true),
                // ])
                // .field("This is the third field", "This is not an inline field", false)
                // .footer(footer)
                // Add a timestamp for the current time
                // This also accepts a rfc3339 Timestamp
                .timestamp(Timestamp::now());

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
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn handle_command(interaction: CommandInteraction) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(format!(
        "Hello from interactions webhook HTTP server! <@{}>",
        interaction.user.id
    )))
}