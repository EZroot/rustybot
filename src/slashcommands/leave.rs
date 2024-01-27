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
    
    let leave_msg = "Left voice channel";
    let leave_msg_failed = "Not in a voice channel dumbass";
    let mut is_error = false;

    let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
       
        if let Err(e) = manager.remove(guild_id).await {
            is_error = true;
        }
        

    } else {
        is_error = true;
    }

    if is_error{
        leave_msg_failed.to_string()
    }else {
        leave_msg.to_string()
    }

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("Leave voice channel")
}
