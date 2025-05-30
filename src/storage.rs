use rusqlite::{Connection, Error as SqliteError};

pub fn init_db(conn: &Connection) -> Result<(), SqliteError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        completed INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}
