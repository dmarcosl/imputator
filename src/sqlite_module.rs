use crate::DB_FILE;
use rusqlite::{params, Connection, Result};

async fn connect() -> Result<Connection> {
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

    Ok(conn)
}

pub(crate) async fn insert_user(username: &str, jirauser: &str) -> Result<()> {
    let conn = connect().await?;
    conn.execute(
        "INSERT INTO users (username, jirauser) VALUES (?1, ?2)",
        params![username, jirauser],
    )?;
    Ok(())
}

pub(crate) async fn get_jirauser(username: &str) -> Result<(bool, String)> {
    let conn = connect().await?;

    let mut stmt = conn.prepare("SELECT jirauser FROM users WHERE username = ?1")?;
    let user_iter = stmt.query_map([username], |row| row.get(0))?;

    for jirauser in user_iter {
        match jirauser {
            Ok(user) => return Ok((true, user)),
            Err(_) => return Ok((false, String::new())),
        }
    }

    Ok((false, String::new()))
}

pub(crate) async fn insert_issue(issue: &str, issue_id: &str) -> Result<()> {
    let conn = connect().await?;
    conn.execute(
        "INSERT INTO issues (issue, issue_id) VALUES (?1, ?2)",
        params![issue, issue_id],
    )?;
    Ok(())
}

pub(crate) async fn get_issue_id(issue: &str) -> Result<(bool, String)> {
    let conn = connect().await?;

    let mut stmt = conn.prepare("SELECT issue_id FROM issues WHERE issue = ?1")?;
    let issue_iter = stmt.query_map([issue], |row| row.get(0))?;

    for issue_id in issue_iter {
        match issue_id {
            Ok(id) => return Ok((true, id)),
            Err(_) => return Ok((false, String::new())),
        }
    }

    Ok((false, String::new()))
}
