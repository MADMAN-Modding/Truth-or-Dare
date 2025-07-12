use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, GuildId};

use crate::{bot::Bot, embed::{dare_button, embed_text, send_page, truth_button}, menu_type::MenuType, other_impl::FindMenuType, questions::QuestionType};

pub async fn truth_or_dare(bot: &Bot, action: &str, guild_id: Option<GuildId>) -> CreateInteractionResponse {
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
                    bot.get_guild_rating(guild_id).await,
                    guild_id
                )
                .await,
            )
            .button(truth_button())
            .button(dare_button()),
    );

    response
}

/// Sends the next page of the menu
/// 
/// # Parameters
/// * `bot: &Bot` - Used to access the database
/// * `interaction: &str` - The interaction sent by the client
/// * `guild_id: Option<GuildId>` - Guild Id of the guild the interaction came from
pub async fn next_page(bot: &Bot, interaction: &str, guild_id: Option<GuildId>) -> CreateInteractionResponse{
    let end = interaction.find("-").unwrap();

    let after_dash = &interaction[end+1..];
    let num_str: String = after_dash.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let num: usize = num_str.parse().unwrap();
    let menu_type = interaction.to_menu_type();

    let questions = match menu_type {
        MenuType::CUSTOM => bot.get_custom_questions(guild_id).await,
        MenuType::DEFAULT => bot.get_questions(guild_id).await
    };

    send_page(num+1, questions, menu_type).await
}

/// Sends the previous page of the menu
/// 
/// # Parameters
/// * `bot: &Bot` - Used to access the database
/// * `interaction: &str` - The interaction sent by the client
/// * `guild_id: Option<GuildId>` - Guild Id of the guild the interaction came from
pub async fn previous_page(bot: &Bot, interaction: &str, guild_id: Option<GuildId>) -> CreateInteractionResponse {
    let end = interaction.find("-").unwrap();

    let after_dash = &interaction[end+1..];
    let num_str: String = after_dash.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let num: usize = num_str.parse().unwrap();
    let menu_type = interaction.to_menu_type();

    let questions = match menu_type {
        MenuType::CUSTOM => bot.get_custom_questions(guild_id).await,
        MenuType::DEFAULT => bot.get_questions(guild_id).await
    };

    send_page(num, questions, menu_type).await
}