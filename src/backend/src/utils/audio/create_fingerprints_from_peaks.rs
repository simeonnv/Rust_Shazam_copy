use super::extract_peaks_from_spectrogram::Peak;
use std::collections::HashMap;

const TARGET_ZONE_SIZE: usize = 10;
//a fingerprint consists of a address that points to time and a id
pub fn create_fingerprints_from_peaks(peaks: Vec<Peak>, song_id: u32) -> HashMap<u32, (u32, u32)> {
    let mut fingerprints = HashMap::new();

    for (i, anchor) in peaks.iter().enumerate() {
        let j_limit = std::cmp::min(i + TARGET_ZONE_SIZE + 1, peaks.len());
        for j in (i + 1)..j_limit {
            let target = &peaks[j];

            let address = create_address(anchor, target);
            let anchor_time_ms = (anchor.time * 1000.0) as u32;

            fingerprints.insert(address, (anchor_time_ms, song_id));
        }
    }

    fingerprints
}

fn create_address(anchor: &Peak, target: &Peak) -> u32 {
    let anchor_freq = anchor.freq.re as i32;
    let target_freq = target.freq.re as i32;
    let delta_ms = ((target.time - anchor.time) * 1000.0) as u32;

    let address = ((anchor_freq as u32) << 23) | ((target_freq as u32) << 14) | delta_ms;

    address
}
