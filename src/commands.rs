use std::str::FromStr;

use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, GuildId, Permissions,
};
use uuid::Uuid;

use crate::{
    bot::Bot, embed::send_page, interactions::truth_or_dare, menu_type::MenuType, other_impl::MessageMaker, questions::QuestionType
};

/// Creates a vector of commands for the bot
pub fn create_commands() -> Vec<CreateCommand> {
    vec![
        set_rating_command(),
        add_question_command(),
        remove_question_command(),
        list_questions_command(),
        list_custom_questions_command(),
        set_question_permissions_command(),
        truth_command(),
        dare_command()
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
            .add_string_choice("PG-13", "PG-13")
            .add_string_choice("PG & PG-13", "ALL"),
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

        if let Err(err) = bot.set_guild_rating(guild_id_i64, rating).await {
            if command.user.id == 741999030623535168 {
                println!("{}", err);
            }
            
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
    // Check if the user is an admin in the guild using the permissions field on the command
    let is_admin = command
        .member
        .as_ref()
        .and_then(|member| member.permissions)
        .map(|perms| perms.administrator())
        .unwrap_or(false);

    if is_admin || !bot.get_guild_question_permissions(command.guild_id).await {
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
            let uid = Uuid::new_v4().to_string();

            sqlx::query(
                        r#"INSERT INTO questions (prompt, question_type, rating, guild_id, uid) VALUES (?1, ?2, ?3, ?4, ?5)"#,
                    )
                    .bind(&question)
                    .bind(question_type.to_string())
                    .bind(rating)
                    .bind(command.guild_id.unwrap().get() as i64)
                    .bind(uid)
                    .execute(&bot.database)
                    .await
                    .ok();

            format!("Question added: {}", question).to_interaction_message()
        }
    } else {
        "You must be an admin to run this command".to_interaction_message()
    }
}

fn remove_question_command() -> CreateCommand {
    CreateCommand::new("remove_question")
        .description("Remove a question from the database")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "question_uid",
                "The question to be removed",
            )
            .required(true),
        )
}

pub async fn remove_question(bot: &Bot, command: &CommandInteraction) -> CreateInteractionResponse {
    let guild_id = command.guild_id;

    let question_uid = command
        .data
        .options
        .iter()
        .find(|o| o.name == "question_uid")
        .and_then(|o| o.value.as_str())
        .unwrap()
        .to_string();

    let query = r#"DELETE FROM questions WHERE guild_id = ?1 AND uid = ?2"#;

    match bot.check_question_guild(guild_id, &question_uid).await {
        true => {
            match sqlx::query(query)
                .bind(guild_id.unwrap().get() as i64)
                .bind(&question_uid)
                .execute(&bot.database)
                .await
            {
                Ok(_) => format!("Question with uid: {} has been removed.", question_uid)
                    .to_interaction_message(),
                Err(e) => e.to_string().to_interaction_message(),
            }
        }
        false => "You can't remove a question outside of your server!".to_interaction_message(),
    }
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

fn set_question_permissions_command() -> CreateCommand {
    CreateCommand::new("set_question_permissions")
        .description("Set if only admins should be able to add questions")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "admin",
                "Only allow admins to add questions",
            )
            .required(true)
            .add_string_choice("Yes", "true")
            .add_string_choice("No", "false"),
        )
}

pub async fn set_question_permissions(
    bot: &Bot,
    command: &CommandInteraction,
) -> CreateInteractionResponse {
    let guild_id = command.guild_id;

    let admin = command
        .data
        .options
        .iter()
        .find(|c| c.name == "admin")
        .unwrap()
        .value
        .as_str()
        .unwrap()
        == "true";

    match bot.set_guild_question_permissions(guild_id, admin).await {
        Ok(_) => format!("Admin only set to {admin}").to_interaction_message(),
        Err(_) => "Error setting permissions".to_interaction_message(),
    }
}

fn truth_command() -> CreateCommand {
    CreateCommand::new("truth")
        .description("Sends a truth question")
}

pub async fn truth(bot: &Bot, command: &CommandInteraction) -> CreateInteractionResponse {
    truth_or_dare(bot, "truth", command.guild_id).await
}

fn dare_command() -> CreateCommand {
    CreateCommand::new("dare")
        .description("Sends a dare question")
}

pub async fn dare(bot: &Bot, command: &CommandInteraction) -> CreateInteractionResponse {
    truth_or_dare(bot, "dare", command.guild_id).await
}