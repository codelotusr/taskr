use taskr::storage;
use rusqlite::{Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("taskrdb.sqlite")?;
    storage::init_db(&conn)?;

    Ok(())
}
