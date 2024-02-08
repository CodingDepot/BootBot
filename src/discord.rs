use std::{fs::File, io::{BufRead, BufReader}};

use serenity::{all::{Command, GatewayIntents, Interaction, Ready}, async_trait, builder::{CreateInteractionResponse, CreateInteractionResponseMessage}, client::{Context, EventHandler}, Client};

mod visualization;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let options = &command.data.options();

            let content = match command.data.name.as_str() {
                "boots" => Some(commands::boots::run(options, &command.user)),
                _ => None
            };

            if let Some(content) = content {
                let response = CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(content).ephemeral(true)
                );
                if let Err(reason) = command.create_response(&ctx.http, response).await {
                    println!("Could not respond to command: {reason}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{:?} is connected", ready.user.name);
        
        let _command = Command::create_global_command(
            &ctx.http,
            commands::boots::register()
        )
        .await;

        // println!("Created global command: {command:#?}");
    }
}

#[tokio::main]
pub async fn main() {
    // Create the client
    let token = get_token("discord_key.txt");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Could not create Client");

    // Start a single shard
    if let Err(reason) = client.start().await {
        println!("Eclient error: {reason}");
    }
}

fn get_token(file_name: &str) -> String {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines().nth(0).unwrap().unwrap()
}