use std::{env, time::Duration, sync::atomic::AtomicBool};
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

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
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
    pub mod aioverlord;
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
    let discord_token = "token";
    let openai_token = "token".to_string();

    //openai
    set_key(openai_token);

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: "You are a large language model built into a command line interface as an example of what the `openai` Rust library made by Valentine Briese can do.".to_string(),
        name: None,
    }];

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
        .event_handler(Handler
        {
            is_loop_running: AtomicBool::new(false),
        })
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");


        

        if let Err(why) = client.start().await {
            println!("An error occurred while running the client: {:?}", why);
        }
    
}