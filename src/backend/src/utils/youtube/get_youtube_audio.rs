use crate::error::Error; // Ensure this is your custom error type
use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};

// Optional: If you want logging
#[cfg(feature = "logging")]
use log::error;

pub async fn get_youtube_audio(url: &String) -> Result<Vec<u8>, Error> {
    let video_options = VideoOptions {
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    // Create the Video object
    let video = Video::new_with_options(url, video_options).map_err(|e| {
        #[cfg(feature = "logging")]
        error!("Failed to create video object for URL {}: {:?}", url, e);
        Error::BadRequest(format!(
            "Failed to create video object for URL {}: {}",
            url, e
        ))
    })?;

    // Get the stream
    let stream = video.stream().await.map_err(|e| {
        #[cfg(feature = "logging")]
        error!("Failed to get stream for URL {}: {:?}", url, e);
        Error::BadRequest(format!("Failed to get stream for URL {}: {}", url, e))
    })?;

    // Collect audio data
    let mut audio_data = Vec::new();
    while let Some(chunk) = stream.chunk().await.map_err(|e| {
        #[cfg(feature = "logging")]
        error!("Failed to read stream chunk for URL {}: {:?}", url, e);
        Error::BadRequest(format!(
            "Failed to read stream chunk for URL {}: {}",
            url, e
        ))
    })? {
        audio_data.extend_from_slice(&chunk);
    }

    Ok(audio_data)
}
