use rusqlite::{Connection, Result};

fn create_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_packets (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             pack_title TEXT NOT NULL,
             pack_content TEXT NOT NULL
         )",
        (),
    )?;
    Ok(())
}

fn insert_packet(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO clipboard_packets (pack_title, pack_content) VALUES (?1, ?2)",
        ("asdssd", "sdsdsdsdfsdf"),
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("subtacs.db")?;

    create_db(&conn)?;
    insert_packet(&conn)?;

    Ok(())
}
