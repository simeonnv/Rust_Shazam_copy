use crate::utils::db::init::{get_pool::get_pool, queries};

pub async fn init_tables() -> Result<(), sqlx::Error> {
    println!("init tables");

    let pool = get_pool();

    for query in queries::QUERIES.iter() {
        sqlx::query(query).execute(pool).await?;
    }

    println!("Database created or already exists!");

    Ok(())
}
