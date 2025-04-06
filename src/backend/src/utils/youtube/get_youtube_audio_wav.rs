use crate::error::Error;
use hound::{WavReader, WavSpec};
use rusty_ytdl::{FFmpegArgs, Video, VideoOptions, VideoSearchOptions};
use std::io::Cursor;
use tokio;

// Define your Wav struct
#[derive(Debug)]
pub struct Wav {
    pub channels: u16,
    pub sample_rate: u32,
    pub raw_bytes: Vec<u8>,
    pub duration: f64,
}

pub async fn get_youtube_audio_wav(url: &String) -> Result<Wav, Error> {
    // Step 1: Set up video options for audio-only
    let video_options = VideoOptions {
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    // Step 2: Create the Video object
    let video = Video::new_with_options(url, video_options).map_err(|e| {
        Error::BadRequest(format!(
            "Failed to create video object for URL {}: {}",
            url, e
        ))
    })?;

    let ffmpeg_args = FFmpegArgs {
        format: Some("wav".to_string()),
        audio_filter: Some("anull".to_string()),
        video_filter: None,
    };

    let stream = video
        .stream_with_ffmpeg(Some(ffmpeg_args))
        .await
        .map_err(|e| Error::BadRequest(format!("Failed to get stream for URL {}: {}", url, e)))?;

    let mut wav_bytes = Vec::new();
    while let Some(chunk) = stream.chunk().await.map_err(|e| {
        Error::BadRequest(format!(
            "Failed to read stream chunk for URL {}: {}",
            url, e
        ))
    })? {
        wav_bytes.extend_from_slice(&chunk);
    }

    if wav_bytes.len() < 44 {
        return Err(Error::BadRequest(
            "WAV data too short to contain a valid header".to_string(),
        ));
    }

    let cursor = Cursor::new(&wav_bytes);
    let reader = WavReader::new(cursor)
        .map_err(|e| Error::BadRequest(format!("Failed to parse WAV data: {}", e)))?;

    let spec: WavSpec = reader.spec();
    let duration = reader.duration() as f64 / spec.sample_rate as f64;

    let wav = Wav {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        raw_bytes: wav_bytes,
        duration,
    };

    Ok(wav)
}
