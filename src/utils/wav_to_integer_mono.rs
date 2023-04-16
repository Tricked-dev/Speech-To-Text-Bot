use std::path::PathBuf;

use anyhow::{anyhow, Result};
use autometrics::autometrics;

#[autometrics]
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
