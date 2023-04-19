//credit m1guelpf https://github.com/m1guelpf/whisper-cli-rs/blob/main/src/ffmpeg_decoder.rs

use anyhow::{anyhow, Result};
use std::env::temp_dir;
use std::fs::File;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

// ffmpeg -i input.mp3 -ar 16000 output.wav
async fn use_ffmpeg<P: AsRef<Path>>(input_path: P) -> Result<Vec<i16>> {
    let temp_file = temp_dir().join(format!("{}.wav", uuid::Uuid::new_v4()));
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            input_path.as_ref().to_str().ok_or_else(|| anyhow!("invalid path"))?,
            "-ar",
            "16000",
            "-ac",
            "1",
            "-c:a",
            "pcm_s16le",
            (temp_file.to_str().unwrap()),
            "-hide_banner",
            "-y",
            "-loglevel",
            "error",
        ])
        .stdin(Stdio::null())
        .status()
        .await?;

    if status.success() {
        let output = File::open(&temp_file)?;
        let mut reader = hound::WavReader::new(output)?;
        let samples: Result<Vec<i16>, _> = reader.samples().collect();
        std::fs::remove_file(temp_file)?;
        samples.map_err(std::convert::Into::into)
    } else {
        Err(anyhow!("unable to convert file"))
    }
}

pub async fn read_file<P: AsRef<Path>>(audio_file_path: P) -> Result<Vec<f32>> {
    let audio_buf = use_ffmpeg(&audio_file_path).await?;
    Ok(whisper_rs::convert_integer_to_float_audio(&audio_buf))
}
