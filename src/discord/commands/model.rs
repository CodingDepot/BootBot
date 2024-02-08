use std::{fs::File, io::{BufRead, BufReader}, thread};

use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption}};

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

pub fn run(options: &Vec<ResolvedOption>, calling_user: &User) -> String {
    let game_count: usize;
    if let Some(
        ResolvedOption { value: ResolvedValue::Integer(games), ..}
    ) = options.first() {
        game_count = *games as usize;
    } else {
        return String::from("Missing the number of games.")
    }

    // TODO: environment variables
    // Get User from option or fall back to triggering User
    if calling_user.id.to_string() != get_vip_snowflake("vip.txt") {
        return String::from("You do not have permission to do this.")
    }

    thread::spawn(move || {
            recreate_model(game_count);
        }
    ).join().unwrap();
    String::from(format!("Successfully started training a new model from {game_count} games"))
}

fn get_vip_snowflake(file_name: &str) -> String {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines().nth(1).unwrap().unwrap()
}