use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::{GuildId, Attachment, UserId, ChannelId, Message};
use serenity::framework::standard::{Args, Command, CommandResult};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue, ApplicationCommandInteraction,
};
use serenity::prelude::Context;
use songbird::input::Restartable;

use std::path::Path;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::ai::voicesynth::synthesize_text;
use crate::messages::check_msg;

lazy_static! {
    static ref YTDL_SEARCH_MUTEX: Mutex<()> = Mutex::new(());
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

pub async fn search(context: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&context.http, "Question? Sorry I didn't quite catch that.").await);
            return Ok(())
        },
    };

    let text = format!("I found your song {}",url);

    let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
    let voice_channel_id = ChannelId::from(498136555102273537);


    let manager = songbird::get(context).await
    .expect("Songbird Voice client placed in at initialisation.");

    manager.join(guild_id, voice_channel_id).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        println!("searching: {}", &url);

        
        let source = 
            Restartable::ytdl_search(url, true).await.unwrap();


        let song = handler.enqueue_source(source.into());
        let song_title = song.metadata().title.as_ref().unwrap();

        let form =  format!("Found {}! Added to queue.",song_title);
        println!("{}",form);
        
    }
    else{

     let derp = url.clone() + " <- Failed to play";
     println!("{}",derp);
    }
    Ok(())
}
