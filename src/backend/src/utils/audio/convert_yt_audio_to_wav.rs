use crate::error::Error;
use hound::{WavSpec, WavWriter};
use std::io::Cursor;
use symphonia::core::{
    audio::Signal, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions,
    probe::Hint,
};

#[derive(Debug)]
pub struct Wav {
    pub channels: u16,
    pub sample_rate: u32,
    pub raw_bytes: Vec<u8>,
    pub duration: f64,
}

pub async fn convert_yt_audio_to_wav(yt_audio: Vec<u8>) -> Result<Wav, Error> {
    let src = Cursor::new(yt_audio);
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let hint = Hint::new();
    let probe = symphonia::default::get_probe();
    let probed = probe
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .unwrap();
    let mut format = probed.format;
    let track = format.default_track().unwrap();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &Default::default())
        .unwrap();

    let sample_rate = track.codec_params.sample_rate.unwrap();
    let channels = track.codec_params.channels.unwrap().count() as u16;

    let mut wav_data = Vec::new();
    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::new(Cursor::new(&mut wav_data), spec).unwrap();

    let mut total_samples = 0;

    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet).unwrap();
        match decoded {
            symphonia::core::audio::AudioBufferRef::S16(buffer) => {
                for frame in 0..buffer.frames() {
                    for chan in 0..channels {
                        let sample = buffer.chan(chan as usize)[frame];
                        writer.write_sample(sample).unwrap();
                    }
                }
                total_samples += buffer.frames();
            }
            _ => {}
        }
    }
    writer.finalize().unwrap();

    // Calculate duration in seconds
    let duration = total_samples as f64 / sample_rate as f64;

    Ok(Wav {
        channels,
        sample_rate,
        raw_bytes: wav_data,
        duration,
    })
}
