// mod downloader;
mod errors;
mod fetcher;
mod structure;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0";
const FFMPEG_PATH: &str = "/usr/bin/ffmpeg";

use color_eyre::Result;
use fetcher::{get_context, get_track_info};
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

fn parse_year(s: String) -> Option<i32> {
    if s.len() == 4 {
        s.parse::<i32>().ok()
    } else {
        None
    }
}

fn parse_duration(s: String) -> Option<i32> {
    if !s.contains(':') {
        return None;
    }
    let tmp = s
        .split(':')
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .ok()?;
    if tmp.len() > 3 {
        return None;
    }
    let duration = tmp
        .iter()
        .rev()
        .zip([1, 60, 3600])
        .map(|(m, n)| m * n)
        .sum::<i32>();
    Some(duration)
}

fn test_string(s: String) -> Option<String> {
    // view pattern = number alphanum ... space ... alphanum
    if s.len() > 3
        && s.chars().next().unwrap().is_numeric()
        && s.chars().filter(|c| c == &' ').count() == 1
    {
        Some(s)
    } else {
        None
    }
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
    // get_track_info("HoBGWhapaho", &get_context()).await?;
    dbg!(test_string("6".to_string()));
    // dbg!(parse_year("2023".to_string()));
    // dbg!(parse_duration("1:03:30".to_string()));
    Ok(())
}
