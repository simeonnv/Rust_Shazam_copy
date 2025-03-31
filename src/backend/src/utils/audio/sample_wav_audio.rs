use super::convert_yt_audio_to_wav::Wav;
use crate::error::Error;

pub fn sample_wav_audio(wav_bytes: &Wav) -> Result<Vec<f64>, Error> {
    let wav_bytes = &wav_bytes.raw_bytes;

    if wav_bytes.len() % 2 != 0 {
        return Err(Error::Internal(
            "There was a error with sampling (the input wav is bad)".to_string(),
        ));
    }

    let num_samples = wav_bytes.len() / 2;
    let mut output = Vec::with_capacity(num_samples);

    for i in (0..wav_bytes.len()).step_by(2) {
        // Interpret bytes as a 16-bit signed integer (little-endian)
        let sample = i16::from_le_bytes([wav_bytes[i], wav_bytes[i + 1]]);

        // Scale the sample to the range [-1, 1]
        output.push(f64::from(sample) / 32768.0);
    }

    Ok(output)
}
