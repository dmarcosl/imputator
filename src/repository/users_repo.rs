use rusqlite::{params, Connection};

pub(crate) async fn insert_user(
    conn: &mut Connection,
    username: &str,
    jirauser: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO users (username, jirauser) VALUES (?1, ?2)",
        params![username, jirauser],
    )?;
    Ok(())
}

pub(crate) async fn get_jirauser(
    conn: &mut Connection,
    username: &str,
) -> rusqlite::Result<(bool, String)> {
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
