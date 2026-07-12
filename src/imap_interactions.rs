use crate::{REGEX_PROMISE_ID, helper_functions};
use anyhow::Ok;
use imap;
use mailparse::{MailHeaderMap, parse_mail};
use time::UtcDateTime;

// i believe this can be more efficient
const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    time::macros::format_description!(version = 2, "[year]-[month]:[day] [hour]:[minute]:[second]");

pub fn get_notifier_email_replies() -> anyhow::Result<Vec<crate::types::NotifierEmailV2>> {
    // pull imap environment vars
    let imap_server = std::env::var("IMAP_SERVER").expect("NO IMAP_SERVER IN ENV FILE");
    let imap_user = std::env::var("IMAP_USER").expect("NO IMAP_USER IN ENV FILE");
    let imap_pass = std::env::var("IMAP_PASS").expect("NO IMAP_PASS IN ENV FILE");
    tracing::info!("Fetching new emails.");

    // connecting to TCP stream
    let tcp = std::net::TcpStream::connect((imap_server, 143))?;
    let client = imap::Client::new(tcp);

    // login over plain IMAP
    let mut session = client.login(imap_user, imap_pass).map_err(|e| e.0)?;
    tracing::info!("Connected to mail.");

    // mailbox.exists is number of msgs
    let mailbox = session.select("INBOX")?;
    // tracing::info!(mailbox.exists);
    if mailbox.exists == 0 {
        tracing::info!("No messages.");
        return Ok(vec![]);
    }

    // fetch messages in mailbox ZEROCOPY VECS

    tracing::info!(mailbox.exists);
    let messages = session
        .fetch("1:*", "RFC822")?
        .into_iter()
        .filter_map(|mail| {
            // could fail
            let raw_mail = mail.body()?;

            // could fial
            let parsed = match parse_mail(&raw_mail) {
                std::result::Result::Ok(p) => p,
                Err(e) => {
                    tracing::warn!("Failed to parse mail: {:?}", e);
                    return None;
                }
            };

            // parse headers and internal data
            let headers = parsed.get_headers();
            let to_email_raw = headers.get_first_value("to");

            let to_email = to_email_raw
                .as_ref()
                .map(|s| REGEX_PROMISE_ID.replace(s, "@").to_string());

            // email parsing
            let to_email = helper_functions::extract_email(&to_email?);
            let from_email = helper_functions::extract_email(&headers.get_first_value("from")?);

            let subject = headers.get_first_value("subject");

            // followed the docs - https://crates.io/crates/mailparse
            let message = parsed.subparts[0].get_body().ok();
            let body_html = parsed.subparts[1].get_body().ok();

            let is_bounce = from_email
                .as_ref()
                .map(|s| s.contains("System Administrator"))
                .unwrap_or(false)
                || message
                    .as_ref()
                    .map(|s| s.contains("could not deliver message"))
                    .unwrap_or(false)
                || headers
                    .get_first_value("report-type")
                    .map_or(false, |v| v == "delivery-status");

            // logging (not sure how to test...)
            if is_bounce {
                tracing::info!(
                    "Email bounced:\nFrom: {:?}\nTo: {:?}\nSubject: {:?}",
                    from_email,
                    to_email,
                    subject
                );
                return None;
            }
            // sometimes mobile # comes from email
            let mobile_number = from_email.as_ref().and_then(|email| {
                email
                    .split('@')
                    .filter_map(|part| {
                        // part.split_once('@')?;
                        let from_parts = part.split_once('@')?;
                        let mut from_mobile = from_parts.0;
                        if from_parts.0.chars().all(|c| c.is_numeric())
                            && from_parts.0.starts_with('1')
                        {
                            from_mobile = &from_parts.0[1..];
                        }
                        Some(from_mobile.to_string())
                    })
                    .next()
            });

            // sometimes email has promise id, not sure why
            let promise_id = to_email_raw.as_ref().and_then(|addr| {
                REGEX_PROMISE_ID.find(addr).map(|m| {
                    m.as_str()
                        .trim_start_matches("+P")
                        .trim_end_matches('@')
                        .to_string()
                })
            });

            // serialize and collect
            Some(crate::types::NotifierEmailV2 {
                to_email,
                from_email,
                promise_id,
                mobile_number,
                date: UtcDateTime::now()
                    .format(&DATE_FORMAT)
                    .unwrap_or_default()
                    .to_string(),
                subject,
                message,
                body_html,
            })
        })
        .collect::<Vec<_>>();

    anyhow::Result::Ok(messages)
}
