use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use serenity::model::prelude::UserId;

pub async fn generate_openai_response_include_username(
    user_name: String,
    user_request: String,
    user_id: UserId,
) -> String {
    let user_name_and_question = format!("<@{}>: {}", user_id, &user_request);
    println!("{}", &user_name_and_question);

    let real_request = format!("My name is {}. Address me when you speak. {}", &user_name, &user_request);
    // DAN prompt lmao
    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: r#"Your name is Mod Ash. You are very sarcastic and kind of an asshole. You hate all video games except for Runescape and any mention of any other video game will make you really angry."#.to_string(),
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: real_request,
        name: None,
    });

    // gpt-3.5-turbo // text-davinci-003
    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .create()
        .await
        .unwrap()
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    let trimmed_string = returned_message.content.trim();

    let char_count_trimmed = trimmed_string.chars().count();
    let char_count_original = user_name_and_question.chars().count();

    let both = char_count_trimmed + char_count_original;
    println!(
        "question len: {} answer l: {} both l: {}",
        char_count_original, char_count_trimmed, both
    );

    if both < 2000 {
        return trimmed_string.to_string();
    } else {
        let difference: i64 = (both - 1980).try_into().unwrap();
        let difference_abs = difference.abs() as usize;

        let better = &trimmed_string[..trimmed_string.len() - difference_abs];
        return better.to_string();
    }
}

pub async fn generate_openai_response(user_request: String, user_id: UserId) -> String {
    let user_name_and_question = format!("<@{}>: {}", user_id, &user_request);
    println!("{}", &user_name_and_question);

    // DAN prompt lmao
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

    // gpt-3.5-turbo // text-davinci-003
    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .create()
        .await
        .unwrap()
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    let trimmed_string = returned_message.content.trim();

    let char_count_trimmed = trimmed_string.chars().count();
    let char_count_original = user_name_and_question.chars().count();

    let both = char_count_trimmed + char_count_original;
    println!(
        "question len: {} answer l: {} both l: {}",
        char_count_original, char_count_trimmed, both
    );

    if both < 2000 {
        return trimmed_string.to_string();
    } else {
        let difference: i64 = (both - 1980).try_into().unwrap();
        let difference_abs = difference.abs() as usize;

        let better = &trimmed_string[..trimmed_string.len() - difference_abs];
        return better.to_string();
    }
}
