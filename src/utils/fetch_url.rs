use std::{io::Cursor, path::Path};

use anyhow::Result;
use autometrics::autometrics;

#[autometrics]
pub async fn fetch_url<P: AsRef<Path>>(url: String, file_name: P) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
