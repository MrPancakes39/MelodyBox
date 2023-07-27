mod downloader;

use color_eyre::Result;
use downloader::download_song;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    download_song("HoBGWhapaho").await?;
    // download_song("I90KY3HNm0Y").await?;
    Ok(())
}
