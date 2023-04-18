use std::{env::temp_dir, path::Path};

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

    let content_type = file.content_type.clone().unwrap_or_default();

    if !content_type.starts_with("audio/") && !content_type.starts_with("video/") {
        return message_early_exit("Sorry i cannot transcribe that!", ctx, interaction).await;
    }

    if file.filename.contains([' ', '/', ',', '!', '"', '\'', '\\']) && !file.filename.starts_with('-') {
        return message_early_exit("Sorry thats a illegal filename!", ctx, interaction).await;
    }

    if file.size > 2097152 {
        return message_early_exit("Sorry file too big!", ctx, interaction).await;
    }

    let lang_id = match (interaction.guild_locale.clone(), interaction.locale.clone()) {
        (_, u_locale) if !u_locale.contains("en") => u_locale,
        (Some(g_locale), _) if !g_locale.contains("en") => g_locale,
        (_, locale) => locale,
    };

    let cache_key = format!("{}{lang_id}", msg.id);

    let fname = temp_dir().join(format!(
        "{}.{}",
        uuid::Uuid::new_v4(),
        Path::new(&file.filename)
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
    ));

    let content = match cache.get(&cache_key) {
        Some(v) => v.clone(),
        _ => {
            fetch_url(file.url.clone(), fname.clone()).await?;
            let data = read_file(fname.clone()).await?;
            let result = speech_to_text(data, lang_id).await?;

            tokio::spawn(async {
                fs::remove_file(fname).await?;
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
