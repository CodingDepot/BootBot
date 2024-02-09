use std::env;

use serenity::{all::{Command, GatewayIntents, GuildId, Interaction, Ready}, async_trait, builder::CreateInteractionResponse, client::{Context, EventHandler}, Client};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let options = &command.data.options();

            let response = match command.data.name.as_str() {
                "boots" => commands::boots::run(options, &command.user),
                "model" => commands::model::run(options, &command.user),
                _ => CreateInteractionResponse::Pong,
            };

            if let Err(reason) = command.create_response(&ctx.http, response).await {
                println!("Could not respond to command: {reason}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{:?} is connected", ready.user.name);
        
        let guild_id = GuildId::new(
            env::var("VIP_GUILD")
                .expect("Could not fetch the VIP guild")
                .parse()
                .expect("Could not parse the VIP guild")
        );
        
        let _guild_command = guild_id.create_command(
            &ctx.http,
            commands::model::register()
        )
        .await;

        let _global_command = Command::create_global_command(
            &ctx.http,
            commands::boots::register()
        )
        .await;

        // println!("Created guild command for {guild_id}: {_guild_command:#?}");
        // println!("Created global command: {_global_command:#?}");
    }
}

#[tokio::main]
pub async fn main() {
    // Create the client
    let token: &str = &env::var("DISCORD_TOKEN")
        .expect("Could not fetch the Discord token");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Could not create Client");

    // Start a single shard
    if let Err(reason) = client.start().await {
        println!("Eclient error: {reason}");
    }
}