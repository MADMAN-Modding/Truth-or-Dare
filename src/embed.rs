use rand::random_bool;
use serenity::all::{ButtonStyle, CreateButton, CreateEmbed, Timestamp};

use crate::{bot::Bot, questions::QuestionType, other_impl::FooterMaker};

pub async fn embed_text(bot: &Bot, question_type: QuestionType, rating_limit: impl AsRef<str>) -> CreateEmbed {
    let mut loops: u8 = 0;

    let (description, rating) = loop {
        // If the limit is PG, question rating is PG
        // If it is PG-13, it has a random chance of the rating
        let rating : &str = match rating_limit.as_ref() {
            "PG-13" => {
                if random_bool(0.5) {
                    "PG-13"
                } else {
                    "PG"
                }

            },
            _ => "PG",  
        };

        let question = bot.get_random_question(question_type, rating).await;

        if question.is_some() {
            let question = question.unwrap();

            break (question.prompt, question.rating);
        }

        loops += 1;


        // This really should never happen, but if it does, this protects against infinite loops
        if loops == 5 {
            break ("No Question Found".to_string(), "N/A".to_string())
        }
    };

    let embed = CreateEmbed::new()
        .title(if question_type.to_string() == "TRUTH" {"Truth"} else {"Dare"})
        .description(description)
        .footer(rating.to_footer())
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
