use serenity::all::{ButtonStyle, CreateButton, CreateEmbed, Timestamp};

use crate::{bot::Bot, questions::QuestionType};

pub async fn embed_text(bot: &Bot, question_type: QuestionType) -> CreateEmbed {
    let description = match bot.get_random_question(question_type).await {
        Some(question) => question.prompt,
        None => format!("No {:?} questions found", question_type)
    };

    let embed = CreateEmbed::new()
        .title("Truth or Dare")
        .description(description)
        .timestamp(Timestamp::now());

    embed
}

/// Returns a `CreateButton` for Truths
pub fn truth_button() -> CreateButton {
    make_button("truth", "Truth", ButtonStyle::Primary)
}

/// Returns a `CreateButton` for Dares
pub fn dare_button() -> CreateButton {
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
