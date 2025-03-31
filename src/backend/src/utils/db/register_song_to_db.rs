use super::init::get_pool::get_pool;
use crate::error::Error;

pub async fn register_song_to_db(
    title: String,
    artist: String,
    yt_id: String,
) -> Result<u32, Error> {
    let pool = get_pool();

    let id: Option<u32> = sqlx::query_scalar(
        r#"
            INSERT INTO songs (title, artist, yt_id)
            VALUES ($1, $2, $3)
            RETURNING id;
        "#,
    )
    .bind(title)
    .bind(artist)
    .bind(yt_id)
    .fetch_one(pool)
    .await?;

    match id {
        Some(e) => Ok(e),
        None => Err(Error::Conflict("failed to register song".to_string())),
    }
}
