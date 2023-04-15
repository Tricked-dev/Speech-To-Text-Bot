use std::{io::Cursor, path::PathBuf};

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

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

struct Handler;

async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

pub fn wav_to_integer_mono(file: &PathBuf) -> Result<Vec<f32>> {
    let mut reader = hound::WavReader::open(file)?;
    let hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: _,
        ..
    } = reader.spec();
    let r = &reader
        .samples::<i16>()
        .map(|s| s.expect("invalid sample"))
        .collect::<Vec<_>>();
    let mut audio = whisper_rs::convert_integer_to_float_audio(r);

    if sample_rate != 16000 {
        return Err(anyhow!("Sample Rate Issue!"));
    }

    if channels == 2 {
        audio = whisper_rs::convert_stereo_to_mono_audio(&audio).unwrap();
    }

    Ok(audio)
}

async fn transcode_video(nin: &str, out: &str) -> Result<()> {
    let _res = tokio::process::Command::new("ffmpeg")
        .arg("-loglevel")
        .arg("quiet")
        .arg("-y")
        .arg("-i")
        .arg(nin)
        .arg("-ar")
        .arg("16000")
        .arg(out)
        .status()
        .await?;
    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(comp) = interaction {
            let result: Result<()> = async {
                comp.create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await?;
                let msg = comp.data.resolved.messages.iter().next().unwrap().1;
                let file = match msg.attachments.get(0) {
                    Some(v) => v,
                    None => {
                        comp.edit_original_interaction_response(&ctx.http, |response| {
                            response.content("Sorry that message doesn't have any voice attached")
                        })
                        .await?;
                        return Err(anyhow!("No file on message"));
                    }
                };

                if file.content_type != Some("audio/ogg".to_lowercase()) || !file.filename.ends_with(".ogg") {
                    comp.edit_original_interaction_response(&ctx.http, |response| {
                        response.content("Sorry thats not a voice message!")
                    })
                    .await?;
                    return Ok(());
                }
                if file.size > 2097152 {
                    comp.edit_original_interaction_response(&ctx.http, |response| {
                        response.content("Sorry file too big!")
                    })
                    .await?;
                    return Ok(());
                }

                let fname = format!("out/{}-{}", msg.id, file.filename);
                let out = fname.replace(".ogg", ".wav");

                //These functions do fs stuff and cant
                fetch_url(file.url.clone(), fname.clone()).await?;
                transcode_video(&fname, &out).await?;

                let result = speech_to_text(&out).await?;

                let end = format!("{} {}", msg.link(), result.trim());

                comp.edit_original_interaction_response(&ctx.http, |response| response.content(end))
                    .await?;
                Ok(())
            }
            .await;
            if let Err(e) = result {
                println!("{e:?}");
            }
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|cmd| cmd.kind(CommandType::Message).name("Transcribe Message"));
            commands.create_application_command(|cmd| cmd.kind(CommandType::ChatInput).name("privacy"));
            commands.create_application_command(|cmd| cmd.kind(CommandType::ChatInput).name("terms"));
            commands.create_application_command(|cmd| cmd.kind(CommandType::ChatInput).name("invite"));
            commands.create_application_command(|cmd| cmd.kind(CommandType::ChatInput).name("help"))
        })
        .await
        .ok();
        println!("{} is connected!", ready.user.name);
    }
}

static WHISPER_CTX: Lazy<Mutex<WhisperContext>> =
    Lazy::new(|| Mutex::new(WhisperContext::new("./ggml-medium.bin").unwrap()));

async fn speech_to_text(file: &str) -> Result<String> {
    let mut ctx = WHISPER_CTX.lock().await;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
    params.set_translate(false);
    params.set_no_context(true);
    //logs
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    ctx.full(params, &wav_to_integer_mono(&PathBuf::from(file))?)
        .map_err(|x| anyhow!(format!("{x:?}")))?;

    let num_segments = ctx.full_n_segments();

    let res = (0..num_segments)
        .flat_map(|i| ctx.full_get_segment_text(i).map_err(|x| anyhow!(format!("{x:?}"))))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<()> {
    Lazy::force(&WHISPER_CTX);
    let mut client = Client::builder(std::env::var("TOKEN").unwrap(), GatewayIntents::empty())
        .event_handler(Handler)
        .await?;

    client.start().await?;
    Ok(())
}
