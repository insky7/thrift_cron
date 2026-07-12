use anyhow::Ok;
use chrono::Utc;
use std::env;
use stomp_rs::{client::ClientBuilder, protocol::frame::Send};

// tracing logger
pub fn logger() -> tracing_appender::non_blocking::WorkerGuard {
    //todo Match expected file location and file naming conventions like datadog uses/use datadog (not sure on this, might take some research)
    let file_appender = tracing_appender::rolling::daily(".", "prefix.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    guard
}
pub fn filter_reply_lines(message: &str) -> String {
    let mut body = String::new();
    for line in message.lines() {
        let trimmed = line.trim();
        if REGEX_EMAIL_STOP.iter().any(|re| re.is_match(trimmed)) {
            break;
        }
        if !trimmed.is_empty() {
            if !body.is_empty() {
                body.push('\n');
            }
            body.push_str(trimmed);
        }
    }
    body
}
// cannot coerce error type from stomp_rs to anyhow error type
pub async fn send_pusher_notification(record: String) -> Result<(), Box<dyn std::error::Error>> {
    let pusher_ip = std::env::var("PusherHost").expect("PusherHost env var not set");
    let pusher_port = std::env::var("PusherPort").expect("PusherPort env var not set");
    let client = stomp_rs::client::Client::connect(ClientBuilder::new(format!(
        "{}:{}",
        pusher_ip, pusher_port
    )))
    .await?;
    client.send(Send::new("/queue/pusher").body(record)).await
}
// init SQL pool connection
pub async fn sql_init() -> Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {
    let pool = sqlx::MySqlPool::connect(
        &env::var("DATABASE_URL")
            .expect("Error when attempting to connect to pool: No database URL in ENV"),
    )
    .await;
    pool
}

// format into the phone # format we expect
// take an option &str and convert it into a phone number format
pub fn format_phone_number(input: Option<&str>) -> Option<String> {
    let digits: String = input?.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() != 10 {
        return None;
    }

    Some(format!(
        "({}) {}-{}",
        &digits[0..3],
        &digits[3..6],
        &digits[6..10]
    ))
}

// pub fn handle_stop(NotifierReply) ->
pub fn is_reply_stop_line(trimmed: &str) -> bool {
    REGEX_EMAIL_STOP.iter().any(|re| re.is_match(trimmed))
}

// extracting email from email replies helper function
pub fn extract_email(addr: &str) -> Option<String> {
    addr.split('<')
        .nth(1)?
        .split('>')
        .next()
        .map(|s| s.trim().to_string())
}
pub fn get_message_hash(body: &str) -> String {
    format!("{:x}", md5::compute(body))
}

use crate::REGEX_EMAIL_STOP;
use std::collections::HashMap;

pub async fn add_phrase_to_cache_queue(
    phrase: &str,
    industry_id: Option<i32>,
    promise_id: Option<i32>,
    transcript_id: Option<i32>,
) -> anyhow::Result<String> {
    // retrieve the sentiment engine host from an environment variable.

    let se_host = std::env::var("SentimentsHost").expect("DATABASE_URL not set");

    let url = format!("http://{}/services/addcommenttocachequeue.py", se_host);

    // build the request parameters.
    let mut params = HashMap::new();
    params.insert("comment".to_string(), phrase.to_string());
    params.insert("dtCreated".to_string(), Utc::now().timestamp().to_string());

    if let Some(id) = industry_id {
        params.insert("iIndustryID".to_string(), id.to_string());
    }
    if let Some(id) = promise_id {
        params.insert("iPromiseID".to_string(), id.to_string());
    }
    if let Some(id) = transcript_id {
        params.insert("iPromiseTranscriptID".to_string(), id.to_string());
    }

    // create a blocking HTTP client with a 30-second timeout.
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client.post(&url).form(&params).send().await?;

    // read the response body as text and return it.
    let response_text = response.text().await?;

    Ok(response_text)
}

/// Parses repair authorization amounts from the metadata string of an event log.
///
/// - `total`: If true, sums all amounts found. If false, returns only the last amount.
pub fn extra_repair_auth_amount(meta_data: &str, total: bool) -> f64 {
    let amounts: Vec<f64> = meta_data
        .split('$')
        .filter_map(|s| s.split_whitespace().next())
        .filter_map(|s| s.trim_end_matches(',').parse::<f64>().ok())
        .collect();

    if total {
        amounts.iter().sum()
    } else {
        amounts.last().cloned().unwrap_or(0.0)
    }
}
pub async fn get_signature_insurance(
    pool: &sqlx::MySqlPool,
    company_id: i32,
    insurance_company_id: i32,
) -> anyhow::Result<Option<(i32, String, String, String)>> {
    let login_id_row = sqlx::query!(
        r#"
        SELECT LoginCompany.iLoginID
        FROM LoginCompany
        JOIN Login AS InsuranceLogin ON InsuranceLogin.iLoginID = LoginCompany.iLoginID
        WHERE LoginCompany.iLoginCompanyTypeID = ?
          AND InsuranceLogin.iCompanyID = ?
          AND LoginCompany.iCompanyID = ?
        "#,
        2,
        insurance_company_id,
        company_id
    )
    .fetch_optional(pool)
    .await?;

    let login_id = match login_id_row {
        Some(row) => row.iLoginID,
        None => return Ok(None),
    };

    let login_row = sqlx::query!(
        r#"
        SELECT sSignatureText, sSignatureShortText, sEmailAddress
        FROM Login
        WHERE iLoginID = ?
        "#,
        login_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(login) = login_row {
        Ok(Some((
            login_id,
            login.sSignatureText.unwrap_or_default(),
            login.sSignatureShortText.unwrap_or_default(),
            login.sEmailAddress.unwrap_or_default(),
        )))
    } else {
        Ok(None)
    }
}
