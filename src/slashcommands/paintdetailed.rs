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

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::ai::diffuserai::generate_stable_diffuse_image;

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

    generate_stable_diffuse_image(&args_clone, 512,768,200,7.5, 4, false).await.unwrap()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("paintdetailed")
        .description("Paint some detailed shit. Takes 6-7 minutes roughly.")
        .create_option(|option| {
            option
                .name("prompt")
                .description("what do you want it to attempt?")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
