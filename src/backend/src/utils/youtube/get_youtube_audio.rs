use crate::error::Error;
use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};

pub async fn get_youtube_audio(url: String) -> Result<Vec<u8>, Error> {
    let video_options = VideoOptions {
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let video = Video::new_with_options(&url, video_options).unwrap();
    let stream = video.stream().await.unwrap();
    let mut audio_data = Vec::new();
    while let Some(chunk) = stream.chunk().await.unwrap() {
        audio_data.extend_from_slice(&chunk);
    }

    Ok(audio_data)
}
