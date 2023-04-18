use autometrics::autometrics;
use serenity::prelude::*;

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use tracing::debug;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

use crate::model_data::MODEL_NAME;

pub static WHISPER_CTX: Lazy<Mutex<WhisperContext>> =
    Lazy::new(|| Mutex::new(WhisperContext::new(&format!("./ggml-{MODEL_NAME}.bin")).unwrap()));

#[autometrics]
pub async fn speech_to_text(data: Vec<f32>, lang: String) -> Result<String> {
    let mut ctx = WHISPER_CTX.lock().await;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
    let lang = match lang.as_ref() {
        "id" => "id",
        "de" => "de",
        "da" => "da",
        "en-GB" | "en-US" | "es-ES" => "en",
        "fr" => "fr",
        "hr" => "hr",
        "it" => "it",
        "lt" => "lt",
        "hu" => "hu",
        "nl" => "nl",
        "no" => "no",
        "pl" => "pl",
        "pt-BR" => "pt",
        "ro" => "ro",
        "fi" => "fi",
        "sv-SE" => "sv",
        "vi" => "vi",
        "tr" => "tr",
        "cs" => "cs",
        "el" => "el",
        "bg" => "bg",
        "ru" => "ru",
        "uk" => "uk",
        "hi" => "hi",
        "th" => "th",
        "zh-TW" | "zh-CN" => "zh",
        "ja" => "ja",
        "ko" => "ko",
        _ => "auto",
    };

    params.set_language(Some(lang));
    params.set_translate(false);
    params.set_no_context(true);
    //logs
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    let res = tokio::task::spawn_blocking(move || -> Result<String> {
        ctx.full(params, &data).map_err(|x| anyhow!(format!("{x:?}")))?;

        let num_segments = ctx.full_n_segments();
        debug!("segments: {num_segments}");
        let res = (0..num_segments)
            .flat_map(|i| ctx.full_get_segment_text(i).map_err(|x| anyhow!(format!("{x:?}"))))
            .collect::<Vec<String>>()
            .join("\n");
        Ok(res)
    })
    .await??;

    Ok(res)
}
