pub const QUERIES: [&str; 2] = [
    r#"
        CREATE TABLE IF NOT EXISTS Songs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            yt_id TEXT
        );
    "#,
    r#"
       CREATE TABLE IF NOT EXISTS Fingerprints (
           address INTEGER NOT NULL,
           anchor_time_ms INTEGER NOT NULL,
           song_id INTEGER NOT NULL,
           PRIMARY KEY (address, anchor_time_ms, song_id)
        );
    "#,
];

// INSERT INTO container_logs (docker_name, container_id, timestamp, error_message) VALUES (?, ?, ?, ?)
