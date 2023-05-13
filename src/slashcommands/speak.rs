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

use std::path::Path;
use std::sync::{Mutex, Arc};

use lazy_static::lazy_static;

use crate::ai::voicesynth::synthesize_text;

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
/*




        Keeping this but for now, we dont want it.




 */

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

    
    synthesize_text(&args_clone, "output.mp3").await.unwrap();

    //join(&ctx_clone, &msg_clone, args_clone).await?;


    let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
    let cache = &ctx.cache;
    let guild = cache.guild(guild_id).unwrap();
    let channel_id: Option<ChannelId> = guild
    .voice_states.get(&command.user.id)
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
        
        let path = Path::new("authserver/output.mp3");
        println!("tryin to speak: {}", path.to_str().unwrap());
        
        let source = tokio::task::spawn_blocking(move || {
            let _lock = YTDL_MUTEX.lock().unwrap();
            Restartable::ffmpeg(path, true) //if any issues, check this lazy instantiation
        }).await.unwrap();

        let true_source = source.await.unwrap();

        let song = handler.enqueue_source(true_source.into());

        return format!("Hope I spoke")
    }
    else{

     let derp = " <- Failed to play".to_string();
    derp
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("speak").description("Make our lord Ash talk").create_option(|option| {
        option
        .name("text")
        .description("Speaketh he shalleth")
        .kind(CommandOptionType::String)
        .required(true)
    })
}
