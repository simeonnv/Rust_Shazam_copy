use super::init::get_pool::get_pool;
use crate::error::Error;
use std::collections::HashMap;

pub async fn store_fingerprints_to_db(fingerprints: HashMap<u32, (u32, u32)>) -> Result<(), Error> {
    let pool = get_pool();
    let mut handles = Vec::new();

    for (address, (anchor_time_ms, song_id)) in fingerprints {
        let pool_ref = pool.clone(); // Assuming pool is a connection pool that can be cloned
        let handle = tokio::spawn(async move {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO fingerprints (address, anchor_time_ms, song_id)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(address)
            .bind(anchor_time_ms)
            .bind(song_id)
            .execute(&pool_ref)
            .await
            .map_err(Error::from)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    Ok(())
}
