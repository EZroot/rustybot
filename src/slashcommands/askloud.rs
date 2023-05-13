use serenity::builder::CreateApplicationCommand;
use serenity::framework::standard::{Args, Command};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::{Attachment, ChannelId, GuildId, UserId};
use serenity::prelude::Context;
use songbird::input::Restartable;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{
    env,
    io::{stdin, stdout, Write},
};

use crate::ai::openai::{generate_openai_response, generate_openai_response_include_username};
use crate::ai::voicesynth::synthesize_text;

lazy_static! {
    static ref YTDL_MUTEX: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

struct CommandContext {
    authorid: UserId,
    channelid: ChannelId,
}

impl CommandContext {
    fn new() -> Self {
        Self {
            authorid: UserId::from(0),
            channelid: ChannelId::from(0),
        }
    }
}

pub async fn run(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    let option = options
        .get(0)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    let mut args_clone = String::new();
    if let CommandDataOptionValue::String(attachment) = option {
        args_clone = attachment.clone();
    }

    let user_request = args_clone;
    let user_name_and_question = format!("<@{}>: {}", &command.user.id, &user_request);
    println!("{}", &user_name_and_question);

    // let trimmed_string = generate_openai_response(user_request, command.user.id).await;
    let cmd_clone = command.user.clone();
    let trimmed_string =
        generate_openai_response_include_username(cmd_clone.name, user_request, cmd_clone.id).await;
    synthesize_text(&trimmed_string, "output.mp3")
        .await
        .unwrap();

    let path = Path::new("authserver/output.mp3");
    println!("tryin to speak: {}", path.to_str().unwrap());

    let guild_id = GuildId::from(308708637679812608); //self.guild_id.unwrap();
    let cache = &ctx.cache;
    let guild = cache.guild(guild_id).unwrap();
    let channel_id: Option<ChannelId> = guild
        .voice_states
        .get(&command.user.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => return "Not in voice channel".to_string(),
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let path = Path::new("authserver/output.mp3");
        println!("tryin to speak: {}", path.to_str().unwrap());

        let source = tokio::task::spawn_blocking(move || {
            let _lock = YTDL_MUTEX.lock().unwrap();
            Restartable::ffmpeg(path, false) //if any issues, check this lazy instantiation
        })
        .await
        .unwrap();

        let true_source = source.await.unwrap();

        let song = handler.play_source(true_source.into());

        return format!("{}\n\n{}", user_name_and_question, trimmed_string);
    } else {
        let derp = " <- Failed to play".to_string();
        return derp;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("askloud")
        .description("Speak with Mod Ash... with voice!")
        .create_option(|option| {
            option
                .name("question")
                .description("your dumb question")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
