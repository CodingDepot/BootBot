use std::thread;

use serenity::{all::{CommandOptionType, ResolvedOption, ResolvedValue, User}, builder::{CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage}};
use crate::prediction::predict;
use crate::constants;

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
            if let Some(prediction) = predict(&snowflake) {
                return Some(prediction);
            }
            None
        }
    ).join();

    let mut response_message: CreateInteractionResponseMessage;
    if let Ok(Some(boots)) = suggestion {
        let content = format!("You should definitely buy {}!", boots);

        response_message = 
            CreateInteractionResponseMessage::new()
                .content(content)
                .ephemeral(true);

        let id = constants::BOOT_IDS.iter()
            .filter(|tuple| tuple.1 == boots)
            .map(|tuple| tuple.0)
            .nth(0);

        // TODO: create constants file with boot mapping and game version
        if let Some(boot_id) = id {
            response_message = response_message.add_embed(
                CreateEmbed::new() 
                    .image(format!("https://ddragon.leagueoflegends.com/cdn/{}/img/item/{}.png", constants::GAME_VERSION, boot_id as u32))
            );
        }
    } else {
        response_message = 
            CreateInteractionResponseMessage::new()
                .content("I could not make a prediction for you...")
                .ephemeral(true);
    }

    CreateInteractionResponse::Message(response_message)
}