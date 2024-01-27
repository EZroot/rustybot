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
use std::sync::{Mutex, Arc};

use lazy_static::lazy_static;

use crate::ai::openai::generate_openai_response_include_username;
use crate::ai::voicesynth::synthesize_text;
use crate::messages::check_msg;

lazy_static! {
    static ref YTDL_SEARCH_MUTEX: Mutex<()> = Mutex::new(());
}


lazy_static! {
    static ref YTDL_MUTEX: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}



// pub async fn ask(context: &Context, msg: &Message, mut args: Args) -> CommandResult {

//     let url = match args.single::<String>() {
//         Ok(url) => url,
//         Err(_) => {
//             check_msg(msg.channel_id.say(&context.http, "Question? Sorry I didn't quite catch that.").await);
//             return Ok(())
//         },
//     };

//     let raw_ai_response =
//     generate_openai_response_include_username("Dillon".to_string(), url, 263098093824638979.into()).await;
    
//     synthesize_text(&raw_ai_response, "output.mp3")
//         .await
//         .unwrap();

//         let path = Path::new("authserver/output.mp3");
//     println!("tryin to speak: {}", path.to_str().unwrap());

//     let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
//     let cache = &context.cache;
//     let voice_channel_id = ChannelId::from(498136555102273537);


//     let manager = songbird::get(context).await
//     .expect("Songbird Voice client placed in at initialisation.");

//     manager.join(guild_id, voice_channel_id).await;

//     if let Some(handler_lock) = manager.get(guild_id) {
//         let mut handler = handler_lock.lock().await;

//         let path = Path::new("authserver/output.mp3");
//         println!("tryin to speak: {}", path.to_str().unwrap());

//         let source = match Restartable::ffmpeg(path, true).await {
//             Ok(source) => source,
//             Err(why) => {
//                 println!("Err starting source: {:?}", why);

//                 check_msg(msg.channel_id.say(&context.http, "Error sourcing ffmpeg").await);

//                 return Ok(());
//             },
//         };

//         let song = handler.play_source(source.into());

//     } else {
//         let derp = " <- Failed to play".to_string();
//         println!("{}",derp);
//     }

//     //msg.delete(&context).await.unwrap();
//     //check_msg(msg.channel_id.say(&context.http, "Paint? Sorry I didn't quite catch that.").await);
//     let msgid = msg.id;
//     let mut newMsg = msg.channel_id.message(&context, msgid).await.unwrap();
//     newMsg.edit(&context, |m| {
//         m.content(raw_ai_response)
//     }).await.unwrap();

//     Ok(())
// }
