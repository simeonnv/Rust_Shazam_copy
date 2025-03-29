use crate::error::Error;

pub fn reduce_audio_sample_rate(
    samples: Vec<f64>,
    original_sample_rate: i32,
    target_sample_rate: i32,
) -> Result<Vec<f64>, Error> {
    if target_sample_rate <= 0 || original_sample_rate <= 0 {
        return Err(Error::Internal(
            "Reduce sample rate: sample rates must be positive".to_string(),
        ));
    }
    if target_sample_rate > original_sample_rate {
        return Err(Error::Internal(
            "Reduce sample: ratetarget sample rate must be less than or equal to original sample rate".to_string(),
        ));
    }

    let ratio = original_sample_rate / target_sample_rate;
    if ratio <= 0 {
        return Err(Error::Internal(
            "Reduce sample rate: invalid ratio calculated from sample rates".to_string(),
        ));
    }

    let mut resampled = Vec::new();
    let mut i = 0;
    while i < samples.len() {
        let end = std::cmp::min(i + ratio as usize, samples.len());
        let mut sum = 0.0;
        for j in i..end {
            sum += samples[j];
        }
        let avg = sum / (end - i) as f64;
        resampled.push(avg);
        i = end;
    }

    Ok(resampled)
}
