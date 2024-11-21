use crate::{COMPANY, ISSUE_URL, LOGIN_URL, TEMPO_CREATE_URL, TEMPO_UPDATE_URL, USER_URL};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct UserResponse {
    key: String,  // JIRAUSER165902
    name: String, // e_dmarco
}

#[derive(Deserialize)]
struct IssueResponse {
    issueId: String,  // 3291889
    issueKey: String, // DEV-930
}

#[derive(Deserialize)]
struct TempoWorklogResponse {
    tempoWorklogId: i64, // 3291889
    originTaskId: i64,   // JIRAUSER165902
    updater: String,     // e_dmarco
}

#[derive(Serialize)]
struct TempoPayload {
    attributes: serde_json::Value,
    billableSeconds: u32,
    originId: i32,
    worker: String,
    comment: String,
    started: String,
    timeSpentSeconds: u32,
    originTaskId: String,
    remainingEstimate: u32,
    endDate: Option<String>,
    includeNonWorkingDays: bool,
}

#[derive(Serialize)]
struct TempoUpdatePayload {
    billableSeconds: u32,
    originId: i32,
    comment: String,
    started: String,
    timeSpentSeconds: u32,
    originTaskId: String,
    remainingEstimate: u32,
    endDate: Option<String>,
    includeNonWorkingDays: bool,
}

pub(crate) async fn login_and_get_cookies(
    client: &Client,
    username: &str,
    password: &str,
) -> Result<bool, reqwest::Error> {
    let payload = LoginPayload {
        username: username.to_string(),
        password: password.to_string(),
    };

    let url = LOGIN_URL.replacen("{}", COMPANY, 1);

    let response = client.post(url).json(&payload).send().await?;

    Ok(response.status().is_success())
}

pub(crate) async fn get_jira_user(
    client: &Client,
    user: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = USER_URL.replacen("{}", COMPANY, 1).replace("{}", user);

    let response = client.get(url).send().await?;

    if response.status().is_success() {
        if let Ok(login_response) = response.json::<UserResponse>().await {
            return Ok(login_response.key);
        };
    }

    Err("Failed to get user".to_string().into())
}

pub(crate) async fn get_issue(
    client: &Client,
    issue: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = ISSUE_URL.replacen("{}", COMPANY, 1).replace("{}", issue);

    let response = client.get(url).send().await?;

    if response.status().is_success() {
        if let Ok(issue_response) = response.json::<IssueResponse>().await {
            return Ok(issue_response.issueId);
        };
    }

    Err("Failed to get issue".to_string().into())
}

pub(crate) async fn create_tempo(
    client: &Client,
    jira_user: String,
    seconds: u32,
    description: &String,
    issue_id: String,
    day: &String,
) -> Result<i64, Box<dyn std::error::Error>> {
    let payload = TempoPayload {
        attributes: serde_json::json!({}),
        billableSeconds: seconds,
        originId: -1,
        worker: jira_user.to_string(),
        comment: description.to_string(),
        started: day.to_string(),
        timeSpentSeconds: seconds,
        originTaskId: issue_id,
        remainingEstimate: 0,
        endDate: None,
        includeNonWorkingDays: false,
    };
    let url = TEMPO_CREATE_URL.replacen("{}", COMPANY, 1);

    let response = client.post(url).json(&payload).send().await?;

    if response.status().is_success() {
        let worklogs: Vec<TempoWorklogResponse> = response.json().await?;

        if let Some(worklog) = worklogs.first() {
            Ok(worklog.tempoWorklogId)
        } else {
            Err("Response array is empty".to_string().into())
        }
    } else {
        Err("Failed to post to tempo".to_string().into())
    }
}

pub(crate) async fn update_tempo(
    client: &Client,
    tempo_id: i64,
    seconds: u32,
    description: &String,
    issue_id: String,
    day: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = TempoUpdatePayload {
        billableSeconds: seconds,
        originId: -1,
        comment: description.to_string(),
        started: format!("{}T00:00:00.000", day.to_string()),
        timeSpentSeconds: seconds,
        originTaskId: issue_id,
        remainingEstimate: 0,
        endDate: None,
        includeNonWorkingDays: false,
    };
    let url = TEMPO_UPDATE_URL.replacen("{}", COMPANY, 1).replace("{}", &*tempo_id.to_string());

    let response = client.put(url).json(&payload).send().await?;

    if response.status().is_success() {
        return Ok(());
    }

    Err(format!("Failed to update tempo: {} {}", response.status().to_string(), response.text().await?).to_string().into())
}
