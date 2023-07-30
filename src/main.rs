// mod downloader;
mod errors;
mod fetcher;
mod structure;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const FFMPEG_PATH: &str = "/usr/bin/ffmpeg";

use color_eyre::Result;
use fetcher::{get_lyrics_from_yt, get_track_info};

// use downloader::download_song;
// use fetcher::get_lyrics_from_yt;

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
    // download_song("HoBGWhapaho").await?;
    // download_song("I90KY3HNm0Y").await?;
    // dbg!(&title, &file);
    // get_lyrics_from_yt("HoBGWhapaho").await?;
    // get_lyrics_from_yt("I90KY3HNm0Y").await?;
    // let lyrics = get_lyrics_from_yt("z34enKCqRGk").await?;
    // dbg!(lyrics);
    // download_song("z34enKCqRGk").await?;
    let info = get_track_info("I90KY3HNm0Y").await?;
    dbg!(&info);
    let lyrics = get_lyrics_from_yt(&info).await?;
    dbg!(&lyrics);
    // dbg!(test_string("6".to_string()));
    // dbg!(parse_year("2023".to_string()));
    // dbg!(parse_duration("1:03:30".to_string()));
    Ok(())
}
