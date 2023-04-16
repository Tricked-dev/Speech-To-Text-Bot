use std::{fs, process::Command};

#[path = "../model_data.rs"]
mod data;

use data::*;

fn main() -> anyhow::Result<()> {
    if fs::read_dir(WHISPER_FOLDER).is_err() {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/ggerganov/whisper.cpp")
            .status()?;
    }

    if fs::read("ggml-medium.bin").is_err() {
        let file = format!("{WHISPER_FOLDER}/models/ggml-{MODEL_NAME}.bin");
        if fs::read(&file).is_err() {
            Command::new("bash")
                .arg("./models/download-ggml-model.sh")
                .arg(MODEL_NAME)
                .current_dir(WHISPER_FOLDER)
                .status()?;
        }

        std::fs::rename(file, format!("./ggml-{MODEL_NAME}.bin"))?;
    }

    Ok(())
}
