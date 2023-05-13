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

    let user_request = args_clone;
    let user_name_and_question = format!("<@{}>: {}",&command.user.id, &user_request);
    println!("{}", &user_name_and_question);

    //let response = generate_openai_response(user_request, command.user.id).await;
    let cmd_clone = command.user.clone();
    let response = generate_openai_response_include_username(cmd_clone.name, user_request, cmd_clone.id).await;
    format!("{}\n\n{}", user_name_and_question, response)
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
