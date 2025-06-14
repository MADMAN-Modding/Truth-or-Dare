use rand::random_bool;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, Timestamp};

use crate::{
    bot::Bot,
    other_impl::{EmbedMaker, FooterMaker},
    questions::QuestionType,
};

use std::future::Future;
use std::pin::Pin;

pub async fn embed_text(
    bot: &Bot,
    question_type: QuestionType,
    rating_limit: impl AsRef<str>,
) -> CreateEmbed {
    let mut loops: u8 = 0;

    let (description, rating) = loop {
        // If the limit is PG, question rating is PG
        // If it is PG-13, it has a random chance of the rating
        let rating: &str = match rating_limit.as_ref() {
            "PG-13" => {
                if random_bool(0.5) {
                    "PG-13"
                } else {
                    "PG"
                }
            }
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
            break ("No Question Found".to_string(), "N/A".to_string());
        }
    };

    let embed = CreateEmbed::new()
        .title(if question_type.to_string() == "TRUTH" {
            "Truth"
        } else {
            "Dare"
        })
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

/// Returns a `CreateButton` for the "Get Question" action
pub fn previous_page_button() -> CreateButton {
    make_button("previous_page", "Previous Page", ButtonStyle::Secondary)
}

/// Returns a `CreateButton` for the "Get Question" action
pub fn next_page_button() -> CreateButton {
    make_button("next_page", "Next Page", ButtonStyle::Secondary)
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

/// Sends a page of questions as an embed
pub fn send_page<'a>(
    page_number: usize,
    bot: &'a Bot,
) -> Pin<Box<dyn Future<Output = CreateInteractionResponse> + Send + 'a>> {
    Box::pin(async move {
        let questions = &bot.questions.read().await;

        let pages = questions.len() / 10 + if questions.len() % 10 > 0 { 1 } else { 0 };
        let start = (page_number - 1) * 10;
        let end = start + 10;
        let questions = &questions[start..end.min(questions.len())];
        if page_number > pages || page_number < 1 {
            return send_page(1, bot).await;
        }
        if questions.is_empty() {
            return CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().add_embed("There are no questions available on this page."
                .to_embed("No Questions", "List of Questions")));
        }

        let buttons = CreateActionRow::Buttons(vec![
            previous_page_button(),
            next_page_button()]);

        // Format the questions for the response
        let questions: Vec<String> = questions
            .iter()
            .map(|q| format!("{} ({} - {})", q.prompt, q.question_type, q.rating))
            .collect();
        // Join the questions into a single string
        let response = format!("{}", questions.join("\n"));
        let title = format!("Page {}/{}", page_number, pages);
        // Create the embed
        let embed = response.to_embed(title, "List of Questions");

        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().add_embed(embed).components(vec![buttons]))

        
    })
}
