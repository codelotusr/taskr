use rusqlite::Connection;

pub fn init_db(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE task (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        due_date TEXT,
        completed INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}
