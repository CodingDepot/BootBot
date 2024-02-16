use std::{thread, env};

use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage}};
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

pub fn run(options: &Vec<ResolvedOption>, calling_user: &User) -> CreateInteractionResponse {
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
    let suggestion = thread::spawn(move || {
            predict(&snowflake)
        }
    ).join();

    let mut response_message: CreateInteractionResponseMessage;
    match suggestion {
        Ok(Ok(boots)) => {
            let content = format!("You should definitely buy {}!", boots);

            response_message = 
                CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true);
    
            let id = crate::constants::BOOT_IDS.iter()
                .filter(|tuple| tuple.1 == boots)
                .map(|tuple| tuple.0)
                .nth(0);
    
            if let Some(boot_id) = id {
                let game_version = env::var("GAME_VERSION").expect("Could not fetch the game version");
    
                response_message = response_message.add_embed(
                    CreateEmbed::new() 
                        .image(format!("https://ddragon.leagueoflegends.com/cdn/{}/img/item/{}.png", game_version, boot_id as u32))
                );
            }
        },
        Ok(Err(error)) => {
            response_message = 
            CreateInteractionResponseMessage::new()
                .content(format!("{}", error.message))
                .ephemeral(true);
        },
        _ => {
            response_message = 
            CreateInteractionResponseMessage::new()
                .content(format!("Error: Thread failed"))
                .ephemeral(true);
        }
    }

    CreateInteractionResponse::Message(response_message)
}