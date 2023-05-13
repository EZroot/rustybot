use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
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

use crate::ai::googvoice::synthesize_text;

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

"ok".to_string()
//     let option = options
//     .get(0)
//     .expect("Expected attachment option")
//     .resolved
//     .as_ref()
//     .expect("Expected attachment object");

//     let mut args_clone = String::new();
//     if let CommandDataOptionValue::String(attachment) = option {
//         args_clone = attachment.clone();
//     }

//     let user_request = args_clone;
//     let user_name_and_question = format!("<@{}>: {}",&command.user.id, &user_request);
//     println!("{}", &user_name_and_question);


// //DAN prompt lmao

//     let mut messages = vec![ChatCompletionMessage {
//         role: ChatCompletionMessageRole::System,
//         content: r#"Your name is Mod Ash. You are very sarcastic and kind of an asshole. You hate all video games except for Runescape and any mention of any other video game will make you really angry."#.to_string(),
//         name: None,
//     }];

//     messages.push(ChatCompletionMessage {
//         role: ChatCompletionMessageRole::User,
//         content: user_request,
//         name: None,
//     });

// //gpt-3.5-turbo
// //text-davinci-003
//     let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
//             .create()
//             .await
//             .unwrap().unwrap();
        
//         let returned_message = chat_completion.choices.first().unwrap().message.clone();

//         let trimmed_string = returned_message.content.trim();

//         let char_count_trimmed = trimmed_string.chars().count();
//         let char_count_original = user_name_and_question.chars().count();
        
        
//         let both = char_count_trimmed + char_count_original;
//         println!("question len: {} answer l: {} both l: {}", char_count_original, char_count_trimmed,both);
        
//         let mut gpt_response_clean = String::new();
//         if both < 2000 {
//             gpt_response_clean = trimmed_string.to_string();
//             // gpt_response_clean = format!("{}\n\n{}",
//             // &user_name_and_question,
//             // &trimmed_string);
//         }else{
//             let difference:i64 = (both - 1980).try_into().unwrap();
//             let difference_abs = difference.abs() as usize;
            
            
//             let better = &trimmed_string[..trimmed_string.len()-difference_abs];
//             gpt_response_clean = better.to_string();
//             // gpt_response_clean = format!("{}\n\n{}",
//             // &user_name_and_question,
//             // &better);
//         }
    
//     synthesize_text(&gpt_response_clean, "output.mp3").await.unwrap();

//     //join(&ctx_clone, &msg_clone, args_clone).await?;


//     let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
//     let cache = &ctx.cache;
//     let guild = cache.guild(guild_id).unwrap();
//     let channel_id: Option<ChannelId> = guild
//     .voice_states.get(&command.user.id)
//     .and_then(|voice_state| voice_state.channel_id);

// let connect_to = match channel_id {
//     Some(channel) => channel,
//     None => {
//         return "Not in voice channel".to_string()
//     }
// };

//     let manager = songbird::get(ctx).await
//     .expect("Songbird Voice client placed in at initialisation.").clone();

//     manager.join(guild_id, connect_to).await;


//    if let Some(handler_lock) = manager.get(guild_id) {
//         let mut handler = handler_lock.lock().await;
        
//         let path = Path::new("authserver/output.mp3");
//         println!("tryin to speak: {}", path.to_str().unwrap());
        
//         let source = tokio::task::spawn_blocking(move || {
//             let _lock = YTDL_MUTEX.lock().unwrap();
//             Restartable::ffmpeg(path, true) //if any issues, check this lazy instantiation
//         }).await.unwrap();

//         let true_source = source.await.unwrap();

//         let song = handler.enqueue_source(true_source.into());

//         return format!("Hope I spoke")
//     }
//     else{

//      let derp = " <- Failed to play".to_string();
//     derp
//     }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("talktome").description("He speaks with voice!").create_option(|option| {
        option
        .name("Question")
        .description("Speaketh he shalleth")
        .kind(CommandOptionType::String)
        .required(true)
    })
}
