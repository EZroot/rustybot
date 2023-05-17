use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use reqwest::Client as ReqwestClient;
use serde_json::json;
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::{
    interaction, Activity, ChannelId, InteractionApplicationCommandCallbackDataFlags, Message,
};
use serenity::model::Timestamp;
use serenity::prelude::*;
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};

use chrono::{Local, Utc};
use tokio::io::{self, AsyncWriteExt};

use std::sync::{ Mutex};


use crate::messages::check_msg;
use crate::{commands, slashcommands};
use std::path::{self, PathBuf};


// Define a global shared data struct
pub struct GlobalData {
    pub queued_command: String,
    // Add other fields as needed
}


// Create a global mutable shared data using an RwLock
lazy_static::lazy_static! {
    pub static ref GLOBAL_DATA: Arc<RwLock<GlobalData>> = Arc::new(RwLock::new(GlobalData {
        queued_command: "Empty".to_string(),
        // Initialize other fields
    }));
}

pub async fn update_global_data(command : String) -> Result<(),()> {
    // Lock the mutex to access the shared data
    let mut global_data = GLOBAL_DATA.write().await;
    // Access and modify the shared data
    global_data.queued_command = command;
    println!("Global data successfuly updated");
    // Unlock the mutex to release the lock
    // The lock will be automatically released when `global_data` goes out of scope
    Ok(())
}

pub struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

pub struct EventHandlers {
    pub slash_command_handler: SlashCommandHandler,
    pub http_request_handler: HttpRequestHandler,
    // Add more handlers as needed
}

pub struct SlashCommandHandler {
    pub is_loop_running: AtomicBool,
}

pub struct HttpRequestHandler {
    pub http: Arc<ReqwestClient>,
    pub channel_id: ChannelId, 
}

pub struct HttpHandler;

impl TypeMapKey for HttpHandler {
    type Value = ChannelId;
}

#[async_trait]
impl EventHandler for EventHandlers {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let ctx_copy = ctx.clone();
        let ready_copy = ready.clone();
        self.slash_command_handler.ready(ctx, ready).await;
        self.http_request_handler.ready(ctx_copy, ready_copy).await;
        // Call the appropriate ready function for each handler
    }
    
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        self.slash_command_handler.interaction_create(ctx, interaction).await;
    }
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        self.slash_command_handler.cache_ready(ctx, _guilds).await;
    }
}


impl HttpRequestHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot is ready!");

        // Send a message when the bot is ready
        let _ = self
            .channel_id
            .say(&ctx.http, "Bot is ready!")
            .await;
    }
}


impl SlashCommandHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //println!("Received command interaction: {:#?}", command);
            let content = match command.data.name.as_str() {
                "queue" => slashcommands::queue::run(&ctx, &command.data.options).await,
                "play" => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.flags(interaction::MessageFlags::EPHEMERAL)
                                })
                        })
                        .await
                        .unwrap();

                    let track =
                        slashcommands::play::run(&ctx, &command, &command.data.options).await;

                    command
                        .edit_original_interaction_response(&ctx.http, |reponse| {
                            reponse.content(track)
                        })
                        .await
                        .unwrap();
                    return;
                }
                "search" => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.flags(interaction::MessageFlags::EPHEMERAL)
                                })
                        })
                        .await
                        .unwrap();

                    let track =
                        slashcommands::search::run(&ctx, &command, &command.data.options).await;

                    command
                        .edit_original_interaction_response(&ctx.http, |reponse| {
                            reponse.content(track)
                        })
                        .await
                        .unwrap();
                    return;
                }
                "ask" => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                                .interaction_response_data(|message| message.ephemeral(false))
                        })
                        .await
                        .unwrap();

                    let track =
                        slashcommands::ask::run(&ctx, &command, &command.data.options).await;

                    command
                        .edit_original_interaction_response(&ctx.http, |reponse| {
                            reponse.content(track)
                        })
                        .await
                        .unwrap();
                    return;
                }
                "askloud" => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                                .interaction_response_data(|message| message.ephemeral(false))
                        })
                        .await
                        .unwrap();

                    let track =
                        slashcommands::askloud::run(&ctx, &command, &command.data.options).await;

                    command
                        .edit_original_interaction_response(&ctx.http, |reponse| {
                            reponse.content(track)
                        })
                        .await
                        .unwrap();
                    return;
                }
                "paint" => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message
                                        .content("Generating image... give me a few minutes.")
                                })
                        })
                        .await
                        .unwrap();

                    let file_path = slashcommands::paint::run(&command.data.options).await;
                    println!("file_path {}", &file_path);
                    let file = PathBuf::from(file_path);

                    // Send the file as a follow-up message
                    command
                        .create_followup_message(&ctx.http, |message| {
                            message.add_file(&file)
                        })
                        .await
                        .unwrap();

                    return;
                }
                "paintdetailed" => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message
                                        .content("Generating image... give me a few minutes.")
                                })
                        })
                        .await
                        .unwrap();

                    let file_path = slashcommands::paintdetailed::run(&command.data.options).await;
                    println!("file_path {}", &file_path);
                    let file = PathBuf::from(file_path);

                    // Send the file as a follow-up message
                    command
                        .create_followup_message(&ctx.http, |message| {
                            message.add_file(&file)
                        })
                        .await
                        .unwrap();

                    return;
                }
                "knowledge" => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                                .interaction_response_data(|message| message.ephemeral(false))
                        })
                        .await
                        .unwrap();

                    let track =
                        slashcommands::knowledge::run(&ctx, &command, &command.data.options).await;

                    command
                        .edit_original_interaction_response(&ctx.http, |reponse| {
                            reponse.content(track)
                        })
                        .await
                        .unwrap();
                    return;
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content(content)
                                .flags(interaction::MessageFlags::EPHEMERAL)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let ready_clone = ready.clone();
        println!("{} is connected!", &ready_clone.user.name);

        //need to do this on channel join event probably?
        // if let Some(guild) = ready_clone.guilds.first() {
        //     self.guild_id = Some(guild.id);
        // }

        let guild_id = GuildId::from(308708637679812608); //self.guild_id.unwrap();

        let ctx_arc = ctx.clone();

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, move |commands| {
            commands
                .create_application_command(|command| slashcommands::ask::register(command))
                .create_application_command(|command| slashcommands::askloud::register(command))
                .create_application_command(|command| slashcommands::search::register(command))
                .create_application_command(|command| slashcommands::queue::register(command))
                .create_application_command(|command| slashcommands::play::register(command))
                .create_application_command(|command| slashcommands::knowledge::register(command))
                .create_application_command(|command| slashcommands::paint::register(command))
                .create_application_command(|command| slashcommands::paintdetailed::register(command))
        })
        .await;
        // let commands: Result<Vec<Command>, SerenityError> = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        //     commands.create_application_command(|command| {
        //         command.name("ping").description("Ping command")}) })
        // .await;
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        // it's safe to clone Context, but Arc is cheaper for this use case.
        // Untested claim, just theoretically. :P
        let ctx = Arc::new(ctx);

        // We need to check that the loop is not already running when this event triggers,
        // as this event triggers every time the bot enters or leaves a guild, along every time the
        // ready shard event triggers.
        //
        // An AtomicBool is used because it doesn't require a mutable reference to be changed, as
        // we don't have one due to self being an immutable reference.
        if !self.is_loop_running.load(Ordering::Relaxed) {
            // We have to clone the Arc, as it gets moved into the new thread.
            let ctx1 = Arc::clone(&ctx);
            // tokio::spawn creates a new green thread that can run in parallel with the rest of
            // the application.
            // tokio::spawn(async move {
            //     loop {
            //         // We clone Context again here, because Arc is owned, so it moves to the
            //         // new function.
            //         log_system_load(Arc::clone(&ctx1)).await;
            //         tokio::time::sleep(Duration::from_secs(120)).await;
            //     }
            // });

            // And of course, we can run more than one thread at different timings.
            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                
                let mut counter : usize  = 0;
                loop {
                    counter += 1;
                    if counter > 3 {
                        counter = 0;
                    }
                    
                    let dots: String = std::iter::repeat('.').take(counter).collect();
                    let message = format!("Polling Commands{} ", dots);
            
                    print!("\r{}{}", message, " ".repeat(20 - message.len()));
                    std::io::stdout().flush().unwrap();
                        // Lock the mutex to access the shared data
                    let mut global_data = GLOBAL_DATA.write().await;
        
                    // Access and modify the counter field
                    let command = global_data.queued_command.clone();
                    if command.len() > 0 {
                        // Do something with the updated counter value
                        let message_content = json!({
                            "content": command,
                        });
                        println!("Polled command: {}", message_content);

                        let channel = &ctx2.cache.guild_channel(703698331141931078).unwrap();
                        channel.send_message(&ctx2, |m| m.content(command)).await.unwrap();

                        global_data.queued_command = "".to_string();
                    }

                    set_status_to_current_time(Arc::clone(&ctx2)).await;
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            });

            // Now that the loop is running, we set the bool to true
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
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

async fn log_system_load(ctx: Arc<Context>) {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    let message = ChannelId(703698331141931078)
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("System Resource Load")
                    .field(
                        "CPU Load Average",
                        format!("{:.2}%", cpu_load.one * 10.0),
                        false,
                    )
                    .field(
                        "Memory Usage",
                        format!(
                            "{:.2} MB Free out of {:.2} MB",
                            mem_use.free as f32 / 1000.0,
                            mem_use.total as f32 / 1000.0
                        ),
                        false,
                    )
            })
        })
        .await;
    if let Err(why) = message {
        eprintln!("Error sending message: {:?}", why);
    };
}

async fn set_status_to_current_time(ctx: Arc<Context>) {
    let guild_id = GuildId::from(308708637679812608); //self.guild_id.unwrap();
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        if let Some(track) = handler.queue().current().as_ref() {
            let data = track.metadata();
            let duration = data.duration.unwrap().as_secs();
            let duration_mins = duration / 60;
            let duration_secs_remaining = duration % 60;
            let duration_formatted = format!("{}:{:02}", duration_mins, duration_secs_remaining);

            if let Some(title) = data.title.clone() {
                let formatted_text = format!("{} - {}", &title, &duration_formatted);
                ctx.set_activity(Activity::listening(&formatted_text)).await;
            } else {
                println!("Failed to get song data");
            }
        }
    } else {
        let current_time = Local::now();
        let formatted_time = current_time.format("NL: %I:%M:%S %p").to_string();
        ctx.set_activity(Activity::playing(&formatted_time)).await;
    }

    //ctx.set_activity(Activity::playing(&formatted_time)).await;
}
