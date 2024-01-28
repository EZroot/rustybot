use colored::Colorize;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use serenity::model::prelude::UserId;

pub async fn generate_openai_response_include_username(
    user_request: String,
    user_id: UserId,
    custom_prompt: String,
    gpt_model: String,
) -> String {

//    let real_request = format!("Just to remind you, my name is {}. {}", &user_name, &user_request);
let real_request = format!("{}", &user_request);

    // DAN prompt lmao
    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: custom_prompt,
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: real_request,
        name: None,
    });

    // gpt-3.5-turbo // text-davinci-003
    let chat_completion = ChatCompletion::builder(&gpt_model, messages.clone())
        .create()
        .await
        .unwrap()
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();
    let debug_msg = returned_message.content.trim().clone();
    println!("Question: {}", &user_request.bright_green());
    println!("Gpt: {}", debug_msg.bright_blue());
    return returned_message.content.trim().to_string();
}

pub async fn generate_openai_response(user_request: String, user_id: UserId, custom_prompt: String, gpt_model:String,) -> String {
    let user_name_and_question = format!("<@{}>: {}", user_id, &user_request);

    // DAN prompt lmao
    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: custom_prompt,
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: user_request,
        name: None,
    });

    // gpt-3.5-turbo // text-davinci-003
    let chat_completion = ChatCompletion::builder(&gpt_model, messages.clone())
        .create()
        .await
        .unwrap()
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    let trimmed_string = returned_message.content.trim();

    let char_count_trimmed = trimmed_string.chars().count();
    let char_count_original = user_name_and_question.chars().count();

    let both = char_count_trimmed + char_count_original;
    // println!(
    //     "question len: {} answer l: {} both l: {}",
    //     char_count_original, char_count_trimmed, both
    // );

    if both < 2000 {
        return trimmed_string.to_string();
    } else {
        let difference: i64 = (both - 1980).try_into().unwrap();
        let difference_abs = difference.abs() as usize;

        let better = &trimmed_string[..trimmed_string.len() - difference_abs];
        return better.to_string();
    }
}
