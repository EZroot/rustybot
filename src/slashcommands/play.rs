use colored::Colorize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::{GuildId, Attachment, UserId, ChannelId};
use serenity::framework::standard::{Args, Command};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue, ApplicationCommandInteraction,
};
use serenity::prelude::Context;
use songbird::input::Restartable;

use std::sync::{Mutex, Arc};

use lazy_static::lazy_static;

use crate::youtubestats::{add_or_update_song, load_or_initialize_songs, save_songs};

lazy_static! {
    static ref YTDL_MUTEX: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

struct CommandContext 
{
    authorid : UserId,
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

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String {

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

    //join(&ctx_clone, &msg_clone, args_clone).await?;

    let url = args_clone;
    if !url.starts_with("http") {
        return "Must provide a valid URL".to_string();
    }

        let command_context = CommandContext {
            authorid: command.user.id,
            channelid: command.channel_id,
        };

    let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
    let cache = &ctx.cache;
    let guild = cache.guild(guild_id).unwrap();
    let channel_id = guild
    .voice_states.get(&command_context.authorid)
    .and_then(|voice_state| voice_state.channel_id);

let connect_to = match channel_id {
    Some(channel) => channel,
    None => {
        return "Not in voice channel".to_string()
    }
};

    let manager = songbird::get(ctx).await
    .expect("Songbird Voice client placed in at initialisation.").clone();

    manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        println!("Url {}", &url.bright_magenta());
        let url_clone: String = url.clone();
        let source = tokio::task::spawn_blocking(move || {
            let _lock = YTDL_MUTEX.lock().unwrap();
            Restartable::ytdl(url, true) //if any issues, check this lazy instantiation
        }).await.unwrap();

        let true_source = source.await.unwrap();

        let song = handler.enqueue_source(true_source.into());
        let song_title = song.metadata().title.as_ref().unwrap();
        
        let mut song_list = load_or_initialize_songs("youtube_history.json").unwrap();
        add_or_update_song(&mut song_list, &song_title, &url_clone);
        save_songs("youtube_history.json", &song_list).unwrap();

        return format!("Added {} to queue",&song_title)
    }
    else{

     let derp = url.clone() + " <- Failed to play";
    derp
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play").description("Rip shit off youtube >:)").create_option(|option| {
        option
        .name("url")
        .description("youtube url")
        .kind(CommandOptionType::String)
        .required(true)
    })
}
