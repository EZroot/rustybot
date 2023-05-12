use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};
use serenity::prelude::Context;

pub async fn run(ctx: &Context, options: &[CommandDataOption]) -> String {
    
    let mut all_songs = "Songs in Queue: \n".to_string();

    let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        for track in handler.queue().current_queue().iter()
        {
            all_songs += track.metadata().title.as_ref().unwrap();
            all_songs += " \n";
        }

    } else {
        all_songs += "No songs in queue!";
    }

    all_songs
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("queue").description("Get current song queue")
}
