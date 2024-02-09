use std::env;
use reqwest::blocking;

use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage}};

pub fn register() -> CreateCommand {
    CreateCommand::new("version")
        .description("Changes the version number for League of Legends download links")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "version_number",
                "What the version number should be. Change with caution!"
            )
        )
}

pub fn run(options: &Vec<ResolvedOption>, calling_user: &User) -> CreateInteractionResponse {
    let content: String;
    let vip_snowflake: &str = &env::var("VIP_USER")
        .expect("Could not fetch the VIP User");

    if let Some(
        ResolvedOption { value: ResolvedValue::String(version), ..}
    ) = options.first() {
        if calling_user.id.to_string() != vip_snowflake {
            content = String::from("You do not have permission to do this.");
        } else {
            // Careful! Could cause problems when being read at the same time!
            // https://doc.rust-lang.org/std/env/fn.set_var.html
            if check_version(version.to_string()) {
                env::set_var("GAME_VERSION", version);
                content = format!("Successfully changed the game version to {}", version);
            } else {
                content = format!("Version {} does not yield a successful download link.\nTry https://ddragon.leagueoflegends.com/cdn/{}/img/item/1001.png manually!", version, version);
            }
        }
    } else {
        content = String::from("Missing the version number.")
    }

    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(content)
            .ephemeral(true)
    )
}

fn check_version(version: String) -> bool {
    let client = blocking::Client::new();
    let uri = format!("https://ddragon.leagueoflegends.com/cdn/{}/img/item/1001.png", version);

    let response = client
        .get(&uri)
        .send();

    if let Ok(res) = response {
        return res.status().is_success()
    }
    false
}