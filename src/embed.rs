use serenity::all::{ButtonStyle, CreateButton, CreateEmbed, Timestamp};

use crate::questions::QuestionType;

pub async fn embed_text(question_type: QuestionType) -> CreateEmbed {
    let description = match question_type {
        QuestionType::TRUTH => "truth",
        QuestionType::DARE => "dare",
        QuestionType::NONE => "Error",
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
