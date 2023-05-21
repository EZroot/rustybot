use handler::HttpHandler;
use serde::Deserialize;
use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::StandardFramework,
    http::Http,
    model::{gateway::GatewayIntents, prelude::ChannelId},
    prelude::TypeMapKey,
    utils::MessageBuilder,
};
use songbird::SerenityInit;
use tokio::time::sleep;
use std::{
    env,
    sync::{atomic::AtomicBool, Arc},
    time::Duration, io::{self, Write},
};

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};

use reqwest::Client as ReqwestClient;

use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};
use songbird::{input::Restartable, tracks::TrackQueue};

extern crate warp;
use warp::{Filter, Reply};

#[derive(Deserialize)]
struct RecievedCommandReq {
    recieved_command: String,
}

mod audioripper;
mod commands;
mod ffmpeg_utils;
mod handler;
mod messages;

mod ai {
    pub mod diffuserai;
    pub mod openai;
    pub mod voicesynth;
    pub mod wolfy;
}

mod voiceactivatedcommands{
    pub mod discordcommands{
        pub mod ask;
        pub mod paint;
        pub mod search;
    }
}

mod slashcommands {
    pub mod ask;
    pub mod askloud;
    pub mod knowledge;
    pub mod paint;
    pub mod paintdetailed;
    pub mod play;
    pub mod queue;
    pub mod search;
    pub mod speak;
}

use crate::{commands::GENERAL_GROUP, handler::GLOBAL_DATA};
use crate::handler::EventHandlers;
use crate::handler::HttpRequestHandler;
use crate::handler::SlashCommandHandler;
use crate::handler::update_global_data;
use crate::handler::GlobalData;

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

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_INTEGRATIONS
        | GatewayIntents::GUILD_MESSAGES;

    let slash_handler = SlashCommandHandler {
        is_loop_running: AtomicBool::new(false),
    };

    let http_channel_id = ChannelId(1234567890); // Replace with the actual channel ID

    let http_req_handler = HttpRequestHandler {
        http: Arc::new(ReqwestClient::new()),
        channel_id: http_channel_id,
    };

    let combined_handlers = EventHandlers {
        slash_command_handler: slash_handler,
        http_request_handler: http_req_handler,
    };

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(combined_handlers)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<HttpHandler>(http_channel_id);
    }

    let request_command_handle = tokio::spawn(async move {
        let route = warp::path("derp")
        .and(warp::get())
        .and(warp::query::<RecievedCommandReq>())
        .map(|data: RecievedCommandReq| {

            let ccc = data.recieved_command.clone();
            println!("Received string: {}", ccc);
            tokio::spawn(async move {
                // Asynchronous command to execute when the route is called
                // Example: Simulating an asynchronous task that takes some time
                // Execute your asynchronous logic here
                println!("Executing asynchronous command");
                update_global_data(data.recieved_command).await.unwrap();
            });

            ccc
        });

        warp::serve(route).run(([192, 168, 0, 4], 3030)).await;
    });

    let client_handle = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("An error occurred while running the client: {:?}", why);
        }
    });

    tokio::try_join!(client_handle, request_command_handle).unwrap();

    println!("Everything working good");
}
