use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, Permissions};

//// Creates a vector of commands for the bot
pub fn create_commands() -> Vec<CreateCommand> {
    vec![
        set_rating(),
        add_question_command(),
        remove_question_command(),
        list_questions_command(),
    ]
}

/// Command to set the rating limit for questions
fn set_rating() -> CreateCommand {
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
        .description("List all questions in the database")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "rating",
                "The rating of the questions to list (PG or PG-13)",
            )
            .required(false)
            .add_string_choice("PG", "PG")
            .add_string_choice("PG-13", "PG-13"),
        )
}
