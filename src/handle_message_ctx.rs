use anyhow::Result;
use autometrics::autometrics;
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
pub async fn handle_message_ctx(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
    let msg = interaction.data.resolved.messages.iter().next().unwrap().1;
    let file = match msg.attachments.get(0) {
        Some(v) => v,
        None => {
            return message_early_exit("Sorry that message doesn't have any voice attached!", ctx, interaction).await;
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

    //These functions do fs stuff and cant
    if std::fs::metadata(fname.clone()).is_err() {
        fetch_url(file.url.clone(), fname.clone()).await?;
    }
    transcode_video(&fname, &out).await?;

    let result = speech_to_text(out).await?;

    let end = format!("{} {}", msg.link(), result.trim());

    interaction
        .edit_original_interaction_response(&ctx.http, |response| response.content(end).allowed_mentions(|x| x))
        .await?;
    Ok(())
}
