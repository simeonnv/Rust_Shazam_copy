use super::{
    convert_yt_audio_to_wav::Wav, create_fingerprints_from_peaks::create_fingerprints_from_peaks,
    create_spectrogram_from_samples::create_spectrogram_from_samples,
    extract_peaks_from_spectrogram::extract_peaks_from_spectrogram,
    sample_wav_audio::sample_wav_audio,
};
use crate::{
    error::Error,
    utils::db::{
        register_song_to_db::register_song_to_db,
        store_fingerprints_to_db::store_fingerprints_to_db,
    },
};

pub async fn proccess_wav_into_db(
    wav: &Wav,
    song_title: String,
    song_artist: String,
    yt_id: String,
) -> Result<(), Error> {
    let samples = sample_wav_audio(wav)?;
    let spectro = create_spectrogram_from_samples(samples, wav).await?;

    let song_id = register_song_to_db(song_title, song_artist, yt_id).await?;

    let peaks = extract_peaks_from_spectrogram(spectro, wav);
    let fingerprints = create_fingerprints_from_peaks(peaks, song_id);

    store_fingerprints_to_db(fingerprints).await?;

    Ok(())
}
