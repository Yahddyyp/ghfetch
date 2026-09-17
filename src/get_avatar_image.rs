use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

pub async fn download_avatar(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = std::env::temp_dir().join("avatar.png");
    let mut file = tokio::fs::File::create(&path).await?;

    let response = reqwest::get(url).await?;
    let mut steam = response.bytes_stream();

    while let Some(chunk) = steam.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(path.to_string_lossy().to_string())
}
