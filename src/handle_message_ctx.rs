use tokio::fs;

use anyhow::{Error, Result};
use autometrics::autometrics;
use dashmap::DashMap;
use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};

use crate::utils::*;

#[autometrics]
async fn message_early_exit(content: &str, ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
    interaction
        .edit_original_interaction_response(&ctx.http, |response| response.content(content))
        .await?;
    Ok(())
}

#[autometrics]
pub async fn handle_message_ctx(
    ctx: Context,
    interaction: ApplicationCommandInteraction,
    cache: &DashMap<String, String>,
) -> Result<()> {
    let msg = interaction.data.resolved.messages.iter().next().unwrap().1;
    let file = match msg.attachments.get(0) {
        Some(v) => v,
        None => {
            return message_early_exit("Sorry that message doesn't have any voice attached!", ctx, interaction).await;
        }
    };

    match cache.get(&msg.id.to_string()) {
        None => cache.insert(msg.id.to_string(), String::new()),
        _ => {
            return message_early_exit(
                "Sorry this message is already being processed please wait!",
                ctx,
                interaction,
            )
            .await;
        }
    };

    if file.content_type != Some("audio/ogg".to_lowercase()) || !file.filename.ends_with(".ogg") {
        return message_early_exit("Sorry thats not a voice message!", ctx, interaction).await;
    }

    if file.filename.contains([' ', '/', '-', ',', '!', '"', '\'', '\\']) && file.filename != "voice-message.ogg" {
        return message_early_exit("Sorry thats a illegal filename!", ctx, interaction).await;
    }

    if file.size > 2097152 {
        return message_early_exit("Sorry file too big!", ctx, interaction).await;
    }

    let fname = format!("out/{}-{}", msg.id, file.filename);
    let out = fname.replace(".ogg", ".wav");

    let lang_id = match (interaction.guild_locale.clone(), interaction.locale.clone()) {
        (_, u_locale) if !u_locale.contains("en") => u_locale,
        (Some(g_locale), _) if !g_locale.contains("en") => g_locale,
        (_, locale) => locale,
    };

    let cache_key = format!("{}{lang_id}", msg.id);

    let content = match cache.get(&cache_key) {
        Some(v) => v.clone(),
        _ => {
            fetch_url(file.url.clone(), fname.clone()).await?;
            transcode_video(&fname, &out).await?;

            let result = speech_to_text(out.clone(), lang_id).await?;

            tokio::spawn(async {
                fs::remove_file(fname).await?;
                fs::remove_file(out).await?;
                Ok::<(), Error>(())
            });

            let end = format!("{} {}", msg.link(), result.trim());
            cache.insert(cache_key, end.clone());
            end
        }
    };

    interaction
        .edit_original_interaction_response(&ctx.http, |response| response.content(content).allowed_mentions(|x| x))
        .await?;

    cache.remove(&msg.id.to_string());
    Ok(())
}
