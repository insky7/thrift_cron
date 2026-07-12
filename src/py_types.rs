// use anyhow::{self, Ok};
// use sqlx;
// use sqlx::MySql;
// use sqlx::Row;
// use sqlx::mysql::MySqlRow;
// use sqlx::query;
// use sqlx::query_as;
// use sqlx::types::Json;
// pub struct Alert;
// pub struct AlertMessage;
// impl Alert {
//     pub const FLAG_NORMAL: i32 = 1;
//     pub const FLAG_MANAGER: i32 = 2;
//     pub const FLAG_CORPORATE: i32 = 4;
//     pub const FLAG_INSURANCE: i32 = 8;
//     pub const FLAG_RECEPTIONIST: i32 = 16;
//     pub const TYPE_SQ_SURVEY: i32 = 1;
//     pub const TYPE_UNPROMISED: i32 = 2;
//     pub const TYPE_CSI_SHOP: i32 = 3;
//     pub const TYPE_PROMISE_NO_PHONE: i32 = 4;
//     pub const TYPE_PROMISE_INVALID_EMAIL: i32 = 5;
//     pub const TYPE_PROMISE_MISSING_EMAIL: i32 = 6;
//     pub const TYPE_CUSTOMER_REPLY: i32 = 9;
//     pub const TYPE_CUSTOMER_REPLY_INSURANCE: i32 = 12;
//     pub const TYPE_LOW_COMMENT_SENTIMENT: i32 = 10;
//     pub const TYPE_DUPLICATE_CONTACT_INFO: i32 = 11;
//     pub const TYPE_PROMISE_EXPIRATION: i32 = 13;
//     pub const TYPE_CUSTOMER_PAYMENT: i32 = 14;
//     pub const TYPE_CUSTOMER_SIGNATURE: i32 = 15;
//     pub const TYPE_PENDING_ATTACHMENT: i32 = 17;
//     pub const TYPE_UNREAD_CUSTOMER_REPLY: i32 = 18;
//     pub const TYPE_MPI_FORM_COMPLETE: i32 = 20;
//     pub const TYPE_PRODUCTION_STATUS: i32 = 21;
//     pub const TYPE_PDF_FOLLOWUP: i32 = 22;
//     pub const TYPE_MANAGEMENT_SYSTEM_ALERT: i32 = 23;
// }
// impl AlertMessage {
//     pub async fn alert_handler(pool: sqlx::MySqlPool, company_id: i32) -> anyhow::Result<()> {
//         let alerts = sqlx::query!(
//             r#"SELECT
//               iAlertMessageID
//               ,Alert.iAlertID
//               ,sSubject
//               ,sBody
//               ,sBodyHTML
//               ,iHours
//               ,iCompanyID
//               ,dtCreated
//               ,dtUpdated
//               ,Alert.sName
//             FROM AlertMessage
//             JOIN Alert ON Alert.iAlertID=AlertMessage.iAlertID
//             WHERE
//               AlertMessage.iAlertID = ?
//               AND iCompanyID = ?"#,
//               Alert::TYPE_CUSTOMER_REPLY_INSURANCE,
//               company_id
//         )

//         .fetch_all(&pool)
//         .await?;
//         if alerts.is_empty() {
//             return Ok(());
//         } else {
//             for alert in alerts {
//                 alert.sSubject
//             }
//             serde_json::json!()
//         }

//         Ok(())
//     }
// }

// // #[derive(Debug, FromRow)]
// // pub struct ModelPromise {
// //     pub iPromiseID: i32,
// //     pub sCustomerFName: Option<String>,
// //     pub sCustomerLName: Option<String>,
// //     pub sMobilePhone: Option<String>,
// //     pub sHomePhone: Option<String>,
// //     pub sWorkPhone: Option<String>,
// //     pub sEmail: Option<String>,
// //     pub sCommPreference: Option<String>,
// //     pub sMessageTriggers: Option<String>,
// //     pub iPromiseTypeID: Option<i32>,
// //     pub iCreatedByLoginID: Option<i32>,
// //     pub iLoginIDOverride: Option<i32>,
// //     pub iCancelFlag: Option<i32>,
// //     pub iAPIUserID: Option<i32>,
// //     pub iInsuranceCompanyID: Option<i32>,
// //     pub sClaimNumber: Option<String>,
// //     pub sUniqueIdentifier: Option<String>,
// //     pub iCommPreferenceOverride: Option<i32>,
// //     pub sExternalURL: Option<String>,
// //     pub iDataShare: Option<i32>,
// //     pub iSendPDF: Option<i32>,
// //     pub iSendCSI: Option<i32>,
// //     pub iSendSR: Option<i32>,
// //     pub iSendPDN: Option<i32>,
// //     pub iEmailInitialFlag: Option<i32>,
// //     pub iMobilePhoneInitialFlag: Option<i32>,
// //     pub iInsuranceEmailInitialFlag: Option<i32>,
// //     pub iInsuranceMobilePhoneInitialFlag: Option<i32>,
// //     pub iStopFlag: Option<i32>,
// //     pub sExternalCustomerID: Option<String>,
// //     pub dtFollowUpCall: Option<NaiveDateTime>,
// //     pub iFollowUpReminder: Option<i32>,
// //     pub dtAppointment: Option<NaiveDateTime>,
// //     pub sService: Option<String>,
// //     pub dtCreatedOn: Option<NaiveDateTime>,
// //     pub iRand: Option<i32>,
// //     pub iAlternateCustomerID: Option<i32>,
// //     pub iCustomerID: Option<i32>,
// //     pub iVehicleID: Option<i32>,
// //     pub sConcerns: Option<String>,
// //     pub sNotes: Option<String>,
// //     pub sStatus: Option<String>,
// //     pub sExternalID: Option<String>,
// //     pub iSourceID: Option<i32>,
// //     pub sTransportOption: Option<String>,
// //     pub iOriginalSourceID: Option<i32>,
// //     pub iOriginalAssignedLoginID: Option<i32>,
// //     pub iOriginalCreatedByLoginID: Option<i32>,
// //     pub dtLastUpdated: Option<NaiveDateTime>,
// //     pub dealer_iMileage: Option<i32>,
// //     pub sHangTag: Option<String>,
// // }
