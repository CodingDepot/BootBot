use std::{thread, env};

use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage}};

use crate::prediction::recreate_model;

pub fn register() -> CreateCommand {
    CreateCommand::new("model")
        .description("Recreates the model")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "game_count",
                "How many games the model uses as training data. 1000 games = 21 minutes"
            )
            .min_int_value(1)
            .max_int_value(100_000)
        )
}

pub fn run(options: &Vec<ResolvedOption>, calling_user: &User) -> CreateInteractionResponse {
    let game_count: usize;
    let content: String;
    let vip_snowflake: &str = &env::var("VIP_USER")
        .expect("Could not fetch the VIP User");

    if let Some(
        ResolvedOption { value: ResolvedValue::Integer(games), ..}
    ) = options.first() {
        game_count = *games as usize;

        if calling_user.id.to_string() != vip_snowflake {
            content = String::from("You do not have permission to do this.");
        } else {
            thread::spawn(move || {
                    recreate_model(game_count);
                }
            );
            content = format!("Successfully started training a new model from {game_count} games");
        }
    } else {
        content = String::from("Missing the number of games.")
    }

    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(content)
            .ephemeral(true)
    )
}