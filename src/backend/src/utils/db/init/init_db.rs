use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

const DB_URL: &'static str = "sqlite://ODM.db";

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(25)
        .connect(DB_URL)
        .await?;
    println!("Connected to SQLite database at {}", DB_URL);
    Ok(pool)
}
