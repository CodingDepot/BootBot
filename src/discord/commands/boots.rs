use std::thread;

use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption}};
use crate::prediction::predict;

pub fn register() -> CreateCommand {
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

    // TODO: try to get user connection to get Riot id
    // Get User from option or fall back to triggering User
    if let Some(
        ResolvedOption { value: ResolvedValue::User(user_option, _), ..}
    ) = options.first() {
        snowflake = user_option.id.to_string();
    } else {
        snowflake = calling_user.id.to_string();
    }

    // A new thread is necessary here:
    // https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
    thread::spawn(move || {
            if let Some(prediction) = predict(&snowflake) {
                return format!("You should definitely buy {}!", prediction);
            }
            String::from("I could not make a prediction for you...")
        }
    ).join().unwrap()
}