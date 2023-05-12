use std::{env, time::Duration};
use songbird::SerenityInit;
use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        StandardFramework,
        standard::macros::{group},
    },
    model::gateway::GatewayIntents,
    prelude::TypeMapKey, http::Http,
};

mod commands;
mod handler;
mod audioripper;
mod messages;
mod ffmpeg_utils;

mod slashcommands
{
    pub mod ping;
    pub mod queue;
    pub mod play;
    pub mod search;
}

use crate::commands::GENERAL_GROUP;
use crate::handler::Handler;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = songbird::Songbird;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // Configure the client with your Discord bot token in the environment.
    let token = "Mzg3MjY5Nzc1MjY2NDE0NTky.GcbzbI.1SXwH3SuFuZWWyc_1jgFk8IWFpN77J8Pf4pg60";

    let framework = StandardFramework::new()
        .configure(|c| c
                   .prefix("/"))
        .group(&GENERAL_GROUP);


    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES;


        
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    

        if let Err(why) = client.start().await {
            println!("An error occurred while running the client: {:?}", why);
        }
    
}