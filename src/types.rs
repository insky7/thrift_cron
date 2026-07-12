#![allow(unused_variables)]
use crate::AUTORESPOND_INTERVAL_HOURS;
use crate::REGEX_REPAIR_AUTH;
use crate::bad_words;
use crate::db_interactions::find_promise;
use crate::helper_functions;
use crate::helper_functions::get_message_hash;
use crate::thrift::ThriftClientContext;
use crate::thrift::Transcript;
use crate::thrift::TranscriptDest;
use crate::thrift::TranscriptMessageType;

use crate::thrift::{
    AlertType, DEFAULT_COMPANY_SCHEDULE, IndustryID, MobileMessage, TAlertMessageServiceSyncClient,
    TAlertServiceSyncClient, TMessagingServiceSyncClient, TPromiseServiceSyncClient,
    TSystemServiceSyncClient, TTcpaServiceSyncClient, TTranscriptServiceSyncClient,
    TranscriptDestType, TranscriptSource, TranscriptSourceType, UserFlag,
};

use chrono::Utc;
use sqlx::query;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct PromiseResult {
    pub industry_id: i32,
    pub promise_id: i32,
    pub api_user_id: i32,
    pub estimator_email: String,
    pub customer_name: String,
    pub promise_string_id: String,
    pub double_opt_in_flag: Option<i32>,
    pub double_opt_in_received_flag: Option<i32>,
    pub company_id: i32,
    pub insurance_company_id: Option<i32>, // can maybe be used?
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NotifierEmailV2 {
    pub to_email: Option<String>,
    pub from_email: Option<String>,
    pub promise_id: Option<String>,
    pub mobile_number: Option<String>,
    pub date: String,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub body_html: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NotifierEmail<'a> {
    pub to_email: Option<&'a str>,
    pub from_email: Option<&'a str>,
    pub promise_id: Option<&'a str>,
    pub mobile_number: Option<&'a str>,
    pub date: &'a str,
    pub subject: Option<&'a str>,
    pub message: Option<&'a str>,
    pub body_html: Option<&'a str>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NotifierSMS<'a> {
    pub to_number: std::borrow::Cow<'a, str>,
    pub mobile_number: std::borrow::Cow<'a, str>,
    pub date: String,
    pub message: Option<std::borrow::Cow<'a, str>>,
    pub mq_id: i32,
}

pub enum NotifierReply<'a> {
    SMS(NotifierSMS<'a>),
    Email(NotifierEmail<'a>),
}

// if we add lifetimes to above structs, we can definitely do some better implementations on our enum, with less allocation

/// Implementation of utility and handler methods for the `NotifierReply` enum.
///
/// This implementation provides a set of accessor and processing functions for handling
/// replies received via SMS or Email, typically in the context of customer communication
/// for promises, events, and notifications. The methods support extracting message details,
/// sender information, dates, and other metadata, as well as handling specific reply actions
/// such as STOP, HELP, NEWS, VEHICLE, ESTREPLY, OPTIN, and general replies.
///
/// # Key Methods
/// - `message`: Returns the message body as a string slice, if available.
/// - `to_sender`: Returns the sender's contact (phone or email), if available.
/// - `get_date`: Returns the date of the reply as a string slice.
/// - `mq_id`: Returns the message queue ID for SMS replies.
/// - `set_body`: Returns the body of the reply, supporting both SMS and Email.
/// - `get_mobile`: Returns the mobile number for SMS or body HTML for Email.
/// - `get_promise`: Returns the promise ID for Email replies.
/// - `get_to_number`: Returns the recipient's phone number for SMS.
/// - `get_subject`: Returns the subject for Email replies.
/// - `source_clause`: Returns a string indicating the source ("via Text" or "via Email").
/// - `get_from_email`: Returns the sender's email address for Email replies.
///
/// # Handler Methods
/// - `handle_cases`: Main entry point for processing a reply, determines action and dispatches to appropriate handler.
/// - `handle_stop`: Handles STOP replies, updates database flags, logs events, and sends notifications.
/// - `handle_help`: Handles HELP replies, sends help message to the customer.
/// - `handle_news`: Handles NEWS replies, sends news message to the customer.
/// - `handle_vehicle`: Handles VEHICLE replies, sends vehicle-related messages to the customer.
/// - `handle_estreply`: Handles estimator replies, processes and forwards messages, logs events.
/// - `handle_optin_reply`: Handles OPTIN replies, updates opt-in flags, logs events, and sends notifications.
/// - `handle_reply`: Handles general replies, processes and logs messages, and triggers repair authorization logic if applicable.
/// - `handle_default_reply`: Handles default customer replies, logs events, sends notifications, and triggers sentiment analysis.
/// - `send_message_notification`: Sends notifications to users or insurance contacts based on reply type.
/// - `handle_repair_auth_reply`: Handles replies to repair authorization requests, logs events, and sends notifications.
///
/// # Async Methods
/// Many handler methods are asynchronous and interact with a MySQL database using `sqlx`.
///
/// # Error Handling
/// All handler methods return `anyhow::Result` for robust error propagation.
///
/// # Usage
/// These methods are intended to be called on instances of `NotifierReply` when processing
/// incoming customer replies in a messaging or notification system.
impl<'a> NotifierReply<'a> {
    pub fn message(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(sms) => sms.message.as_deref(),
            NotifierReply::Email(email) => email.message.as_deref(),
        }
    }

    pub fn to_sender(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(sms) => Some(&sms.to_number),
            NotifierReply::Email(email) => email.to_email.as_deref(),
        }
    }
    pub fn get_date(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(sms) => Some(&sms.date),
            NotifierReply::Email(email) => Some(email.date.as_ref()),
        }
    }
    pub fn mq_id(&self) -> Option<i32> {
        match self {
            NotifierReply::SMS(sms) => Some(sms.mq_id),
            NotifierReply::Email(_) => None, // only SMS has mq_id
        }
    }
    pub fn set_body(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(sms) => Some(sms.message.as_deref()?),
            NotifierReply::Email(email) => email.body_html.as_deref(), // only SMS has mq_id
        }
    }
    pub fn get_mobile(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(sms) => Some(&sms.mobile_number),
            NotifierReply::Email(email) => email.body_html.as_deref(),
        }
    }
    pub fn get_promise(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(_sms) => None,
            NotifierReply::Email(email) => email.promise_id.as_deref(),
        }
    }
    pub fn get_to_number(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(sms) => Some(&sms.to_number),
            NotifierReply::Email(_email) => None,
        }
    }
    pub fn get_subject(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(_sms) => None,
            NotifierReply::Email(email) => email.subject.as_deref(),
        }
    }
    pub fn source_clause(&self) -> String {
        match self {
            NotifierReply::SMS(_sms) => String::from("via Text"),
            NotifierReply::Email(_email) => String::from("via Email"),
        }
    }
    pub fn get_from_email(&self) -> Option<&str> {
        match self {
            NotifierReply::SMS(_sms) => None,
            NotifierReply::Email(email) => email.from_email.as_deref(),
        }
    }

    pub async fn handle_cases(
        &self,
        pool: &sqlx::MySqlPool,
        client_ctx: &mut ThriftClientContext,
    ) -> anyhow::Result<()> {
        self.handle_stop(&pool, client_ctx).await?;
        let Some(message) = self.message() else {
            anyhow::bail!("TODO")
        };

        let filtered_message = helper_functions::filter_reply_lines(message);
        let lines = filtered_message.lines();
        let mut action = String::new();

        // check for action in message body
        for line in lines {
            let trimmed = line.trim();
            if helper_functions::is_reply_stop_line(trimmed) {
                break;
            }
            use std::result::Result::Ok;

            // todo also, carefully carefully check that this is working, this will be main entries into other message functions
            // todo build tests that trigger each of these functions, and definitely ensure we're parsing all values from both email AND sms structs properly and safely
            match client_ctx
                .tcpa_client
                .check_stop_phrase(trimmed.to_string())
            {
                Ok(true) => {
                    action = "STOP".to_string();
                }
                Ok(false) => {
                    if trimmed.eq_ignore_ascii_case("HELP") {
                        action = "HELP".to_string();
                    } else if trimmed.eq_ignore_ascii_case("TNEWS") {
                        action = "NEWS".to_string();
                    } else if trimmed.eq_ignore_ascii_case("VEHICLE") {
                        action = "VEHICLE".to_string();
                    } else if self.to_sender().is_some() && trimmed.eq_ignore_ascii_case("C") {
                        action = "OPTIN".to_string();
                    }
                }
                Err(e) => {
                    tracing::error!("check_stop_phrase failed: {:?}", e);
                }
            }
        }
        if let Some(promise) = self.get_promise() {
            tracing::info!("Promise found: {}", promise);
            action = "ESTREPLY".to_string();
        }
        tracing::info!("Action was determined to be: {:?}", &action);
        // match on action for what we will send

        self.handle_stop(pool, client_ctx).await?;
        match action.as_str() {
            "STOP" => self.handle_stop(pool, client_ctx).await?,
            "HELP" => self.handle_help(client_ctx)?,
            "NEWS" => self.handle_news(client_ctx)?,
            "VEHICLE" => self.handle_vehicle(client_ctx)?,
            "ESTREPLY" => self.handle_estreply(pool, client_ctx).await?,
            "OPTIN" => self.handle_optin_reply(pool, client_ctx).await?,
            _ => self.handle_reply(pool, client_ctx).await?,
        }
        if let Some(mq_id) = self.mq_id() {
            sqlx::query!(
                r#"UPDATE MessageQueue
           SET iProcessedFlag = 1
           WHERE iMessageQueueID = ?"#,
                mq_id
            )
            .execute(pool)
            .await?;
        }
        Ok(())
    }
    pub async fn handle_stop(
        &self,
        pool: &sqlx::MySqlPool,
        client_ctx: &mut ThriftClientContext,
    ) -> anyhow::Result<()> {
        use sqlx::Row;
        let mobile_phone_compressed = helper_functions::format_phone_number(self.get_mobile());
        tracing::info!("Test here: {:?}", mobile_phone_compressed);

        // Mark TestDrive entry as inactive + opted out
        sqlx::query!(
            r#"UPDATE TestDrive
               SET iActiveFlag = 0, iOptOutFlag = 1
               WHERE iActiveFlag = 1
                 AND sMobilePhone = ?"#,
            mobile_phone_compressed
        )
        .execute(pool)
        .await?;

        // build query for matching Promise
        let mut args = sqlx::query_builder::QueryBuilder::new(
            r#"SELECT Promise.iPromiseID, Promise.sCustomerFName, Promise.sCustomerLName,
                      Promise.iCreatedByLoginID, Login.sEmailAddress, Company.iCompanyID,
                      Industry.iIndustryID
               FROM Promise
               LEFT JOIN Customer ON Customer.iPromiseID=Promise.iPromiseID
               JOIN Login ON Login.iLoginID=Promise.iCreatedByLoginID
               LEFT JOIN Login AS Parent ON Parent.iLoginID=Login.iParentLoginID
               JOIN Company ON Company.iCompanyID=Login.iCompanyID
               JOIN Industry ON Industry.iIndustryID=Company.iIndustryID
               WHERE 1=1"#,
        );

        if let Some(pid) = self.get_promise() {
            args.push(" AND Promise.iPromiseID = ");
            args.push_bind(pid);
        }

        if let Some(mobile) = self.get_mobile() {
            args.push(" AND ((Promise.iAlternateCustomerID IS NULL AND Promise.sMobilePhone = ");
            args.push_bind(mobile);
            args.push(
                ") OR (Promise.iAlternateCustomerID IS NOT NULL AND Customer.sMobilePhone = ",
            );
            args.push_bind(mobile);
            args.push(")) ");
        }

        if let Some(from_email) = self.to_sender() {
            args.push(" AND ((Promise.iAlternateCustomerID IS NULL AND Promise.sEmail = ");
            args.push_bind(from_email);
            args.push(") OR (Promise.iAlternateCustomerID IS NOT NULL AND Customer.sEmail = ");
            args.push_bind(from_email);
            args.push(")) ");
        }

        args.push(" ORDER BY Promise.iPromiseID DESC");

        let query = args.build();

        let rows = query
            .fetch_all(pool)
            .await
            .inspect_err(|err| {
                if !matches!(err, sqlx::Error::RowNotFound) {
                    tracing::warn!("unexpected error occurred while fetching all promises: {err}");
                }
            })
            .unwrap_or_default();

        if !rows.is_empty() {
            let mut first = true;
            for promise in rows {
                let promise_id: i32 = promise.get("iPromiseID");
                let fname: String = promise.get("sCustomerFName");
                let lname: String = promise.get("sCustomerLName");
                let subject = format!(
                    "Reply received {} from customer {} {}. Subject: {}",
                    self.source_clause(),
                    fname,
                    lname,
                    self.get_subject().unwrap_or_default()
                );
                let meta_data = self.message().unwrap_or_default();
                let message_hash = helper_functions::get_message_hash(meta_data);

                // insert event log
                sqlx::query!(
                    r#"INSERT INTO PromiseEventLog (
                        iPromiseID, dtEventDateTime, sDescription, iCustNotifyFlag, sMetaData, iMetaDataFlag, sMetaDataHash, iNewFlag, iCustReplyFlag
                    ) VALUES (?, ?, ?, 0, ?, 1, ?, 1, 1)"#,
                    promise_id,
                    self.get_date().unwrap_or_default(),
                    subject,
                    meta_data,
                    message_hash
                )
                .execute(pool)
                .await?;

                // Build transcript
                let transcript = Transcript::new(
                    promise_id,
                    self.get_date().unwrap_or_default().to_string(),
                    TranscriptMessageType::CUSTOMER_OPT_OUT_REPLY,
                    TranscriptSource::CUSTOMER,
                    if self.get_from_email().is_some() {
                        TranscriptSourceType::EMAIL
                    } else {
                        TranscriptSourceType::SMS
                    },
                    self.source_clause(),
                    TranscriptDest::ESTIMATOR,
                    TranscriptDestType::EMAIL,
                    Some(promise.get("sEmailAddress")),
                    Some(subject.clone()),
                    Some(meta_data.to_string()),
                    Some(meta_data.to_string()),
                    Some(meta_data.to_string()),
                    None,
                    None,
                    None,
                    None,
                );

                // Save transcript
                let transcript_id = client_ctx
                    .transcript_client
                    .save(transcript.clone(), promise.get("iAPIUserID"))?;

                // stop the promise
                client_ctx.modelpromise_client.stop(
                    promise_id,
                    0,
                    transcript_id,
                    "Customer Reply".to_string(),
                    first,
                )?;

                // send notification on first match
                if first {
                    let login_id: &str = promise.get("iCreatedByLoginID");
                    Self::send_message_notification(
                        pool,
                        promise_id,
                        promise.get("iCompanyID"),
                        transcript,
                        self.source_clause(),
                        format!("{}_{}_{}", promise_id, login_id, lname),
                        client_ctx,
                    )
                    .await?;
                }
                first = false;
            }
        } else {
            // Send default confirmation if no match
            let stop_message = client_ctx
                .sys_config_client
                .get_value("SMS STOP REPLY".to_string())?;
            let company_name = "UpdatePromise".to_string();
            if let Some(from_email) = self.get_from_email() {
                client_ctx.messenger_client.my_mail(
                    from_email.to_string(),
                    self.get_subject().unwrap_or_default().to_string(),
                    stop_message.clone(),
                    0,
                    "".to_string(),
                    company_name.clone(),
                )?;
            }
            if let Some(mobile) = self.get_mobile() {
                let message = MobileMessage {
                    mobile_phone: Some(mobile.to_string()),
                    subject: Some(String::new()),
                    message: Some(stop_message),
                    time_zone: Some("America/Los_Angeles".to_string()),
                    return_address: Some(self.get_to_number().unwrap_or_default().to_string()),
                    check_dups: Some(false),
                    schedule: Some(DEFAULT_COMPANY_SCHEDULE.clone()),
                };
                client_ctx.messenger_client.send_mobile_message(message)?;
            }
        }

        Ok(())
    }
    pub fn handle_help(&self, client_ctx: &mut ThriftClientContext) -> anyhow::Result<()> {
        tracing::info!("Entered handle_help fn");
        // todo move to const

        if let Some(mobile) = self.get_mobile() {
            let message = MobileMessage {
                mobile_phone: Some(mobile.to_string()),
                subject: Some(String::new()),

                // sender_id: ,
                // message_body: SysConfig::get_value("SMS HELP REPLY"), // Replace with actual call
                time_zone: Some(String::from("America/Los_Angeles")), // Define TIME_ZONE appropriately
                // to_number: Some(self.get_mobile().unwrap().to_string()), // Assume method exists
                // is_urgent: false,
                schedule: Some(DEFAULT_COMPANY_SCHEDULE.clone()),
                // subject: todo!(),
                message: Some(
                    client_ctx
                        .sys_config_client
                        .get_value("SMS HELP REPLY".to_string())?,
                ),
                return_address: Some(self.get_to_number().unwrap().to_string()),
                check_dups: Some(false),
            };

            tracing::info!("Before send");
            client_ctx.messenger_client.send_mobile_message(message)?;

            tracing::info!("After send");
        }

        Ok(())
    }

    pub fn handle_news(&self, client_ctx: &mut ThriftClientContext) -> anyhow::Result<()> {
        tracing::info!("Entered handle_news fn");

        if let Some(mobile) = self.get_mobile() {
            let message = MobileMessage {
                mobile_phone: Some(mobile.to_string()),
                subject: Some(String::new()),
                // todo make sure we know the correct thrift get_value() params
                message: Some(
                    client_ctx
                        .sys_config_client
                        .get_value("SMS TNEWS REPLY".to_string())?,
                ),
                time_zone: Some("America/Los_Angeles".to_string()),
                return_address: Some(self.get_to_number().unwrap().to_string()),
                check_dups: Some(false),
                schedule: Some(DEFAULT_COMPANY_SCHEDULE.clone()),
            };

            client_ctx.messenger_client.send_mobile_message(message)?;
        }

        Ok(())
    }

    // Handler for self (NotifierReply::SMS) that sends the companys vehicle replies
    pub fn handle_vehicle(&self, client_ctx: &mut ThriftClientContext) -> anyhow::Result<()> {
        tracing::info!("Entered handle_veh fn");

        if let Some(mobile) = self.get_mobile() {
            let base_message = |body: String| MobileMessage {
                mobile_phone: Some(mobile.to_string()),
                subject: Some(String::new()),
                message: Some(body),
                time_zone: Some("America/Los_Angeles".to_string()),
                return_address: Some(self.get_to_number().unwrap().to_string()),
                check_dups: Some(false),
                schedule: Some(DEFAULT_COMPANY_SCHEDULE.clone()),
            };

            // first message
            let msg1 = base_message(
                client_ctx
                    .sys_config_client
                    .get_value("SMS VEHICLE REPLY 1".to_string())?,
            );
            client_ctx.messenger_client.send_mobile_message(msg1)?;

            // THIS IS INTENDED! (ideally don't introduce sleeps...)
            // wait 1 second
            std::thread::sleep(std::time::Duration::from_secs(1));

            // second message
            let msg2 = base_message(
                client_ctx
                    .sys_config_client
                    .get_value("SMS VEHICLE REPLY 2".to_string())?,
            );
            client_ctx.messenger_client.send_mobile_message(msg2)?;
        }

        Ok(())
    }
    async fn handle_estreply(
        &self,
        pool: &sqlx::MySqlPool,
        client_ctx: &mut ThriftClientContext,
    ) -> anyhow::Result<()> {
        tracing::info!("Entered handle_estreply fn");
        let regex = &crate::REGEX_PROMISE_STRING_ID; // already compiled

        if let Some(captures) = regex.captures(&self.message().unwrap_or_default()) {
            if let Some(matched) = captures.get(1) {
                let parts: Vec<&str> = matched.as_str().split('_').collect();

                let promise_id = parts.get(0).and_then(|s| s.parse::<i32>().ok());
                let login_id = parts.get(1).and_then(|s| s.parse::<i32>().ok());
                let cust_last_name = parts.get(2).map(|s| *s);
                let from_login_id = if parts.len() > 3 {
                    parts[3].parse::<i32>().ok()
                } else {
                    None
                };
                let from_3rd_party = if parts.len() > 4 {
                    parts[4].parse::<i32>().ok()
                } else {
                    None
                };

                // Fetch promise details including the estimator's email
                let potential_promise = query!(
                    r#"SELECT p.iPromiseID, l.sEmailAddress as estimator_email
                       FROM Promise p
                       JOIN Login l ON l.iLoginID = p.iCreatedByLoginID
                       WHERE p.iPromiseID = ?
                         AND p.iCreatedByLoginID = ?
                         AND p.sCustomerLName = ?"#,
                    promise_id,
                    login_id,
                    cust_last_name
                )
                .fetch_optional(pool)
                .await?;

                if let Some(promise) = potential_promise {
                    if let Some(body) = self.set_body() {
                        let message_hash = format!("{:x}", md5::compute(body.as_bytes()));
                        let meta_data = body.to_string();

                        // Branch for handling replies from a 3rd party
                        if from_login_id.is_some() && from_3rd_party.is_some() {
                            let sender = query!(
                                r#"SELECT sFullName, sEmailAddress, sCompanyName
                  FROM Login
                  JOIN Company ON Company.iCompanyID=Login.iCompanyID
                  WHERE iLoginID = ?"#,
                                from_login_id
                            )
                            .fetch_one(pool)
                            .await?;

                            let subject = format!(
                                "Message from estimator to {} of {} regarding customer {}",
                                sender.sFullName.as_deref().unwrap_or_default(),
                                sender.sCompanyName.as_deref().unwrap_or_default(),
                                cust_last_name.unwrap_or_default()
                            );
                            let log_subject = format!(
                                "Response received from estimator and forwarded to {} of {}",
                                sender.sFullName.as_deref().unwrap_or_default(),
                                sender.sCompanyName.as_deref().unwrap_or_default()
                            );

                            let promise_string_id = format!(
                                "{}_{}_{}_{}_1",
                                promise.iPromiseID,
                                login_id.unwrap_or_default(),
                                cust_last_name.unwrap_or_default(),
                                from_login_id.unwrap_or_default()
                            );

                            let message_html = format!(
                                r#"<div style="background-color: #eeeeee; padding: 10px;">
                                      The following message was entered by {} of {} regarding customer {}:
                                      <p></p>
                                      <pre style="background-color: #ffffff; border: 2px solid #808080; padding: 8px; font-size: 9pt;">{}</pre>
                                      A copy of this message has been saved to the Event Log for this promise. Reply to this message to auto-forward a response to your estimator and log your message in the event log for this promise.
                                      <p style="margin-top: 20px; color: #dddddd;">Promise ID: [{} overpowering]</p>
                                    </div>"#,
                                sender.sFullName.as_deref().unwrap_or_default(),
                                sender.sCompanyName.as_deref().unwrap_or_default(),
                                cust_last_name.unwrap_or_default(),
                                body,
                                promise_string_id
                            );

                            // send email
                            client_ctx.messenger_client.my_mail(
                                promise.estimator_email.unwrap_or_default(),
                                subject,
                                message_html,
                                0,              // company_id can be 0 for system messages
                                "".to_string(), // from_address
                                sender.sFullName.unwrap_or_default(),
                            )?;

                            // log event
                            query!(
                                r#"INSERT INTO PromiseEventLog (iPromiseID, dtEventDateTime, sDescription, sMetaData, iMetaDataFlag, sMetaDataHash, iNewFlag)
                                   VALUES (?, UTC_TIMESTAMP(), ?, ?, 1, ?, 0)"#,
                                promise.iPromiseID, log_subject, meta_data, message_hash
                            ).execute(pool).await?;
                        } else {
                            // branch for direct replies

                            let insurance_mail = client_ctx
                                .sys_config_client
                                .get_value("MailSenders/Insurance".to_string())?;
                            let insurance_sms = client_ctx
                                .sys_config_client
                                .get_value("ShortCodes/Insurance".to_string())?;

                            let is_insurance = self
                                .to_sender()
                                .map_or(false, |to| to == insurance_mail || to == insurance_sms);

                            let message_type = if is_insurance {
                                TranscriptMessageType::INSURANCE_REPLY
                            } else {
                                TranscriptMessageType::ESTIMATOR_REPLY
                            };
                            let source = if is_insurance {
                                TranscriptSource::INSURANCE
                            } else {
                                TranscriptSource::ESTIMATOR
                            };

                            let subject = "Status Reply".to_string();
                            let log_subject =
                                "Reply received from promise creator and forwarded to customer."
                                    .to_string();
                            let timestamp = Utc::now().format(DATE_FORMAT).to_string();

                            let transcript = Transcript::new(
                                promise.iPromiseID,
                                timestamp,
                                message_type,
                                source,
                                TranscriptSourceType::EMAIL,
                                self.get_from_email().unwrap_or_default().to_string(),
                                TranscriptDest::CUSTOMER,
                                TranscriptDestType::EMAIL,
                                self.get_mobile().unwrap_or_default().to_string(),
                                Some(subject.clone()),
                                Some(body.to_string()),
                                Some(body.to_string()),
                                Some(body.to_string()),
                                None,
                                None,
                                None,
                                None,
                            );

                            client_ctx.modelpromise_client.send_customer_message(
                                promise.iPromiseID,
                                Box::new(transcript),
                                subject.clone(),
                                body.to_string(),
                                subject,
                                body.to_string(),
                                vec![], // attachments
                                false,  // force
                            )?;

                            // Log event
                            query!(
                                r#"INSERT INTO PromiseEventLog (iPromiseID, dtEventDateTime, sDescription, iCustNotifyFlag, sMetaData, iMetaDataFlag, sMetaDataHash, iNewFlag)
                                   VALUES (?, UTC_TIMESTAMP(), ?, 0, ?, 1, ?, 0)"#,
                                promise.iPromiseID, log_subject, meta_data, message_hash
                            ).execute(pool).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    async fn handle_optin_reply(
        &self,
        pool: &sqlx::MySqlPool,
        client_ctx: &mut ThriftClientContext,
    ) -> anyhow::Result<()> {
        tracing::info!("Entered optin_reply fn");
        let send_notification;
        let message_type;

        if let Some(promise) = find_promise(&self, pool).await? {
            if promise.double_opt_in_flag.is_some() && promise.double_opt_in_received_flag.is_none()
            {
                sqlx::query!(
                    r#"UPDATE Promise
                   SET iDoubleOptInReceivedFlag=1
                   WHERE iPromiseID=?"#,
                    promise.promise_id
                )
                .execute(pool)
                .await?;
                message_type = TranscriptMessageType::CUSTOMER_OPT_IN_REPLY;
                send_notification = false;
            } else {
                message_type = TranscriptMessageType::CUSTOMER_REPLY;
                send_notification = true;
            }

            let message_hash = get_message_hash(self.set_body().unwrap_or_default());

            self.handle_default_reply(
                promise,
                pool,
                &message_hash,
                client_ctx,
                send_notification,
                message_type,
            )
            .await?;
        }
        Ok(())
    }
    pub async fn handle_reply(
        &self,
        pool: &sqlx::MySqlPool,
        client_ctx: &mut ThriftClientContext,
    ) -> anyhow::Result<()> {
        tracing::info!("Entered handle_reply fn");

        let promise = find_promise(&self, pool).await;
        let body = self.message().unwrap_or_default().trim().to_string();
        // let subject = self.().unwrap_or_default();

        if promise.is_ok()
            && self.get_promise().is_some()
            && !self
                .get_subject()
                .unwrap()
                .contains("Message from estimator")
            && !body.is_empty()
        {
            tracing::info!(
                "New message detected: {} - {}",
                self.get_date().unwrap(),
                self.get_subject().unwrap()
            );

            let source_clause = self.source_clause();
            let message_hash = helper_functions::get_message_hash(self.set_body().unwrap()); // Implement hashing logic if not present
            let mut handled = false;

            if let Some(promise) = find_promise(&self, &pool).await? {
                if promise.industry_id == IndustryID::AUTOBODY.0 {
                    // todo add handle_repair_auth_reply
                    // handled |= handle_repair_auth_reply(reply, promise, source_clause, message_hash);
                    tracing::info!("Matched on Autobody industry promise")
                } else if !handled {
                    tracing::info!("Did not match on Autobody industry promise")
                } else {
                    tracing::info!(
                        "No match found for {}",
                        &self.get_from_email().unwrap().trim()
                    )
                }
            }
            let promise = promise?.unwrap();
            // if promise?.g == IndustryID::AUTOBODY {}
            if promise.industry_id == IndustryID::AUTOBODY.0 {
                handled |= self
                    .handle_repair_auth_reply(
                        &promise,
                        &source_clause,
                        &message_hash,
                        pool,
                        client_ctx,
                    )
                    .await?;
            }

            if !handled {
                self.handle_repair_auth_reply(
                    &promise,
                    &source_clause,
                    &message_hash,
                    pool,
                    client_ctx,
                )
                .await?;
            }
        } else if let Some(from_email) = self.get_from_email() {
            if !from_email.trim().is_empty() {
                tracing::info!("No match found for {}", from_email);
            }
        }

        Ok(())
    }

    pub async fn handle_default_reply(
        &self,
        promise: PromiseResult,
        pool: &sqlx::MySqlPool,
        message_hash: &str,
        client_ctx: &mut ThriftClientContext,
        send_notification: bool,
        message_type: TranscriptMessageType,
    ) -> anyhow::Result<()> {
        let subject = self.get_subject().unwrap_or("(No subject)");
        let log_subject = format!(
            "Reply received {} from customer {}. Subject: {}",
            self.source_clause(),
            promise.customer_name,
            subject
        );

        // Filter survey links
        let short_url = regex::escape(
            &client_ctx
                .sys_config_client
                .get_value("Short URL".to_string())?,
        );
        let pattern = format!(r"{}.+\b", short_url);
        let re = regex::Regex::new(&pattern)?;
        let body_with_links_filtered =
            re.replace_all(self.set_body().unwrap_or_default(), "[Link] ");
        let body_filtered = bad_words::replace_bad_words(&body_with_links_filtered);

        sqlx::query!(
            r#"
        INSERT INTO PromiseEventLog (
            iPromiseID,
            dtEventDateTime,
            sDescription,
            iCustNotifyFlag,
            sMetaData,
            iMetaDataFlag,
            sMetaDataHash,
            iNewFlag,
            iCustReplyFlag
        ) VALUES (?, ?, ?, 0, ?, 1, ?, 1, 1)
        "#,
            promise.promise_id,
            self.get_date(),
            log_subject,
            self.set_body(),
            message_hash
        )
        .execute(pool)
        .await?;

        // Set new message flag in promise
        sqlx::query!(
            r#"UPDATE Promise
           SET iNewMessageFlag=1
           WHERE iPromiseID = ?"#,
            promise.promise_id
        )
        .execute(pool)
        .await?;

        // Determine if insurance
        let insurance_mail = std::env::var("MailSendersInsurance").unwrap_or_default();
        let insurance_sms = std::env::var("ShortCodesInsurance").unwrap_or_default();
        let is_insurance = match self {
            NotifierReply::Email(_) => self
                .to_sender()
                .map(|s| s == insurance_mail)
                .unwrap_or(false),
            NotifierReply::SMS(_) => self
                .to_sender()
                .map(|s| s == insurance_sms)
                .unwrap_or(false),
        };

        let s_source_type: Option<TranscriptSourceType>;
        let mut s_source_comm = String::new();
        if let Some(source) = self.get_mobile() {
            s_source_type = Some(TranscriptSourceType::SMS);
            s_source_comm = helper_functions::format_phone_number(Some(source)).unwrap_or_default();
        } else if let Some(source) = self.get_from_email() {
            s_source_type = Some(TranscriptSourceType::EMAIL);
            s_source_comm = source.to_string();
        } else {
            tracing::info!("Could not determine reply source (mobile/email)");
            s_source_type = None;
        }

        let transcript = Transcript::new(
            promise.promise_id,
            Utc::now().format(DATE_FORMAT).to_string(),
            message_type,
            TranscriptSource::CUSTOMER,
            s_source_type,
            s_source_comm,
            if is_insurance {
                TranscriptDest::INSURANCE
            } else {
                TranscriptDest::ESTIMATOR
            },
            TranscriptDestType::EMAIL,
            promise.estimator_email.clone(),
            Some(subject.to_string()),
            Some(self.set_body().unwrap_or_default().to_string()),
            body_filtered.to_string(),
            Some(self.set_body().unwrap_or_default().to_string()),
            None,
            None,
            None,
            None,
        );
        if send_notification {
            Self::send_message_notification(
                pool,
                promise.promise_id,
                promise.company_id,
                transcript.clone(),
                self.source_clause(),
                promise.promise_string_id.clone(),
                client_ctx,
            )
            .await?;
        }
        let transcript_id = client_ctx
            .transcript_client
            .save(transcript.clone(), promise.api_user_id)?;

        // Send auto response away message if needed
        if self.get_mobile().is_some() {
            let rows = sqlx::query!(
                r#"SELECT sAwayMessage
               FROM Promise
               JOIN Login ON Login.iLoginID = Promise.iCreatedByLoginID
               LEFT JOIN PromiseTranscript ON PromiseTranscript.iPromiseID = Promise.iPromiseID
                 AND Login.sAwayMessage = PromiseTranscript.sBody
                 AND PromiseTranscript.dtDateTimeStamp > UTC_TIMESTAMP() - INTERVAL ? HOUR
               WHERE Login.iAwayFlag = 1
                 AND Promise.iPromiseID = ?
                 AND PromiseTranscript.iPromiseTranscriptID IS NULL"#,
                AUTORESPOND_INTERVAL_HOURS,
                promise.promise_id
            )
            .fetch_optional(pool)
            .await?;

            if let Some(row) = rows {
                if let Some(s_away_message) = row.sAwayMessage {
                    client_ctx.modelpromise_client.send_customer_message(
                        promise.promise_id,
                        Box::new(transcript.clone()),
                        "Away".to_string(),
                        s_away_message.clone(),
                        "Away".to_string(),
                        s_away_message,
                        vec![],
                        true,
                    )?;
                }
            }
        }

        // add the message to the TCPA Reply list
        client_ctx.tcpa_client.insert_reply(transcript_id)?;

        // add the message to the sentiment engine queue for sentiment analysis
        helper_functions::add_phrase_to_cache_queue(
            self.set_body().unwrap_or_default(),
            Some(promise.industry_id),
            Some(promise.promise_id),
            Some(transcript_id),
        )
        .await?;

        // send to pusher queue for real time notifications
        #[derive(serde::Serialize)]
        struct PusherPayload {
            company_id: i32,
            payload: InnerPayload,
        }
        #[derive(serde::Serialize)]
        struct InnerPayload {
            promise: i32,
            history: i32,
        }
        let record = PusherPayload {
            company_id: promise.company_id,
            payload: InnerPayload {
                promise: promise.promise_id,
                history: 1,
            },
        };
        let json_record = serde_json::to_string(&record)?;
        helper_functions::send_pusher_notification(json_record)
            .await
            .expect("Pusher is not working");

        Ok(())
    }
    pub async fn send_message_notification(
        pool: &sqlx::MySqlPool,
        promise_id: i32,
        company_id: i32,
        transcript: Transcript,
        source_clause: String,
        promise_string_id: String,
        client_ctx: &mut ThriftClientContext,
    ) -> anyhow::Result<()> {
        let response_to = "a status update".to_string();
        let customer_name = query!(
            r#"SELECT sCustomerFName, sCustomerLName, iInsuranceCompanyID
           FROM Promise
           WHERE iPromiseID = ?"#,
            promise_id
        )
        .fetch_one(pool)
        .await?;
        let s_from_name = format!(
            "{}, {}",
            customer_name.sCustomerFName.unwrap(),
            customer_name.sCustomerLName.unwrap()
        );
        // let Industry::
        let body_filter = bad_words::replace_bad_words(&transcript.s_body_filtered.unwrap());
        let mut token_map = client_ctx.modelpromise_client.get_tokens(promise_id)?;
        if transcript.s_dest == Some(TranscriptDest::INSURANCE) {
            let alert_message = client_ctx.alertmessage_client.get_alert_message(
                AlertType::TYPE_CUSTOMER_REPLY.into(),
                company_id,
                true,
            )?;

            token_map.insert(
                "Subject".to_string(),
                transcript.s_subject.clone().unwrap_or_default(),
            );
            let subject_complete = client_ctx.modelpromise_client.replace_tokens(
                promise_id,
                alert_message.subject.clone().unwrap(),
                token_map.clone(),
            );

            token_map.remove("Subject");
            token_map.insert("Message Body".to_string(), body_filter);
            token_map.insert("Promise ID".to_string(), promise_string_id.clone());
            token_map.insert("Source".to_string(), source_clause.clone());
            token_map.insert("In Response To".to_string(), response_to.clone());

            let body_complete = client_ctx.modelpromise_client.replace_tokens(
                promise_id,
                alert_message.body.unwrap().clone(),
                token_map,
            )?;
            let signature = helper_functions::get_signature_insurance(
                pool,
                company_id,
                customer_name.iInsuranceCompanyID.unwrap_or_default(),
            )
            .await?;

            if let Some((login_id, _signature_email, _signature_text, email_address)) = signature {
                let from_add = client_ctx
                    .sys_config_client
                    .get_value("MailSenders/Insurance".to_string())?;
                client_ctx.messenger_client.my_mail(
                    email_address,
                    subject_complete.unwrap(),
                    body_complete,
                    login_id,
                    from_add,
                    s_from_name,
                )?;
            } else {
                tracing::error!(
                    "No insurance signature found for company_id {} insurance_company_id {}",
                    company_id,
                    customer_name.iInsuranceCompanyID.unwrap_or_default()
                );
                return Err(anyhow::anyhow!("No insurance signature found"));
            }
        } else {
            client_ctx.alert_client.send_users(
                AlertType::TYPE_CUSTOMER_REPLY,
                vec![
                    UserFlag::FLAG_MANAGER.into(),
                    UserFlag::FLAG_NORMAL.into(),
                    UserFlag::FLAG_INSURANCE.into(),
                ],
                promise_id.into(),
                token_map,
                s_from_name,
            )?;
            tracing::info!("New message copy sent to local users");
        }

        Ok(())
    }
    pub async fn handle_repair_auth_reply(
        &self,
        promise: &PromiseResult,
        source_clause: &str,
        message_hash: &str,
        pool: &sqlx::MySqlPool,
        client_ctx: &mut ThriftClientContext,
    ) -> anyhow::Result<bool> {
        // see if a repair auth message went out recently
        let repair_auth = sqlx::query!(
            r#"SELECT iPromiseEventLogID, sMetaData
               FROM PromiseEventLog
               WHERE iPromiseID = ?
                 AND (sMetaData LIKE '%Initial Repair Authorization%' OR sMetaData LIKE '%Additional Repair Authorization%')
                 AND iRelatedPromiseEventLogID IS NULL
               ORDER BY dtEventDateTime DESC
               LIMIT 1"#,
            promise.promise_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(auth_event) = repair_auth {
            let meta_data = auth_event.sMetaData.as_deref().unwrap_or_default();
            let last_amount_sent = helper_functions::extra_repair_auth_amount(meta_data, false);
            let initial_flag = meta_data.matches('$').count() == 1;
            let mut yes_flag = false;
            let mut no_flag = false;
            let body = self.message().unwrap_or_default();

            for line in body.lines() {
                // use the consolidated regex vector by index
                if REGEX_REPAIR_AUTH[0].is_match(line) {
                    // check for "yes"
                    yes_flag = true;
                } else if REGEX_REPAIR_AUTH[1].is_match(line) {
                    // check for "no"
                    no_flag = true;
                }
            }

            let mut description = String::new();
            // Handle "Yes" reply
            if yes_flag {
                let base_message =
                    format!("Customer Reply Received {}: \"{}\"; ", source_clause, body);
                description = if initial_flag {
                    format!(
                        "{}Customer agreed to initial repair amount of ${:.2}.",
                        base_message, last_amount_sent
                    )
                } else {
                    let total_amount = helper_functions::extra_repair_auth_amount(meta_data, true);
                    format!(
                        "{}Customer agreed to additional repair amount of ${:.2} for a total of ${:.2}.",
                        base_message, last_amount_sent, total_amount
                    )
                };
            // Handle "No" reply
            } else if no_flag {
                let base_message =
                    format!("Customer Reply Received {}: \"{}\"; ", source_clause, body);
                description = if initial_flag {
                    format!(
                        "{}Customer declined initial repair amount of ${:.2}.",
                        base_message, last_amount_sent
                    )
                } else {
                    let total_amount = helper_functions::extra_repair_auth_amount(meta_data, true);
                    format!(
                        "{}Customer declined additional repair amount of ${:.2} for a total of ${:.2}.",
                        base_message, last_amount_sent, total_amount
                    )
                };
            }

            // If a "Yes" or "No" was found, process the reply
            if !description.is_empty() {
                let date = self.get_date().unwrap_or_default();
                let result = sqlx::query!(
                    r#"INSERT INTO PromiseEventLog (iPromiseID, dtEventDateTime, sDescription, sMetaData, iMetaDataFlag, sMetaDataHash, iNewFlag)
                       VALUES (?, ?, ?, '', 0, ?, 0)"#,
                    promise.promise_id, date, description, message_hash
                )
                .execute(pool)
                .await?;

                let new_log_id = result.last_insert_id();

                // Link the original auth message to this reply
                sqlx::query!(
                    "UPDATE PromiseEventLog SET iRelatedPromiseEventLogID = ? WHERE iPromiseEventLogID = ?",
                    new_log_id, auth_event.iPromiseEventLogID
                )
                .execute(pool)
                .await?;

                let (source_type, source_comm) = if self.get_mobile().is_some() {
                    (
                        TranscriptSourceType::SMS,
                        self.get_mobile().unwrap_or_default().to_string(),
                    )
                } else {
                    (
                        TranscriptSourceType::EMAIL,
                        self.get_from_email().unwrap_or_default().to_string(),
                    )
                };

                let transcript = Transcript::new(
                    promise.promise_id,
                    Utc::now().format(DATE_FORMAT).to_string(),
                    TranscriptMessageType::REPAIR_AUTH_REPLY,
                    TranscriptSource::CUSTOMER,
                    source_type,
                    source_comm,
                    TranscriptDest::ESTIMATOR,
                    TranscriptDestType::EMAIL,
                    promise.estimator_email.clone(),
                    self.get_subject().map(String::from),
                    Some(body.to_string()),
                    Some(body.to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                );

                Self::send_message_notification(
                    pool,
                    promise.promise_id,
                    promise.company_id,
                    transcript.clone(),
                    source_clause.to_string(),
                    promise.promise_string_id.clone(),
                    client_ctx,
                )
                .await?;

                client_ctx
                    .transcript_client
                    .save(transcript, promise.api_user_id)?;

                return Ok(true);
            }
        }

        Ok(false)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_notifier_email_serde() {
        let email = NotifierEmail {
            to_email: Some("to@example.com"),
            from_email: Some("from@example.com"),
            promise_id: Some("123"),
            mobile_number: Some("5551234"),
            date: "2024-01-01 12:00:00",
            subject: Some("Test Subject"),
            message: Some("Test message"),
            body_html: Some("<b>HTML</b>"),
        };
        let ser = serde_json::to_string(&email).unwrap();
        let de: NotifierEmail = serde_json::from_str(&ser).unwrap();
        assert_eq!(email.to_email, de.to_email);
        assert_eq!(email.message, de.message);
    }

    #[test]
    fn test_notifier_sms_serde() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5551234"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("Test SMS")),
            mq_id: 42,
        };
        let ser = serde_json::to_string(&sms).unwrap();
        let de: NotifierSMS = serde_json::from_str(&ser).unwrap();
        assert_eq!(sms.to_number, de.to_number);
        assert_eq!(sms.message, de.message);
    }

    #[test]
    fn test_notifier_reply_message() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5551234"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("SMS body")),
            mq_id: 42,
        };
        let email = NotifierEmail {
            to_email: Some("to@example.com"),
            from_email: Some("from@example.com"),
            promise_id: Some("123"),
            mobile_number: Some("5551234"),
            date: "2024-01-01 12:00:00",
            subject: Some("Subject"),
            message: Some("Email body"),
            body_html: Some("<b>HTML</b>"),
        };
        let reply_sms = NotifierReply::SMS(sms);
        let reply_email = NotifierReply::Email(email);
        assert_eq!(reply_sms.message(), Some("SMS body"));
        assert_eq!(reply_email.message(), Some("Email body"));
    }

    #[test]
    fn test_notifier_reply_to_sender() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5551234"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("SMS body")),
            mq_id: 42,
        };
        let email = NotifierEmail {
            to_email: Some("to@example.com"),
            from_email: Some("from@example.com"),
            promise_id: Some("123"),
            mobile_number: Some("5551234"),
            date: "2024-01-01 12:00:00",
            subject: Some("Subject"),
            message: Some("Email body"),
            body_html: Some("<b>HTML</b>"),
        };
        let reply_sms = NotifierReply::SMS(sms);
        let reply_email = NotifierReply::Email(email);
        assert_eq!(reply_sms.to_sender(), Some("5551234"));
        assert_eq!(reply_email.to_sender(), Some("to@example.com"));
    }

    #[test]
    fn test_notifier_reply_getters() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5551234"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("SMS body")),
            mq_id: 42,
        };
        let email = NotifierEmail {
            to_email: Some("to@example.com"),
            from_email: Some("from@example.com"),
            promise_id: Some("123"),
            mobile_number: Some("5551234"),
            date: "2024-01-01 12:00:00",
            subject: Some("Subject"),
            message: Some("Email body"),
            body_html: Some("<b>HTML</b>"),
        };
        let reply_sms = NotifierReply::SMS(sms);
        let reply_email = NotifierReply::Email(email);

        assert_eq!(reply_sms.mq_id(), Some(42));
        assert_eq!(reply_email.mq_id(), None);

        assert_eq!(reply_sms.get_date(), Some("2024-01-01 13:00:00"));
        assert_eq!(reply_email.get_date(), Some("2024-01-01 12:00:00"));

        assert_eq!(reply_sms.get_promise(), None);
        assert_eq!(reply_email.get_promise(), Some("123"));

        assert_eq!(reply_sms.get_to_number(), Some("5551234"));
        assert_eq!(reply_email.get_to_number(), None);

        assert_eq!(reply_sms.get_subject(), None);
        assert_eq!(reply_email.get_subject(), Some("Subject"));

        assert_eq!(reply_sms.source_clause(), "via Text");
        assert_eq!(reply_email.source_clause(), "via Email");

        assert_eq!(reply_sms.get_from_email(), None);
        assert_eq!(reply_email.get_from_email(), Some("from@example.com"));
    }
    #[test]
    fn test_notifier_email_partial_fields() {
        let email = NotifierEmail {
            to_email: None,
            from_email: Some("from@example.com"),
            promise_id: None,
            mobile_number: None,
            date: "2024-01-01 12:00:00",
            subject: None,
            message: None,
            body_html: None,
        };
        let ser = serde_json::to_string(&email).unwrap();
        let de: NotifierEmail = serde_json::from_str(&ser).unwrap();
        assert_eq!(email.from_email, de.from_email);
        assert_eq!(email.to_email, de.to_email);
        assert_eq!(email.promise_id, de.promise_id);
    }

    #[test]
    fn test_notifier_sms_partial_fields() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5551234"),
            date: "2024-01-01 13:00:00".to_string(),
            message: None,
            mq_id: 0,
        };
        let ser = serde_json::to_string(&sms).unwrap();
        let de: NotifierSMS = serde_json::from_str(&ser).unwrap();
        assert_eq!(sms.to_number, de.to_number);
        assert_eq!(sms.message, de.message);
        assert_eq!(sms.mq_id, de.mq_id);
    }

    #[test]
    fn test_notifier_reply_message_none() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5551234"),
            date: "2024-01-01 13:00:00".to_string(),
            message: None,
            mq_id: 42,
        };
        let email = NotifierEmail {
            to_email: Some("to@example.com"),
            from_email: Some("from@example.com"),
            promise_id: Some("123"),
            mobile_number: Some("5551234"),
            date: "2024-01-01 12:00:00",
            subject: Some("Subject"),
            message: None,
            body_html: Some("<b>HTML</b>"),
        };
        let reply_sms = NotifierReply::SMS(sms);
        let reply_email = NotifierReply::Email(email);
        assert_eq!(reply_sms.message(), None);
        assert_eq!(reply_email.message(), None);
    }

    #[test]
    fn test_notifier_reply_get_mobile_email() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5555678"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("SMS body")),
            mq_id: 42,
        };
        let email = NotifierEmail {
            to_email: Some("to@example.com"),
            from_email: Some("from@example.com"),
            promise_id: Some("123"),
            mobile_number: Some("5559999"),
            date: "2024-01-01 12:00:00",
            subject: Some("Subject"),
            message: Some("Email body"),
            body_html: Some("body html"),
        };
        let reply_sms = NotifierReply::SMS(sms);
        let reply_email = NotifierReply::Email(email);

        assert_eq!(reply_sms.get_mobile(), Some("5555678"));
        assert_eq!(reply_email.get_mobile(), Some("body html"));
    }

    #[test]
    fn test_notifier_reply_set_body() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5555678"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("SMS body")),
            mq_id: 42,
        };
        let email = NotifierEmail {
            to_email: Some("to@example.com"),
            from_email: Some("from@example.com"),
            promise_id: Some("123"),
            mobile_number: Some("5559999"),
            date: "2024-01-01 12:00:00",
            subject: Some("Subject"),
            message: Some("Email body"),
            body_html: Some("body html"),
        };
        let reply_sms = NotifierReply::SMS(sms);
        let reply_email = NotifierReply::Email(email);

        assert_eq!(reply_sms.set_body(), Some("SMS body"));
        assert_eq!(reply_email.set_body(), Some("body html"));
    }

    #[test]
    fn test_notifier_reply_get_subject_none() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5555678"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("SMS body")),
            mq_id: 42,
        };
        let reply_sms = NotifierReply::SMS(sms);
        assert_eq!(reply_sms.get_subject(), None);
    }

    #[test]
    fn test_notifier_reply_get_from_email_none() {
        let sms = NotifierSMS {
            to_number: std::borrow::Cow::Borrowed("5551234"),
            mobile_number: std::borrow::Cow::Borrowed("5555678"),
            date: "2024-01-01 13:00:00".to_string(),
            message: Some(std::borrow::Cow::Borrowed("SMS body")),
            mq_id: 42,
        };
        let reply_sms = NotifierReply::SMS(sms);
        assert_eq!(reply_sms.get_from_email(), None);
    }
}
