use serenity::{
    client::Context,
    framework::{
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::channel::Message,
    Result as SerenityResult,
};
use songbird::{
    Event, EventHandler, EventContext, TrackEvent,
    input::Input
};

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}