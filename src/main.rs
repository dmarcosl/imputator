mod api_module;
mod csv_module;
mod sqlite_module;

use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

const COMPANY: &str = "domain";

const LOGIN_URL: &str = "https://jira.{}.com/rest/auth/1/session";
const USER_URL: &str = "https://jira.{}.com/rest/api/latest/user?username={}";
const ISSUE_URL: &str = "https://jira.{}.com/rest/com.yasoon/1.0/view/issue/{}";
const TEMPO_URL: &str = "https://jira.{}.com/rest/tempo-timesheets/4/worklogs";

const IMPUTATIONS_FILE: &str = "csv/imputations.csv";
const CREDENTIALS_FILE: &str = "csv/credentials.csv";

const DB_FILE: &str = "db/imputations.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let imputations = csv_module::read_imputations().await?;
    let credentials = csv_module::read_credentials().await?;

    let mut index = 0;
    for imputation in imputations {
        println!(
            "Procesando imputación {} para {} sobre tarea {}",
            index, imputation.user, imputation.issue
        );

        let client = Client::builder().cookie_store(true).build()?;

        let seconds = time_to_seconds(&imputation.time);

        let user_credentials = credentials
            .iter()
            .find(|credential| credential.user == imputation.user);

        if user_credentials.is_none() {
            println!("User {} not found in credentials file", imputation.user);
            continue;
        }

        // Login, true ok, false ko
        if api_module::login_and_get_cookies(
            &client,
            &user_credentials.unwrap().user,
            &user_credentials.unwrap().pass,
        )
        .await?
        {
            let jira_user = get_jira_user(&imputation.user, &client).await?;
            let issue_id = get_issue_id(&imputation.issue, &client).await?;

            api_module::post_to_tempo(
                &client,
                jira_user,
                seconds,
                imputation.description,
                issue_id,
                imputation.day,
            )
            .await?;
        } else {
            println!("Login failed");
        }

        csv_module::remove_imputation(index).await?;
        index += 1;
        print!("Imputación procesada\n");
    }

    Ok(())
}

async fn get_jira_user(user: &str, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    // If the user is not found in the database, request it from Jira
    let user_optional = sqlite_module::get_jirauser(user).await?;
    if user_optional.0 == false {
        let user_response = api_module::get_jira_user(client, user).await?;
        sqlite_module::insert_user(user, &user_response.key).await?;
        Ok(user_response.key)
    } else {
        Ok(user_optional.1)
    }
}

async fn get_issue_id(issue: &str, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    // If the issue is not found in the database, request it from Jira
    let issue_optional = sqlite_module::get_issue_id(issue).await?;
    if issue_optional.0 == false {
        let issue_response = api_module::get_issue(client, issue).await?;
        sqlite_module::insert_issue(issue, &issue_response.issueId).await?;
        Ok(issue_response.issueId)
    } else {
        Ok(issue_optional.1)
    }
}

fn time_to_seconds(time: &str) -> u32 {
    let mut seconds = 0;

    for fragment in time.split_whitespace() {
        if let Some(hours) = fragment.strip_suffix("h") {
            if let Ok(hours_int) = u32::from_str(hours) {
                seconds += hours_int * 3600;
            }
        } else if let Some(minutes) = fragment.strip_suffix("m") {
            if let Ok(minutes_int) = u32::from_str(minutes) {
                seconds += minutes_int * 60;
            }
        }
    }

    seconds
}
