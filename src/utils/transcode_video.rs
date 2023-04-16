use anyhow::Result;
use autometrics::autometrics;
use tokio::process::Command;

#[autometrics]
pub async fn transcode_video(nin: &str, out: &str) -> Result<()> {
    let _res = Command::new("ffmpeg")
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
