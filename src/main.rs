mod downloader;

use color_eyre::Result;
use downloader::download_song;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    if !std::path::Path::new("tmp").exists() {
        std::fs::create_dir("tmp")?;
    }
    download_song("HoBGWhapaho").await?;
    // download_song("I90KY3HNm0Y").await?;
    Ok(())
}
