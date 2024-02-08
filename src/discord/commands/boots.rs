use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption}};
use crate::prediction::predict;

pub fn register() -> CreateCommand {
    // TODO: how to set default value for option to user that triggers the command?
    CreateCommand::new("boots")
        .description("Suggests the correct boots for your ARAM match")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "player",
                "The player that needs help"
            )
            .required(false)
        )
}

pub fn run(options: &Vec<ResolvedOption>, calling_user: &User) -> String {
    let snowflake: String;

    // Get User from option or fall back to triggering User
    if let Some(
        ResolvedOption { value: ResolvedValue::User(user_option, _), ..}
    ) = options.first() {
        snowflake = user_option.id.to_string();
    } else {
        snowflake = calling_user.id.to_string();
    }

    if let Some(prediction) = predict(&snowflake) {
        return format!("You should definitely buy {}", prediction);
    }
    String::from("I could not make a prediction for you")
}