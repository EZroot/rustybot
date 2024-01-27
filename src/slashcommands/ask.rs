use colored::Colorize;
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

use crate::ai::openai::{generate_openai_response, generate_openai_response_include_username};
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

    let cmd_clone = command.user.clone();
    let ai_req = args_clone.clone();
    let raw_ai_response =
        generate_openai_response_include_username(cmd_clone.name, ai_req, cmd_clone.id).await;

    
        let user_name_and_question = format!("<@{}>: {}", cmd_clone.id, args_clone);
        let char_count_trimmed = raw_ai_response.chars().count();
        let char_count_original = user_name_and_question.chars().count();
    
        let both = char_count_trimmed + char_count_original;
        // println!(
        //     "question len: {} answer l: {} both l: {}",
        //     char_count_original, char_count_trimmed, both
        // );
        
        let mut cleaned_up_string = String::new();
        if both < 2000 {
            cleaned_up_string = raw_ai_response.to_string();
        } else {
            let difference: i64 = (both - 1980).try_into().unwrap();
            let difference_abs = difference.abs() as usize;
    
            let better = &raw_ai_response[..raw_ai_response.len() - difference_abs];
            cleaned_up_string = better.to_string();
        }
    format!("{}\n\n{}", user_name_and_question, cleaned_up_string)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ask").description("Speak with Mod Ash").create_option(|option| {
        option
        .name("question")
        .description("your dumb question")
        .kind(CommandOptionType::String)
        .required(true)
    })
}
