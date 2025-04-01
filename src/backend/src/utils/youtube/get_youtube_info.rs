use crate::error::Error;
use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};

pub struct YtInfo {
    pub title: String,
    pub artist: String,
    pub id: String,
}

pub async fn get_youtube_info(url: &String) -> Result<YtInfo, Error> {
    let video_options = VideoOptions {
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let video = Video::new_with_options(url, video_options).unwrap();

    let info = video.get_info().await.unwrap();

    Ok(YtInfo {
        title: info.video_details.title,
        artist: info.video_details.owner_channel_name,
        id: info.video_details.video_id,
    })
}
