use rand::random_bool;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Timestamp};

use crate::{
    bot::Bot, menu_type::{MenuToStr, MenuType}, other_impl::{EmbedMaker, FooterMaker, MessageMaker}, questions::{Question, QuestionTraits, QuestionType}
};

use std::future::Future;
use std::pin::Pin;

/// Makes the embed that is sent to the front end for questions
/// 
/// # Parameters
/// * `bot: &Bot` - Bot instance for database interaction
/// * 'question_type: QuestionType' - The type of question being asked
/// * `rating_limit: impl AsRef<str>` - What rating the question must be
/// * 'guild_id: Option<GuildId>' - GuildId to use when looking at the database
pub async fn embed_text(
    bot: &Bot,
    question_type: QuestionType,
    rating_limit: impl AsRef<str>,
    guild_id: Option<GuildId>
) -> CreateEmbed {
    // Tracks how many times it has tried to find a question
    let mut loops: u8 = 0;

    // Gets the description, rating, and uid of the question
    let question: Question = loop {
        // Sets the rating, will be the set rating unless it is ALL
        // If it is ALL, it has a random change of PG or PG-13
        let rating: &str = match rating_limit.as_ref() {
            "ALL" => {
                if random_bool(0.5) {
                    "PG-13"
                } else {
                    "PG"
                }
            },
            "PG-13" => "PG-13",
            _ => "PG",
        };

        // Gets a random question from the database
        let question = bot.get_random_question(question_type, rating, guild_id).await;

        // Unwrap the question and return the unwrap if it is Ok
        if question.is_ok() {
            let question = question.unwrap().unwrap();

            break question;
        }

        // Increments how many loops have been completed
        loops += 1;

        // This really should never happen, but if it does, this protects against infinite loops
        if loops == 5 {
            break Question::new(-1, "N/A".to_string(), QuestionType::NONE, "PG-13".to_string(), "0".to_string());
        }
    };

    // Creates the embed to send to the client
    let embed = CreateEmbed::new()
        .title(if question_type == QuestionType::TRUTH {
            "Truth"
        } else {
            "Dare"
        })
        .description(question.prompt)
        .footer(format!("Rating: {} | UID: {}", question.rating, question.uid).to_footer())
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
pub fn previous_page_button(page_number :usize, menu_type: &str) -> CreateButton {
    make_button(format!("previous_page-{page_number}:{menu_type}"), "Previous Page", ButtonStyle::Secondary)
}

/// Returns a `CreateButton` for the "Get Question" action
pub fn next_page_button(page_number :usize, menu_type: &str) -> CreateButton {
    make_button(format!("next_page-{page_number}:{menu_type}").as_str(), "Next Page", ButtonStyle::Secondary)
}

/// Makes a button based on provided input
///
/// # Parameters
/// * `id: &str` - ID of the button
/// * `label: &str` - Text to be displayed on the button
/// * `style: ButtonStyle` - Style of the button
fn make_button(id: impl AsRef<str>, label: &str, style: ButtonStyle) -> CreateButton {
    CreateButton::new(id.as_ref()).label(label).style(style)
}

/// Sends a page of questions as an embed
pub fn send_page(
    page_number: usize,
    questions: Vec<Question>,
    menu_type: MenuType
) -> Pin<Box<dyn Future<Output = CreateInteractionResponse> + Send>> {
    Box::pin(async move {
        
        if questions.is_empty() {
            return "No questions found...".to_interaction_message();
        }

        let pages = questions.len() / 10 + if questions.len() % 10 > 0 { 1 } else { 0 };
        // If the requested page is 0, send 0, otherwise, send page_number - 1
        let start = (if page_number == 0 {0} else {page_number - 1}) * 10;
        let end = start + 10;
        
        if page_number > pages {
            return send_page(1, questions, menu_type).await;
        } else if  page_number < 1 {
            return send_page(pages, questions, menu_type).await;
        }

        // Questions to be sent to the quested page
        let page_questions = &questions[start..end.min(questions.len())];

        let buttons = CreateActionRow::Buttons(vec![
            previous_page_button(page_number-1, &menu_type.to_str()),
            next_page_button(page_number, &menu_type.to_str())]);

        // Format the questions for the response
        let questions: Vec<String> = page_questions
            .iter()
            .map(|question| {
                // This prevents the uid of default questions from being sent to the user
                let uid =  match menu_type {
                    MenuType::CUSTOM => format!(" UID: {}", question.uid),
                    MenuType::DEFAULT => "".to_string()
                };

                format!("{} ({} - {}){}", question.prompt, question.question_type, question.rating, uid)
            
            })
            .collect();
        // Join the questions into a single string
        let response = format!("{}", questions.join("\n"));
        let title = format!("Page {}/{}", page_number, pages);
        // Create the embed
        let embed = response.to_embed(title, "List of Questions");

        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().add_embed(embed).components(vec![buttons]))

        
    })
}
