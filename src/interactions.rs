use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, GuildId};

use crate::{bot::Bot, embed::{dare_button, embed_text, truth_button}, questions::QuestionType};

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