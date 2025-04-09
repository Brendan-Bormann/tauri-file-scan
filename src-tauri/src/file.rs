use rusqlite::params;
use rusqlite::Connection;
use std::fs;
use std::io::Read;
use walkdir::WalkDir;

use crate::db;

pub fn index_files(conn: &Connection, directory: &str) -> rusqlite::Result<()> {
    println!("┌ Indexing directory: [{}]\n│", directory);

    db::create_clean_file_table(conn);

    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        let path = entry.path().display().to_string();
        let name = entry.file_name();

        if entry.file_type().is_file() {
            println!("├┬ Scanning file: {:?}", name);

            let mut content = String::new();

            if let Ok(mut file) = fs::File::open(entry.path()) {
                match file.read_to_string(&mut content) {
                    Ok(_) => println!("│└ Successfully indexed!"),
                    Err(err) => println!("│└ Failed to index {:?}", err),
                }
            }

            conn.execute(
                "INSERT INTO files (path, content) VALUES (?1, ?2);",
                params![path, content],
            )
            .expect("failed to insert row");
        }
    }

    println!("│\n└ Directory scan complete!");

    Ok(())
}

pub fn search_files(conn: &Connection, query: &str) -> rusqlite::Result<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT path FROM files WHERE path MATCH ?1 or content MATCH ?1")?;
    let results = stmt.query_map(params![query], |row| row.get(0)).unwrap();
    let result_vec = results.collect::<Result<Vec<_>, _>>()?;

    println!(
        "Searched for \"{}\" and found {} records",
        query,
        result_vec.len()
    );

    Ok(result_vec)
}
