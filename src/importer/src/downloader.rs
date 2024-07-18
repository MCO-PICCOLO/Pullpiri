use std::{
    fs::File,
    io::{copy, BufWriter},
};

use reqwest;

pub async fn download(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 요청을 보내고 응답을 받음
    let response = reqwest::get(url).await?;

    // 응답이 성공적인지 확인
    if response.status().is_success() {
        // 파일을 로컬에 저장
        let mut file = BufWriter::new(File::create(path)?);

        // 응답의 바디를 파일에 복사
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut file)?;

        println!("File downloaded to {}", path);
    } else {
        println!("Failed to download file: {}", response.status());
    }

    Ok(())
}
