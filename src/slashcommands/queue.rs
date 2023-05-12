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

        let mut counter = 0;
        for track in handler.queue().current_queue().iter()
        {
            let meta_data = track.metadata().clone();
            let leftover_seconds: u64 = meta_data.duration.unwrap().as_secs() % 60;
            let remove_this_from_truemins = leftover_seconds / 60; 
            let true_mins: u64 = (meta_data.duration.unwrap().as_secs() - remove_this_from_truemins) / 60;

            let formated_dur = format!("{}:{}", &true_mins, &leftover_seconds);
            let formated_song = format!("#{} {} [{}] \n", counter.to_string(), &meta_data.title.unwrap(), &formated_dur.to_string());
            all_songs += &formated_song;
            counter+=1;
        }

    } else {
        all_songs += "No songs in queue!";
    }

    all_songs
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("queue").description("Get current song queue")
}
