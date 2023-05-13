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

use std::sync::Mutex;
use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use std::{
    env,
    io::{stdin, stdout, Write},
};
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

    let user_request = args_clone;
    let user_name_and_question = format!("<@{}>: {}",&command.user.id, &user_request);
    println!("{}", &user_name_and_question);


//DAN prompt lmao

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: r#"Your name is Mod Ash. You are very sarcastic and kind of an asshole. You hate all video games except for Runescape and any mention of any other video game will make you really angry."#.to_string(),
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: user_request,
        name: None,
    });

//gpt-3.5-turbo
//text-davinci-003
    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
            .create()
            .await
            .unwrap().unwrap();
        
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        let trimmed_string = returned_message.content.trim();

        let char_count_trimmed = trimmed_string.chars().count();
        let char_count_original = user_name_and_question.chars().count();
        
        
        let both = char_count_trimmed + char_count_original;
        println!("question len: {} answer l: {} both l: {}", char_count_original, char_count_trimmed,both);

        if both < 2000 {
            format!("{}\n\n{}",
            &user_name_and_question,
            &trimmed_string)
        }else{
            let difference:i64 = (both - 1980).try_into().unwrap();
            let difference_abs = difference.abs() as usize;
            
            
            let better = &trimmed_string[..trimmed_string.len()-difference_abs];
            format!("{}\n\n{}",
            &user_name_and_question,
            &better)
        }

        

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hey").description("speak with Mod Ash").create_option(|option| {
        option
        .name("question")
        .description("your dumb question")
        .kind(CommandOptionType::String)
        .required(true)
    })
}
