use id3::{frame, Tag, TagLike, Version};
use std::path::Path;

pub struct TagInfo {
    title: String,
    artist: String,
    album: Option<String>,
    lyrics: Option<(String, String)>,
    cover: Option<Vec<u8>>,
    comment: Option<String>,
}

pub fn create_tag_info(
    title: impl Into<String>,
    artist: impl Into<String>,
    album: Option<impl Into<String>>,
    lyrics: Option<(impl Into<String>, impl Into<String>)>,
    cover: Option<impl AsRef<Path>>,
    comment: Option<impl Into<String>>,
) -> Result<TagInfo, std::io::Error> {
    Ok(TagInfo {
        title: title.into(),
        artist: artist.into(),
        album: album.map(|s| s.into()),
        lyrics: lyrics.map(|(s1, s2)| (s1.into(), s2.into())),
        cover: cover.map(|path| std::fs::read(path).ok()).flatten(),
        comment: comment.map(|s| s.into()),
    })
}

pub fn write_tags(path: impl AsRef<Path>, info: TagInfo) -> id3::Result<()> {
    let mut tag = Tag::new();
    tag.set_title(info.title);
    tag.set_artist(info.artist);
    if let Some(album) = info.album {
        tag.set_album(album);
    }
    if let Some(lyrics) = info.lyrics {
        tag.add_frame(frame::Lyrics {
            lang: lyrics.0,
            description: String::new(),
            text: lyrics.1,
        });
    }
    if let Some(cover) = info.cover {
        tag.add_frame(frame::Picture {
            mime_type: String::from("image/jpeg"),
            picture_type: id3::frame::PictureType::CoverFront,
            description: String::new(),
            data: cover,
        });
    }
    if let Some(comment) = info.comment {
        tag.add_frame(frame::Comment {
            lang: String::from("eng"),
            description: String::new(),
            text: comment,
        });
    }
    tag.write_to_path(path, Version::Id3v24)
}
