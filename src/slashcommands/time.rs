use chrono::{DateTime, FixedOffset, Local, Utc};
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
use std::fmt::Write;
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
    let mut time_zones = String::new();
    let now_utc: DateTime<Utc> = Utc::now();

    // Newfoundland time
    let local_time: DateTime<Local> = Local::now();
    let time = format!("{}", local_time.format("%I:%M:%S %p"));
    write!(time_zones, "### :flag_ca: NL \t|\t {}\n", time).unwrap();
    
    // Timezone offset for Toronto (Eastern Time), UTC-5
    let toronto_offset = FixedOffset::west(5 * 3600);
    let toronto_time = now_utc.with_timezone(&toronto_offset);
    let toronto_formatted = format!("{}", toronto_time.format("%I:%M:%S %p"));
    write!(time_zones, "### :flag_ca: TO \t|\t {}\n", toronto_formatted).unwrap();

    // Timezone offset for Oregon (Pacific Time), UTC-8
    let oregon_offset = FixedOffset::west(8 * 3600);
    let oregon_time = now_utc.with_timezone(&oregon_offset);
    let oregon_formatted = format!("{}", oregon_time.format("%I:%M:%S %p"));
    write!(time_zones, "### :flag_us: OR \t|\t {}", oregon_formatted).unwrap();

    time_zones
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("time").description("Check the current time of NL, Oregon, and Toronto")
}
