use crate::DB_FILE;
use rusqlite::{Connection, Result};

pub(crate) async fn connect() -> Result<Connection> {
    let conn = Connection::open(DB_FILE)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            jirauser TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS issues (
            issue TEXT PRIMARY KEY,
            issue_id TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tempo (
            tempo_id INTEGER NOT NULL PRIMARY KEY,
            username TEXT NOT NULL,
            day TEXT NOT NULL,
            issue TEXT NOT NULL,
            description TEXT NOT NULL,
            work_time TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}
