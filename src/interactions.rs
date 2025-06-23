use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, GuildId};

use crate::{bot::Bot, embed::{dare_button, embed_text, send_page, truth_button}, questions::QuestionType};

pub async fn truth_or_dare(bot: &Bot, action: &str, guild_id: GuildId) -> CreateInteractionResponse {
    let question_type: QuestionType = match action {
        "truth" => QuestionType::TRUTH,
        "dare" => QuestionType::DARE,
        _ => QuestionType::NONE,
    };

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(
                embed_text(
                    &bot,
                    question_type,
                    bot.get_guild_rating(Some(guild_id)).await,
                )
                .await,
            )
            .button(truth_button())
            .button(dare_button()),
    );

    response
}

pub async fn next_page(bot: &Bot, interaction: &str, guild_id: Option<GuildId>) -> CreateInteractionResponse{
    let end = interaction.find("-").unwrap();

    let num: usize = interaction.split_at(end+1).1.parse().unwrap();

    send_page(num+1, bot.get_questions(guild_id).await).await
}

pub async fn previous_page(bot: &Bot, interaction: &str, guild_id: Option<GuildId>) -> CreateInteractionResponse {
    let end = interaction.find("-").unwrap();

    let num: usize = interaction.split_at(end+1).1.parse().unwrap();

    send_page(num , bot.get_questions(guild_id).await).await 

}