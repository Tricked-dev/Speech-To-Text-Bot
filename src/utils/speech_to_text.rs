use std::path::PathBuf;

use autometrics::autometrics;
use serenity::prelude::*;

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use tracing::debug;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

use crate::utils::wav_to_integer_mono;

pub static WHISPER_CTX: Lazy<Mutex<WhisperContext>> =
    Lazy::new(|| Mutex::new(WhisperContext::new("./ggml-medium.bin").unwrap()));

#[autometrics]
pub async fn speech_to_text(file: String) -> Result<String> {
    let mut ctx = WHISPER_CTX.lock().await;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
    params.set_translate(false);
    params.set_no_context(true);
    //logs
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    let file_path = PathBuf::from(file.clone());
    let res = tokio::task::spawn_blocking(move || -> Result<String> {
        ctx.full(params, &wav_to_integer_mono(&file_path)?)
            .map_err(|x| anyhow!(format!("{x:?}")))?;

        let num_segments = ctx.full_n_segments();
        debug!("parsed: {file} segments: {num_segments}");
        let res = (0..num_segments)
            .flat_map(|i| ctx.full_get_segment_text(i).map_err(|x| anyhow!(format!("{x:?}"))))
            .collect::<Vec<String>>()
            .join("\n");
        Ok(res)
    })
    .await??;

    Ok(res)
}
