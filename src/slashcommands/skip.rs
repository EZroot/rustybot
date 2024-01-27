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

pub async fn run(ctx: &Context, options: &[CommandDataOption]) -> String {
    
    let mut final_message = "".to_string();
    let no_songs_available = "No songs available to skip to, idiot";
    let mut is_error = false;

    let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
       let mut handler = handler_lock.lock().await;
        if handler.queue().is_empty() || handler.queue().current_queue().len() == 1{
            is_error = true;
        }
        else 
        {
            let meta_data =  handler.queue().current_queue()[0].metadata().clone();
            let title = meta_data.title.unwrap();
            final_message = format!("Skipping {}", &title);

            handler.queue().skip();
                
        }

    } else {
        is_error = true;
    }

    if is_error{
        no_songs_available.to_string()
    }else {
        final_message
    }

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("skip").description("Skip current song")
}
