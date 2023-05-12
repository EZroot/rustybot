use serenity::{
    client::Context,
    framework::{
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::channel::Message,
};
use songbird::{
    input::Restartable, tracks::TrackQueue
};

use std::{time::Duration};

use crate::messages::check_msg;

#[group]
#[commands(help, deafen, join, leave, mute, play, queue, ping, undeafen, unmute)]
struct General;

#[command]
#[only_in(guilds)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Commands - /help \n /play youtubeurl \n /queue \n /leave").await);
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        check_msg(msg.channel_id.say(&ctx.http, "Already deafened").await);
    } else {
        if let Err(e) = handler.deafen(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Deafened").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    // let mut debuginfo = "guildid: ".to_string();
    // debuginfo += &guild_id.to_string();

    // check_msg(msg.reply(ctx, debuginfo).await);

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        check_msg(msg.channel_id.say(&ctx.http, "Already muted").await);
    } else {
        if let Err(e) = handler.mute(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Now muted").await);
    }

    Ok(())
}

#[command]
async fn ping(context: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, "Pong!").await);

    Ok(())
}


#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let ctx_clone = ctx.clone();
    let msg_clone = msg.clone();

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let song: &TrackQueue = handler.queue();
        
        let mut all_songs = String::new();
        for track in handler.queue().current_queue().iter()
        {
            all_songs += track.metadata().title.as_ref().unwrap();
            all_songs += " \n";
        }
        
        let sent_msg =  msg.channel_id
        .say(
            &ctx.http,
            format!("Songs in Queue: \n{}",&all_songs),
        )
        .await?;

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(10)).await;
            if let Err(why) = sent_msg.delete(&ctx_clone.http).await {
                println!("Error deleting message: {:?}", why);
            }

             // Delete the user's command message after the delay
        if let Err(why) = msg_clone.delete(&ctx_clone.http).await {
            println!("Error deleting message: {:?}", why);
        }
        });
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "No songs in queue!").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    
    let ctx_clone = ctx.clone();
    let msg_clone = msg.clone();
    let args_clone = args.clone();

    join(&ctx_clone, &msg_clone, args_clone).await?;

    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        println!("Url {}", &url);
      
      
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        let song = handler.enqueue_source(source.into());

        // This shows how to fire an event once an audio track completes,
        // either due to hitting the end of the bytestream or stopped by user code.
        let song_title = song.metadata().title.as_ref().unwrap();
        
        let sent_msg =  msg.channel_id
        .say(
            &ctx.http,
            format!("Added {} to queue #{}",&song_title, handler.queue().len()),
        )
        .await?;

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(10)).await;
            if let Err(why) = sent_msg.delete(&ctx_clone.http).await {
                println!("Error deleting message: {:?}", why);
            }

             // Delete the user's command message after the delay
        if let Err(why) = msg_clone.delete(&ctx_clone.http).await {
            println!("Error deleting message: {:?}", why);
        }
        });
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.deafen(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Undeafened").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to undeafen in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.mute(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Unmuted").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to unmute in").await);
    }

    Ok(())
}