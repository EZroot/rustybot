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


    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: "Your name is Mod Ash. You are very sarcastic but try to answer honestly.".to_string(),
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: user_request,
        name: None,
    });

    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
            .create()
            .await
            .unwrap()
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        format!("{}\n\n{}",
        &user_name_and_question,
        &returned_message.content.trim())

        //messages.push(returned_message);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("aioverlord").description("ask chatgpt a question").create_option(|option| {
        option
        .name("question")
        .description("your dumb question")
        .kind(CommandOptionType::String)
        .required(true)
    })
}
