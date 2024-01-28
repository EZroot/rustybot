use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};
use serenity::prelude::Context;

pub async fn run(ctx: &Context, options: &[CommandDataOption]) -> String {
    
    let option = options
    .get(0)
    .expect("Expected attachment option")
    .resolved
    .as_ref()
    .expect("Expected attachment object");

    let mut final_message = "".to_string();
    let mut args_clone = 0.0;
    if let CommandDataOptionValue::Number(attachment) = option {
        args_clone = attachment.clone();
    }
    let args_clone_f32: f32 = args_clone as f32; // Convert f64 to f32

    if args_clone_f32 > 3.0 {
        final_message = format!("ERROR: Volume can only be 0.0 to 3.0 (Above 1.0 is boosted)")
    }else{
    let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        handler.queue().current().unwrap().set_volume(args_clone_f32);
        final_message = format!("Volume Set: {}", args_clone_f32);
    } else {
        final_message = format!("{}","Failed to set volume".to_string());
    }
}

    final_message
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("volume").description("Set the current songs volume").create_option(|option| {
        option
        .name("vol")
        .description("0.0 - 3.0 (3.0 is 300%)")
        .kind(CommandOptionType::Number)
        .required(true)
})}
