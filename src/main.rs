mod ytmusic;

const FFMPEG_PATH: &str = "/usr/bin/ffmpeg";

use color_eyre::Result;
use ytmusic::YTMusic;

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

    let yt = YTMusic::new();
    let info = yt.get_track_info("FJX0JPXD2nM").await?;
    dbg!(&info);
    let lyrics = yt.get_lyrics(&info).await?;
    dbg!(&lyrics);
    let path = yt.download_song(&info.video_id).await?;
    println!("Path: {path:#}");

    Ok(())
}
