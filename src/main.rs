mod api;
mod domain;
mod repository;
mod util;

use crate::domain::imputation::Imputation;
use reqwest::Client;
use rusqlite::Connection;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;

const COMPANY: &str = "domain";

const LOGIN_URL: &str = "https://jira.{}.com/rest/auth/1/session";
const USER_URL: &str = "https://jira.{}.com/rest/api/latest/user?username={}";
const ISSUE_URL: &str = "https://jira.{}.com/rest/com.yasoon/1.0/view/issue/{}";
const TEMPO_CREATE_URL: &str = "https://jira.{}.com/rest/tempo-timesheets/4/worklogs";
const TEMPO_UPDATE_URL: &str = "https://jira.{}.com/rest/tempo-timesheets/4/worklogs/{}";

const IMPUTATIONS_FILE: &str = "csv/imputations.csv";
const CREDENTIALS_FILE: &str = "csv/credentials.csv";

const DB_FILE: &str = "db/imputations.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the imputations from the CSV file and sort them by user
    let mut imputations = util::imputation_csv::read_imputations().await?;
    imputations.sort_by(|a, b| a.user.cmp(&b.user));
    // Read the credentials from the CSV file
    let credentials = util::credential_csv::read_credentials().await?;

    let mut conn = repository::connection::connect().await?;

    let mut current_user = String::new();
    let mut client: Client = Client::builder().cookie_store(true).build()?;

    let mut index = 0;
    for imputation in imputations {
        println!(
            "Processing imputation {} of {} about task {}",
            index, imputation.user, imputation.issue
        );

        let mut is_updated = false;

        // Check if the imputation is already imputed
        if imputation.tempo_id.is_some() {
            print!("  Imputation already registered\n");
            let db_imputation =
                repository::tempo_repo::get_work(&mut conn, &imputation.tempo_id.unwrap()).await?;
            if db_imputation.0 {
                // If it is already imputed without changes, skip it
                if imputation.compare(&db_imputation.1) {
                    print!("  No changes detected, it'll be skipped\n");
                    continue;
                }
                print!("  Changes detected, it'll be updated\n");
                is_updated = true;
            }
        } else {
            print!("  Imputation not registered, it'll be created\n");
        }

        if current_user != imputation.user {
            current_user = imputation.user.clone();
            client = Client::builder().cookie_store(true).build()?;
        }

        let seconds = util::time_util::time_to_seconds(&imputation.time);

        let user_credentials = credentials
            .iter()
            .find(|credential| credential.user == imputation.user);

        if user_credentials.is_none() {
            println!("  User {} not found in credentials file", imputation.user);
            continue;
        }

        // Login, true ok, false ko
        if api::jira_api::login_and_get_cookies(
            &client,
            &user_credentials.unwrap().user,
            &user_credentials.unwrap().pass,
        )
        .await?
        {
            let jira_user = get_jira_user(&mut conn, &imputation.user, &client).await?;
            let issue_id = get_issue_id(&mut conn, &imputation.issue, &client).await?;

            if is_updated {
                update_imputation(
                    &mut conn,
                    &mut client,
                    imputation.tempo_id.unwrap(),
                    &imputation,
                    seconds,
                    issue_id,
                )
                .await?;
            } else {
                let tempo_id = create_imputation(
                    &mut conn,
                    &mut client,
                    &imputation,
                    seconds,
                    jira_user,
                    issue_id,
                )
                .await?;
                util::imputation_csv::update_imputation(index, tempo_id).await?;
            }
        } else {
            println!("  Login failed");
        }

        index += 1;
        print!("  Imputation processed\n");
    }

    Ok(())
}

async fn create_imputation(
    mut conn: &mut Connection,
    client: &mut Client,
    imputation: &Imputation,
    seconds: u32,
    jira_user: String,
    issue_id: String,
) -> Result<i64, Box<dyn Error>> {
    let tempo_id = api::jira_api::create_tempo(
        &client,
        jira_user,
        seconds,
        &imputation.description,
        issue_id,
        &imputation.day,
    )
    .await?;

    repository::tempo_repo::insert_work(
        &mut conn,
        &tempo_id,
        &imputation.user,
        &imputation.day,
        &imputation.issue,
        &imputation.description,
        &imputation.time,
    )
    .await?;

    Ok(tempo_id)
}

async fn update_imputation(
    mut conn: &mut Connection,
    client: &mut Client,
    tempo_id: i64,
    imputation: &Imputation,
    seconds: u32,
    issue_id: String,
) -> Result<(), Box<dyn Error>> {
    api::jira_api::update_tempo(
        &client,
        tempo_id,
        seconds,
        &imputation.description,
        issue_id,
        &imputation.day,
    )
    .await?;

    repository::tempo_repo::update_work(
        &mut conn,
        &tempo_id,
        &imputation.day,
        &imputation.issue,
        &imputation.description,
        &imputation.time,
    )
    .await?;

    Ok(())
}

async fn get_jira_user(
    conn: &mut Connection,
    user: &str,
    client: &Client,
) -> Result<String, Box<dyn Error>> {
    // If the user is not found in the database, request it from Jira
    let user_optional = repository::users_repo::get_jirauser(conn, user).await?;
    if user_optional.0 == false {
        let jira_user = api::jira_api::get_jira_user(client, user).await?;
        repository::users_repo::insert_user(conn, user, &jira_user).await?;
        Ok(jira_user)
    } else {
        Ok(user_optional.1)
    }
}

async fn get_issue_id(
    conn: &mut Connection,
    issue: &str,
    client: &Client,
) -> Result<String, Box<dyn Error>> {
    // If the issue is not found in the database, request it from Jira
    let issue_optional = repository::issue_repo::get_issue_id(conn, issue).await?;
    if issue_optional.0 == false {
        let issue_id = api::jira_api::get_issue(client, issue).await?;
        repository::issue_repo::insert_issue(conn, issue, &issue_id).await?;
        Ok(issue_id)
    } else {
        Ok(issue_optional.1)
    }
}
