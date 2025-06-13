use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, Permissions};

pub fn create_commands() -> Vec<CreateCommand> {
    vec![set_rating()]
}

fn set_rating() -> CreateCommand {
    CreateCommand::new("set_rating")
        .description("Set the question rating limit (PG or PG-13)")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "rating",
                "The max rating to allow",
            )
            .required(true)
            .add_string_choice("PG", "PG")
            .add_string_choice("PG-13", "PG-13"),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
