use super::convert_yt_audio_to_wav::Wav;
use std::f64::consts::PI;

pub fn filter_samples(cut_off_freq: f64, wav: &Wav, samples: Vec<f64>) -> Vec<f64> {
    let rc = 1.0 / (2.0 * PI * cut_off_freq);
    let dt = 1.0 / wav.sample_rate as f64;
    let alpha = dt / (rc + dt);

    let mut filtered_samples: Vec<f64> = Vec::with_capacity(samples.len());
    let mut prev_output = 0.0;

    for (i, &sample) in samples.iter().enumerate() {
        if i == 0 {
            filtered_samples.push(sample * alpha);
        } else {
            filtered_samples.push(alpha * sample + (1.0 - alpha) * prev_output);
        }
        prev_output = filtered_samples[i];
    }

    filtered_samples
}
