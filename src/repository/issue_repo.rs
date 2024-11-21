use rusqlite::{params, Connection};

pub(crate) async fn insert_issue(
    conn: &mut Connection,
    issue: &str,
    issue_id: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO issues (issue, issue_id) VALUES (?1, ?2)",
        params![issue, issue_id],
    )?;
    Ok(())
}

pub(crate) async fn get_issue_id(
    conn: &mut Connection,
    issue: &str,
) -> rusqlite::Result<(bool, String)> {
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
