use num_complex::Complex;

#[derive(Debug)]
pub struct Peak {
    time: f64,
    freq: Complex<f64>,
}

pub fn extract_peaks_from_spectrogram(
    spectrogram: Vec<Vec<Complex<f64>>>,
    audio_duration: f64,
) -> Vec<Peak> {
    if spectrogram.is_empty() {
        return Vec::new();
    }

    // Define the maxies struct
    #[derive(Debug)]
    struct Maxies {
        max_mag: f64,
        max_freq: Complex<f64>,
        freq_idx: usize,
    }

    // Define frequency bands
    let bands = vec![(0, 10), (10, 20), (20, 40), (40, 80), (80, 160), (160, 512)];

    let mut peaks = Vec::new();
    let bin_duration = audio_duration / spectrogram.len() as f64;

    for (bin_idx, bin) in spectrogram.iter().enumerate() {
        let mut max_mags = Vec::new();
        let mut max_freqs = Vec::new();
        let mut freq_indices = Vec::new();

        let mut bin_band_maxies = Vec::new();

        for (min, max) in &bands {
            let mut maxx = Maxies {
                max_mag: 0.0,
                max_freq: Complex::<f64>::new(0.0, 0.0),
                freq_idx: 0,
            };
            let mut max_mag = 0.0;

            for (idx, &freq) in bin[*min..*max].iter().enumerate() {
                let magnitude = freq.norm();
                if magnitude > max_mag {
                    max_mag = magnitude;
                    let freq_idx = min + idx;
                    maxx = Maxies {
                        max_mag: magnitude,
                        max_freq: freq,
                        freq_idx,
                    };
                }
            }
            bin_band_maxies.push(maxx);
        }

        for value in &bin_band_maxies {
            max_mags.push(value.max_mag);
            max_freqs.push(value.max_freq);
            freq_indices.push(value.freq_idx as f64);
        }

        let max_mags_sum: f64 = max_mags.iter().sum();
        let avg = max_mags_sum / max_freqs.len() as f64;

        for (i, &value) in max_mags.iter().enumerate() {
            if value > avg {
                let peak_time_in_bin = freq_indices[i] * bin_duration / bin.len() as f64;
                let peak_time = bin_idx as f64 * bin_duration + peak_time_in_bin;

                peaks.push(Peak {
                    time: peak_time,
                    freq: max_freqs[i],
                });
            }
        }
    }

    peaks
}
