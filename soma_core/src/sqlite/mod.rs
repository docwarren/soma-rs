pub mod genes;

use rusqlite::{Connection, Result};

pub fn establish_connection(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    Ok(conn)
}

