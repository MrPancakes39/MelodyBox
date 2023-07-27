mod downloader;

const FFMPEG_PATH: &str = "/usr/bin/ffmpeg";

use color_eyre::Result;
use downloader::download_song;
use execute::Execute;
use std::process::Command;

fn prechecks() -> Result<()> {
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
    download_song("HoBGWhapaho").await?;
    // download_song("I90KY3HNm0Y").await?;
    Ok(())
}
