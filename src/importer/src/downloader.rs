use std::{
    fs::File,
    io::{copy, BufWriter},
};

use reqwest;

pub async fn download(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let mut file = BufWriter::new(File::create(path)?);
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut file)?;
        println!("File downloaded to {}", path);
    } else {
        println!("Failed to download file: {}", response.status());
    }
    Ok(())
}
