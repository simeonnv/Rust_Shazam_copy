use super::init::get_pool::get_pool;
use crate::{error::Error, utils::youtube::get_youtube_info::YtInfo};

pub async fn register_song_to_db(yt_info: &YtInfo) -> Result<u32, Error> {
    let pool = get_pool();

    let id: Option<u32> = sqlx::query_scalar(
        r#"
            INSERT INTO songs (title, artist, yt_id)
            VALUES ($1, $2, $3)
            RETURNING id;
        "#,
    )
    .bind(yt_info.title.clone())
    .bind(yt_info.artist.clone())
    .bind(yt_info.id.clone())
    .fetch_one(pool)
    .await?;

    match id {
        Some(e) => Ok(e),
        None => Err(Error::Conflict("failed to register song".to_string())),
    }
}
