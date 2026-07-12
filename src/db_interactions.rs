#![allow(non_snake_case)]
#![allow(unused_variables)]
use crate::thrift::ThriftClientContext;
// use crate::types::ModelPromise;
use crate::imap_interactions::get_notifier_email_replies;
use crate::types::NotifierReply;
use crate::types::PromiseResult;
use anyhow::{self, Ok};
use sqlx::Row;
use sqlx::{self, MySqlPool};
use time::macros::format_description;
const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!(version = 2, "[year]-[month]:[day] [hour]:[minute]:[second]");

// DB interaction that has match clauses for promise ID's in our SQL DB
pub async fn find_promise<'a>(
    reply: &NotifierReply<'a>,
    pool: &MySqlPool,
) -> anyhow::Result<Option<PromiseResult>> {
    let mut match_clause = String::new();
    let mut query_params: Vec<String> = vec![];

    if let Some(promise_id) = reply.get_promise() {
        let exists = sqlx::query_scalar!(
            r#"SELECT 1 as iExists FROM Promise WHERE iPromiseID = ?"#,
            promise_id
        )
        .fetch_optional(pool)
        .await?;

        if exists.is_some() {
            match_clause = "AND (Promise.iPromiseID = ?)".to_string();
            query_params.push(promise_id.to_string());
        }
    }

    if match_clause.is_empty() {
        if let Some(mobile_number) = reply.get_mobile() {
            match_clause = "AND (((Promise.iAlternateCustomerID IS NULL AND Promise.sMobilePhone = ?) \
                            OR (Promise.iAlternateCustomerID IS NOT NULL AND Customer.sMobilePhone = ?))"
                .to_string();
            query_params.push(mobile_number.to_string());
            query_params.push(mobile_number.to_string());

            if let Some(to_number) = reply.get_to_number() {
                if to_number != "InsuranceShortCodeFromConfig" {
                    match_clause.push_str(" AND (Industry.sShortCode = ? OR L2.sCloudPhone = ? OR Company.sCloudPhone = ?)");
                    query_params.push(to_number.to_string());
                    query_params.push(to_number.to_string());
                    query_params.push(to_number.to_string());
                }
            }

            if let Some(from_email) = reply.get_from_email() {
                match_clause.push_str(" OR ((Promise.iAlternateCustomerID IS NULL AND Promise.sEmail = ?) \
                                        OR (Promise.iAlternateCustomerID IS NOT NULL AND Customer.sEmail = ?))");
                query_params.push(from_email.to_string());
                query_params.push(from_email.to_string());
            }

            match_clause.push(')');
        }
    }

    if match_clause.is_empty() {
        return Ok(None);
    }

    let sql = format!(
        r#"
        SELECT 
            IF(Promise.iAlternateCustomerID, Customer.sCustomerFName, Promise.sCustomerFName) AS sCustomerFName,
            IF(Promise.iAlternateCustomerID, Customer.sCustomerLName, Promise.sCustomerLName) AS sCustomerLName,
            Promise.iPromiseID, Promise.iAPIUserID, iCreatedByLoginID, sFullName, sEmailAddress,
            iDoubleOptInFlag, iDoubleOptInReceivedFlag,
            Company.iCompanyID, Company.iIndustryID, Promise.iInsuranceCompanyID
        FROM Promise
        LEFT JOIN Customer ON Customer.iPromiseID = Promise.iPromiseID
        JOIN Login AS L2 ON L2.iLoginID = Promise.iCreatedByLoginID
        JOIN Company ON Company.iCompanyID = L2.iCompanyID
        JOIN Industry ON Industry.iIndustryID = Company.iIndustryID
        WHERE L2.iLiveFlag = 1
        {}
        ORDER BY iCancelFlag ASC, dtDelivered IS NULL DESC, dtDelivered DESC, dtCreatedOn DESC
        LIMIT 1
        "#,
        match_clause
    );

    let mut query = sqlx::query(&sql);
    for param in &query_params {
        query = query.bind(param);
    }

    if let Some(row) = query.fetch_optional(pool).await? {
        let first = |col: &str| row.try_get::<String, _>(col).unwrap_or_default();
        let id = |col: &str| row.try_get::<i32, _>(col).unwrap_or(0);

        let promise_id = id("iPromiseID");
        let login_id = id("iCreatedByLoginID");
        let last_name = first("sCustomerLName");

        let result = PromiseResult {
            industry_id: id("iIndustryID"),
            promise_id,
            api_user_id: id("iAPIUserID"),
            estimator_email: first("sEmailAddress"),
            customer_name: format!("{} {}", first("sCustomerFName"), last_name),
            promise_string_id: format!("{}_{}_{}", promise_id, login_id, last_name),
            double_opt_in_flag: Some(id("iDoubleOptInFlag")),
            double_opt_in_received_flag: Some(id("iDoubleOptInReceivedFlag")),
            company_id: id("iCompanyID"),
            insurance_company_id: Some(id("iInsuranceCompanyID")),
        };

        Ok(Some(result))
    } else {
        Ok(None)
    }
}

pub async fn process_replies(
    pool: &sqlx::MySqlPool,
    ctx: &mut ThriftClientContext,
) -> anyhow::Result<()> {
    tracing::info!("Fetching new text messages.");

    #[derive(Debug, sqlx::FromRow)]
    struct MessageQueueRow {
        iMessageQueueID: i32,
        dtTimeStamp: Option<time::PrimitiveDateTime>,
        sFrom: Option<String>,
        sTo: Option<String>,
        sMessage: Option<String>,
    }
    // vec of records from db for SMS messages
    let msgs: Vec<MessageQueueRow> = sqlx::query_as!(
        MessageQueueRow,
        r#"SELECT iMessageQueueID, dtTimeStamp, sFrom, sTo, sMessage
           FROM MessageQueue
           WHERE iProcessedFlag = 0
             AND iFromMobileFlag = 1
           ORDER BY dtTimeStamp,iSortOrder"#
    )
    .fetch_all(pool)
    .await?;

    let email_msgs = get_notifier_email_replies()?;

    let replies = msgs
        .iter()
        .filter_map(|x| {
            let date = x.dtTimeStamp.unwrap().format(DATE_FORMAT).ok()?;
            let to_number = x.sTo.as_deref()?;

            let mq_id = x.iMessageQueueID;

            let message = x.sMessage.as_deref();
            let mobile_number = {
                let s = x.sFrom.as_deref()?.trim();
                let starts_with_plus = s.starts_with("+1");
                let ret = s.trim_start_matches(|c: char| c == '+');
                if starts_with_plus { &ret[1..] } else { ret }
            };

            Some(crate::types::NotifierSMS {
                to_number: std::borrow::Cow::Borrowed(to_number),
                mobile_number: std::borrow::Cow::Borrowed(mobile_number),
                date,
                message: message.map(std::borrow::Cow::Borrowed),
                mq_id,
            })
        })
        .map(crate::types::NotifierReply::SMS)
        .chain(
            email_msgs
                .iter()
                .map(|v| crate::types::NotifierEmail {
                    to_email: v.to_email.as_deref(),
                    from_email: v.from_email.as_deref(),
                    promise_id: v.promise_id.as_deref(),
                    mobile_number: v.mobile_number.as_deref(),
                    date: v.date.as_str(),
                    subject: v.subject.as_deref(),
                    message: v.message.as_deref(),
                    body_html: v.body_html.as_deref(),
                })
                .map(crate::types::NotifierReply::Email),
        )
        .collect::<Vec<_>>();

    let num_replies = replies.len();
    for reply in replies {
        reply
            .handle_cases(pool, ctx)
            .await
            .map_err(|x| tracing::info!("{:?}", x))
            .unwrap();
    }

    tracing::info!("Reply Count = {num_replies}");

    Ok(())
}
