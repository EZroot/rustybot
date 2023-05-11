use std::sync::Arc;

use serenity::{
    async_trait,
    client::{ Context, EventHandler},
    model::{gateway::Ready, prelude::ChannelId}, http::Http,
};
use songbird::{EventContext,
    EventHandler as VoiceEventHandler, Event,
};

use crate::messages::check_msg;

pub struct Handler;
pub struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        check_msg(
            self.chan_id
                .say(&self.http, "All songs have finished playing.")
                .await,
        );

        None
    }
}