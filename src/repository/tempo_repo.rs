use crate::domain::imputation::Imputation;
use rusqlite::{params, Connection};

pub(crate) async fn insert_work(
    conn: &mut Connection,
    tempo_id: &i64,
    username: &str,
    day: &str,
    issue: &str,
    description: &str,
    work_time: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO tempo (tempo_id, username, day, issue, description, work_time) \
              VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![tempo_id, username, day, issue, description, work_time],
    )?;
    Ok(())
}

pub(crate) async fn update_work(
    conn: &mut Connection,
    tempo_id: &i64,
    day: &str,
    issue: &str,
    description: &str,
    work_time: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE tempo SET day = ?2, issue = ?3, description = ?4, work_time = ?5 WHERE tempo_id = ?1",
        params![tempo_id, day, issue, description, work_time],
    )?;
    Ok(())
}

pub(crate) async fn get_work(
    conn: &mut Connection,
    tempo_id: &i64,
) -> rusqlite::Result<(bool, Imputation)> {
    let mut stmt = conn.prepare("SELECT tempo_id, username, day, issue, description, work_time FROM tempo WHERE tempo_id = ?1")?;
    let work_iter = stmt
        .query_map([tempo_id], |row| {
            Ok(Imputation {
                tempo_id: row.get(0)?,
                user: row.get(1)?,
                day: row.get(2)?,
                issue: row.get(3)?,
                description: row.get(4)?,
                time: row.get(5)?,
            })
        })?;

    for work in work_iter {
        return match work {
            Ok(work) => Ok((true, work)),
            Err(_) => Ok((false, Imputation::default())),
        }
    }

    Ok((false, Imputation::default()))
}
