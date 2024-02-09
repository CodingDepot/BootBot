use std::{fs::File, io::{BufRead, BufReader}, thread};

use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage}};

use crate::prediction::recreate_model;

pub fn register() -> CreateCommand {
    CreateCommand::new("model")
        .description("Recreates the model")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "game count",
                "How many games the model uses as training data. 1000 games = 21 minutes"
            )
            .min_int_value(1)
            .max_int_value(100_000)
        )
}

pub fn run(options: &Vec<ResolvedOption>, calling_user: &User) -> CreateInteractionResponse {
    let game_count: usize;
    let content: String;

    if let Some(
        ResolvedOption { value: ResolvedValue::Integer(games), ..}
    ) = options.first() {
        game_count = *games as usize;

        // TODO: environment variables
        // Get User from option or fall back to triggering User
        if calling_user.id.to_string() != get_vip_snowflake("vip.txt") {
            content = String::from("You do not have permission to do this.");
        } else {
            thread::spawn(move || {
                    recreate_model(game_count);
                }
            ).join().unwrap();
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

fn get_vip_snowflake(file_name: &str) -> String {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines().nth(1).unwrap().unwrap()
}