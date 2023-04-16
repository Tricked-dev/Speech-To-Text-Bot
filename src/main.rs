use anyhow::Result;
use autometrics::autometrics;
use metrics_server::start_metrics;
use once_cell::sync::Lazy;
use serenity::{
    async_trait,
    model::{
        application::{
            command::Command,
            interaction::{Interaction, InteractionResponseType},
        },
        gateway::Ready,
        prelude::command::CommandType,
    },
    prelude::*,
};
use tracing::{error, info};

use crate::{handle_slash::handle_slash, utils::*};
use handle_message_ctx::handle_message_ctx;

mod handle_message_ctx;
mod handle_slash;
mod metrics_server;
mod model_data;
mod utils;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[autometrics]
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(comp) = interaction {
            let result: Result<()> = async {
                comp.create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await?;

                tracing::debug!("command: {} ran", &comp.data.name);
                match comp.data.kind {
                    CommandType::ChatInput => handle_slash(ctx, comp).await,
                    CommandType::Message => handle_message_ctx(ctx, comp).await,
                    _ => Ok(()),
                }
            }
            .await;
            if let Err(e) = result {
                error!("{e:?}");
            }
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let ok = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|cmd| cmd.kind(CommandType::Message).name("Transcribe Message"));
            commands.create_application_command(|cmd| {
                cmd.kind(CommandType::ChatInput)
                    .name("privacy")
                    .description("Privacy Policy")
            });
            commands.create_application_command(|cmd| {
                cmd.kind(CommandType::ChatInput)
                    .name("terms")
                    .description("Terms Of Service")
            });
            commands.create_application_command(|cmd| {
                cmd.kind(CommandType::ChatInput)
                    .name("invite")
                    .description("Invite The Bot")
            });
            commands.create_application_command(|cmd| {
                cmd.kind(CommandType::ChatInput)
                    .name("help")
                    .description("The Help Center")
            })
        })
        .await;
        info!(
            "{} is connected! registering commands ok: {}",
            ready.user.name,
            ok.is_ok()
        );
    }
}

#[tokio::main]
#[autometrics]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().compact().init();
    Lazy::force(&WHISPER_CTX);
    let mut client = Client::builder(
        std::env::var("TOKEN").expect("Please set the TOKEN environment variable"),
        GatewayIntents::empty(),
    )
    .event_handler(Handler)
    .await?;

    tokio::spawn(start_metrics());

    client.start().await?;
    Ok(())
}
