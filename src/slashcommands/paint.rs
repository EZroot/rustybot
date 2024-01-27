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

use crate::ai::diffuserai::{generate_stable_diffuse_image, ImageFilter};

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
    let prompt_option = options
        .get(0)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    let high_res_option = options
        .get(1)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    let negative_default = CommandDataOptionValue::String(String::from("duplicate, amputation, easynegative, negative_hand"));
    let negative_prompt_option = options
        .get(2)
        .and_then(|option| option.resolved.as_ref())
    .unwrap_or(&negative_default);


    let mut prompt_args_clone = String::new();
    if let CommandDataOptionValue::String(attachment) = prompt_option {
        prompt_args_clone = format!("{} {}", attachment.clone(), "(--ar 1:1)");
    }

    let mut negative_prompt_args_clone = String::new();
    if let CommandDataOptionValue::String(attachment) = negative_prompt_option {
        negative_prompt_args_clone = attachment.clone();
    }
    let mut high_res_args_clone = false;
    if let CommandDataOptionValue::Boolean(attachment) = high_res_option {
        high_res_args_clone = attachment.clone();
    }

    //generate_stable_diffuse_image(&args_clone, 904,904,50, 2, false).await.unwrap() //904x904 817k for realistic v2.0
    //generate_stable_diffuse_image(&args_clone, 944,944,50, 2, false).await.unwrap() // 944x944 for paragon
    //generate_stable_diffuse_image(&args_clone, 720,1280,50, 2, false).await.unwrap() // 944x944 891k for paragon
    generate_stable_diffuse_image(&prompt_args_clone,&negative_prompt_args_clone, 512, 512,50, 2, true, 500, ImageFilter::GaussainNoise,0.8, high_res_args_clone).await.unwrap() // 944x944 891k for paragon
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("paint")
        .description("2 images at 512x512. (fast)")
        .create_option(|option| {
            option
                .name("prompt")
                .description("What do you want to attempt?")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("extend-background")
                .description("Extends the background. Or regular upscale if not")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })        .create_option(|option| {
            option
                .name("negative-prompt")
                .description("Default: duplicate, amputation, easynegative, negative_hand")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
