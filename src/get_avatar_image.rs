use futures_util::StreamExt;
use std::num::NonZeroU32;
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

pub fn print_avatar(path: &str) {
    let conf = viuer::Config {
        width: Some(20),
        height: Some(10),
        ..Default::default()
    };

    if viuer::print_from_file(path, &conf).is_err() {
        print_ascii_avatar(path);
    }
}

fn print_ascii_avatar(path: &str) {
    let img = match image::open(path) {
        Ok(img) => img,
        Err(_) => {
            println!("[could not load avatar]");
            return;
        }
    };

    let config = artem::config::ConfigBuilder::new()
        .target_size(NonZeroU32::new(30).unwrap())
        .build();

    let ascii = artem::convert(img, &config);
    println!("{}", ascii);
}
