#![allow(non_snake_case)]
#![allow(unused_variables)]
mod bad_words;
mod db_interactions;
mod helper_functions;
mod imap_interactions;
pub mod py_types;
mod thrift;
mod types;

use crate::{
    db_interactions::process_replies, helper_functions::sql_init, thrift::ThriftClientContext,
};
use anyhow;

// use mailparse::parse_mail;
use std::{sync::LazyLock, vec};

static _REGEX_HEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new("^[0-9A-Fa-f]{4,}$").expect("should have compiled REGEX_HEX")
});
static REGEX_PROMISE_ID: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)\+P([0-9]+)@").expect("should have compiled REGEX_PROMISE_ID")
});
static REGEX_PROMISE_STRING_ID: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"Promise ID: \[(.*)\]")
        .expect("should have compiled REGEX_PROMISE_STRING_ID")
});
static REGEX_EMAIL_STOP: LazyLock<Vec<regex::Regex>> = LazyLock::new(|| {
    vec![
        regex::Regex::new(r"^--$").unwrap(),
        regex::Regex::new(r"^________________________________.*$").unwrap(),
        regex::Regex::new(r"(?i)^[- ]+Original Message.*$").unwrap(),
        regex::Regex::new(r"(?i)^On .* wrote:$").unwrap(),
        regex::Regex::new(r"(?i)^From: .*$").unwrap(),
        regex::Regex::new(r"(?i)^Sent from my.*$").unwrap(),
        regex::Regex::new(r"(?i)^> .*$").unwrap(),
    ]
});
pub static REGEX_STRIP_HTML: LazyLock<Vec<regex::Regex>> = LazyLock::new(|| {
    vec![
        regex::Regex::new(r"(?is)<script[^>]*?>.*?</script>").unwrap(),
        regex::Regex::new(r"(?is)<style[^>]*?>.*</style>").unwrap(),
        regex::Regex::new(r"(?is)<[\/\!]*?[^<>]*?>").unwrap(),
        regex::Regex::new(r"<![\s\S]*?--[ \t\n\r]*>").unwrap(),
    ]
});
pub static REGEX_REPAIR_AUTH: LazyLock<Vec<regex::Regex>> = LazyLock::new(|| {
    vec![
        regex::Regex::new(r"(?i)yes|yes").unwrap(),
        regex::Regex::new(r"(?i)no|'no'").unwrap(),
    ]
});

const AUTORESPOND_INTERVAL_HOURS: i32 = 1;

// Yes we could parse command line args, and it's very efficient, I just think to match python code we don't need this and it speeds up development

// #[derive(Debug, Parser)]
// struct Args {
//     #[clap(long, env)]
//     s_mail_server: String,
//     #[clap(long, env)]
//     s_map_username: String,
//     #[clap(long, env)]
//     s_map_password: String,
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Tracing logger that outputs log files into outer directory, will drop at end of main scope
    let _guard = helper_functions::logger();

    // Load env vars from .env file
    dotenvy::dotenv()?;
    let _db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Init thrift connection

    // Init sql connection
    let pool = sql_init().await?;

    let mut client_ctx = ThriftClientContext::new("REDACTED_THRIFT_HOST:9090")
        .expect("should have created a ThriftClientContext");

    // Process incoming replies from IMAP (for emails) and SQLDB (where we store incoming SMS messages)
    process_replies(&pool, &mut client_ctx).await?;

    Ok(())
}
