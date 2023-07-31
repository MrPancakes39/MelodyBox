mod downloader;
mod errors;
mod fetcher;
mod structure;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const FFMPEG_PATH: &str = "/usr/bin/ffmpeg";

use color_eyre::Result;
use downloader::download_song;
use fetcher::{get_lyrics_from_yt, get_track_info};

fn prechecks() -> Result<()> {
    use execute::Execute;
    use std::process::Command;

    if !std::path::Path::new("tmp").exists() {
        std::fs::create_dir("tmp")?;
    }
    let mut cmd = Command::new(FFMPEG_PATH);
    cmd.arg("-version");
    if cmd.execute_check_exit_status_code(0).is_err() {
        panic!(
            "The path `{}` is not a correct FFmpeg executable binary file. Do you even have ffmpeg installed?",
            FFMPEG_PATH
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    prechecks()?;

    let client = reqwest::Client::new();

    let info = get_track_info(&client, "z34enKCqRGk").await?;
    dbg!(&info);
    let lyrics = get_lyrics_from_yt(&client, &info).await?;
    dbg!(&lyrics);
    let path = download_song(&client, &info.video_id).await?;
    dbg!(&path);

    Ok(())
}
