use rusqlite::Connection;
use taskr::storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("taskrdb.sqlite")?;
    storage::init_db(&conn)?;

    Ok(())
}
