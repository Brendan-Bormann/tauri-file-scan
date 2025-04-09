use rusqlite::{Connection, Result};

pub fn initialize_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    create_clean_file_table(&conn);
    Ok(conn)
}

pub fn create_clean_file_table(conn: &Connection) {
    conn.execute("DROP TABLE IF EXISTS files", []).unwrap();
    conn.execute(
        "
        CREATE VIRTUAL TABLE files USING fts5(
            path,
            content
        );
        ",
        [],
    )
    .unwrap();
}
