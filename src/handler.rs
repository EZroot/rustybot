
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::{ChannelId, InteractionApplicationCommandCallbackDataFlags, interaction};
use serenity::model::prelude::application_command::ApplicationCommandOptionType;
use serenity::prelude::*;
use songbird::{EventContext, Event, EventHandler as VoiceEventHandler};

use crate::{commands, slashcommands};
use crate::messages::check_msg;

pub struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => slashcommands::ping::run(&command.data.options),
                "queue" => slashcommands::queue::run(&ctx, &command.data.options).await,
                "play" => {

                    command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                            .interaction_response_data(|message| {message
                                .flags(interaction::MessageFlags::EPHEMERAL)})
                    })
                    .await
                    .unwrap();

                let track = slashcommands::play::run(&ctx, &command, &command.data.options).await;

                command
                .edit_original_interaction_response(&ctx.http, |reponse| {
                    reponse
                    .content(track)
                })
                .await
                .unwrap();
            return;
                },
                "search" =>{

                    command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                            .interaction_response_data(|message| {message
                                .flags(interaction::MessageFlags::EPHEMERAL)})
                    })
                    .await
                    .unwrap();

                let track = slashcommands::search::run(&ctx, &command, &command.data.options).await;

                command
                .edit_original_interaction_response(&ctx.http, |reponse| {
                    reponse
                    .content(track)
                })
                .await
                .unwrap();
            return;
                },
                _ => "not implemented :(".to_string(),
            };
            
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {message
                        .content(content)
                        .flags(interaction::MessageFlags::EPHEMERAL)})
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
        
        let guild_id = GuildId::from(308708637679812608);//self.guild_id.unwrap();


        let ctx_arc = ctx.clone();

let commands = GuildId::set_application_commands(&guild_id, &ctx.http, move |commands| {
    commands
    .create_application_command(|command| slashcommands::ping::register(command))
    .create_application_command(|command| slashcommands::queue::register(command))
    .create_application_command(|command| slashcommands::play::register(command))
    .create_application_command(|command| slashcommands::search::register(command))
})
.await;
        // let commands: Result<Vec<Command>, SerenityError> = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        //     commands.create_application_command(|command| {
        //         command.name("ping").description("Ping command")}) })
        // .await;
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