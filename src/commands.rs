use std::str::FromStr;

use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, GuildId, Permissions,
};

use crate::{bot::Bot, embed::send_page, menu_type::MenuType, other_impl::MessageMaker, questions::QuestionType};

//// Creates a vector of commands for the bot
pub fn create_commands() -> Vec<CreateCommand> {
    vec![
        set_rating_command(),
        add_question_command(),
        remove_question_command(),
        list_questions_command(),
        list_custom_questions_command(),
    ]
}

/// Command to set the rating limit for questions
fn set_rating_command() -> CreateCommand {
    CreateCommand::new("set_rating")
        .description("Set the question rating limit (PG or PG-13)")
        .add_option(
            // Option with a PG and PG-13 choice
            CreateCommandOption::new(
                CommandOptionType::String,
                "rating",
                "The max rating to allow",
            )
            .required(true)
            .add_string_choice("PG", "PG")
            .add_string_choice("PG-13", "PG-13"),
        )
        // Only allow users with the Administrator permission to use this command
        .default_member_permissions(Permissions::ADMINISTRATOR)
}

pub async fn set_rating(bot: &Bot, command: &CommandInteraction) -> CreateInteractionResponse {
    // Get the rating from the command
    let rating = command
        .data
        .options
        .iter()
        .find(|option| option.name == "rating")
        .and_then(|option| option.value.as_str())
        .unwrap_or("PG");

    if let Some(guild_id) = command.guild_id {
        let guild_id_i64 = guild_id.get() as i64;

        if let Err(_) = bot.set_guild_rating(guild_id_i64, rating).await {
            return "Failed to set rating.".to_interaction_message();
        }
    }
     
    format!("Rating set to {}.", rating).to_interaction_message()    
}

/// Command to add a question to the database
fn add_question_command() -> CreateCommand {
    CreateCommand::new("add_question")
        .description("Add a question to the database")
        // Prompt to ask
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "question", "The question to add")
                .required(true),
        )
        // Question type (TRUTH or DARE)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "question_type",
                "The type of question (TRUTH or DARE)",
            )
            .required(true)
            .add_string_choice("Truth", "TRUTH")
            .add_string_choice("Dare", "DARE"),
        )
        // Rating of the question (PG or PG-13)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "rating",
                "The rating of the question (PG or PG-13)",
            )
            .required(true)
            .add_string_choice("PG", "PG")
            .add_string_choice("PG-13", "PG-13"),
        )
}

pub async fn add_question(bot: &Bot, command: &CommandInteraction) -> CreateInteractionResponse {
    let get_option = |name| {
        command
            .data
            .options
            .iter()
            .find(|o| o.name == name)
            .and_then(|o| o.value.as_str())
    };
    // Sanitize input to remove potentially dangerous characters
    let question = get_option("question").unwrap_or("");

    let question_type = get_option("question_type")
        .and_then(|s| QuestionType::from_str(s.to_uppercase().as_str()).ok())
        .unwrap_or(QuestionType::NONE);
    let rating = get_option("rating").unwrap_or("PG");

    if question.is_empty() {
        "Question cannot be empty.".to_interaction_message()
    } else {
        sqlx::query(
                    r#"INSERT INTO questions (prompt, question_type, rating, guild_id) VALUES (?1, ?2, ?3, ?4)"#,
                )
                .bind(&question)
                .bind(question_type.to_string())
                .bind(rating)
                .bind(command.guild_id.unwrap().get() as i64)
                .execute(&bot.database)
                .await
                .ok();

        format!("Question added: {}", question).to_interaction_message()
    }
}

fn remove_question_command() -> CreateCommand {
    CreateCommand::new("remove_question")
        .description("Remove a question from the database")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "question",
                "The question to remove",
            )
            .required(true),
        )
}

fn list_questions_command() -> CreateCommand {
    CreateCommand::new("list_questions")
        .description("List all default questions and questions added by users in this server")
}

pub async fn list_questions(bot: &Bot, guild_id: Option<GuildId>) -> CreateInteractionResponse {
    let questions = bot.get_questions(guild_id).await;

    // Send the response
    send_page(1, questions, MenuType::DEFAULT).await
}

fn list_custom_questions_command() -> CreateCommand {
    CreateCommand::new("list_custom_questions")
        .description("List all questions added by users in this server")
}

pub async fn list_custom_questions(
    bot: &Bot,
    guild_id: Option<GuildId>,
) -> CreateInteractionResponse {
    let questions = bot.get_custom_questions(guild_id).await;

    send_page(1, questions, MenuType::CUSTOM).await
}
