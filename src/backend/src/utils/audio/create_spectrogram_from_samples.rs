use super::{
    fast_fourier_transform::fast_fourier_transform, filter_samples::filter_samples,
    reduce_audio_sample_rate::reduce_audio_sample_rate,
};
use crate::{error::Error, utils::youtube::get_youtube_audio_wav::Wav};
use num_complex::Complex;
const DSP_RATIO: usize = 4;
const FREQ_BIN_SIZE: usize = 1024;
const MAX_FREQ: f64 = 5000.0;
const HOP_SIZE: usize = FREQ_BIN_SIZE / 32;

pub async fn create_spectrogram_from_samples(
    samples: Vec<f64>,
    wav: &Wav,
) -> Result<Vec<Vec<Complex<f64>>>, Error> {
    let filtered_samples = filter_samples(MAX_FREQ, wav, samples);
    let reduced_samples = reduce_audio_sample_rate(
        filtered_samples,
        FREQ_BIN_SIZE as i32,
        (wav.sample_rate / 4) as i32,
    )?;

    let num_of_windows = reduced_samples.len() / (FREQ_BIN_SIZE - HOP_SIZE);
    let mut spectrogram = Vec::<Vec<Complex<f64>>>::new();
    let mut windows = Vec::<f64>::with_capacity(num_of_windows);

    for i in 0..windows.len() {
        windows[i] = 0.54
            - 0.46 * f64::cos(2.0 * std::f64::consts::PI * i as f64 / (FREQ_BIN_SIZE as f64 - 1.0));
    }

    for i in 0..num_of_windows {
        let start = i * HOP_SIZE;
        let end = start + FREQ_BIN_SIZE;
        let end = if end > reduced_samples.len() {
            reduced_samples.len()
        } else {
            end
        };

        //copy
        let mut bin = vec![0.0; FREQ_BIN_SIZE];
        for (j, &val) in reduced_samples[start..end].iter().enumerate() {
            bin[j] = val;
        }

        // Apply Hamming window
        for j in 0..windows.len() {
            bin[j] *= windows[j];
        }

        spectrogram[i] = fast_fourier_transform(bin);
    }

    Ok(spectrogram)
}
