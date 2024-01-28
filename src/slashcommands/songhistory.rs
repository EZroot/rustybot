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
    
    // for user in users {
    //     println!("User: {}, Level: {}, XP: {}/{}", user.user_name, user.level, user.experience, xp_for_level(user.level+1));
    //     for command in &user.commands {
    //         println!("Command: {}, Used: {}", command.command_name, command.times_used);
    //         for (arg, count) in &command.command_args {
    //             println!("Arg: {}, Used: {}", arg, count);
    //         }
    //     }
    // }
    "ok".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("history").description("Top 10 most played")
}
