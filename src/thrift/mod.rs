use std::sync::LazyLock;

mod autogen;
pub mod rpc;

pub use autogen::{
    AlertMessageServiceSyncClient, AlertServiceSyncClient, AlertType, CompanyHours,
    CompanySchedule, IndustryID, MessagingServiceSyncClient, MobileMessage,
    PromiseServiceSyncClient, TAlertMessageServiceSyncClient, TAlertServiceSyncClient,
    TMessagingServiceSyncClient, TPromiseServiceSyncClient, TSystemServiceSyncClient,
    TTcpaServiceSyncClient, TTranscriptServiceSyncClient, TcpaServiceSyncClient, Transcript,
    TranscriptDest, TranscriptDestType, TranscriptMessageType, TranscriptServiceSyncClient,
    TranscriptSource, TranscriptSourceType, UserFlag,
};

use crate::thrift::{
    autogen::{IndustryServiceSyncClient, SystemServiceSyncClient},
    rpc::{
        AlertClient, AlertMessageClient, IndustryClient, MessengerClient, ModelPromiseClient,
        SysConfigClient, TcpaClient, TranscriptClient, create_multiplexed_client_v2,
    },
};

const DEFAULT_OPEN: &str = "00:00";
const DEFAULT_CLOSE: &str = "23:59";

#[rustfmt::skip]
pub static DEFAULT_COMPANY_SCHEDULE: LazyLock<CompanySchedule> =
    LazyLock::new(|| CompanySchedule {
        sun: Some(CompanyHours::new(DEFAULT_OPEN.to_string(), DEFAULT_CLOSE.to_string())),
        mon: Some(CompanyHours::new(DEFAULT_OPEN.to_string(), DEFAULT_CLOSE.to_string())),
        tue: Some(CompanyHours::new(DEFAULT_OPEN.to_string(), DEFAULT_CLOSE.to_string())),
        wed: Some(CompanyHours::new(DEFAULT_OPEN.to_string(), DEFAULT_CLOSE.to_string())),
        thu: Some(CompanyHours::new(DEFAULT_OPEN.to_string(), DEFAULT_CLOSE.to_string())),
        fri: Some(CompanyHours::new(DEFAULT_OPEN.to_string(), DEFAULT_CLOSE.to_string())),
        sat: Some(CompanyHours::new(DEFAULT_OPEN.to_string(), DEFAULT_CLOSE.to_string())),
    });

pub struct ThriftClientContext {
    pub sys_config_client: SysConfigClient,
    pub tcpa_client: TcpaClient,
    pub industry_client: IndustryClient,
    pub messenger_client: MessengerClient,
    pub transcript_client: TranscriptClient,
    pub alert_client: AlertClient,
    pub alertmessage_client: AlertMessageClient,
    pub modelpromise_client: ModelPromiseClient,
}

impl ThriftClientContext {
    pub fn new(remote_address: &str) -> thrift::Result<Self> {
        let sys_config_client =
            create_multiplexed_client_v2("System", remote_address, SystemServiceSyncClient::new)?;

        let tcpa_client =
            create_multiplexed_client_v2("Tcpa", remote_address, TcpaServiceSyncClient::new)?;

        let industry_client = create_multiplexed_client_v2(
            "Industry",
            remote_address,
            IndustryServiceSyncClient::new,
        )?;

        let messenger_client = create_multiplexed_client_v2(
            "Messaging",
            remote_address,
            MessagingServiceSyncClient::new,
        )?;

        let transcript_client = create_multiplexed_client_v2(
            "Transcript",
            remote_address,
            TranscriptServiceSyncClient::new,
        )?;

        let alert_client =
            create_multiplexed_client_v2("Alert", remote_address, AlertServiceSyncClient::new)?;

        let alertmessage_client = create_multiplexed_client_v2(
            "AlertMessage",
            remote_address,
            AlertMessageServiceSyncClient::new,
        )?;

        let modelpromise_client =
            create_multiplexed_client_v2("Promise", remote_address, PromiseServiceSyncClient::new)?;

        Ok(Self {
            sys_config_client,
            tcpa_client,
            industry_client,
            messenger_client,
            transcript_client,
            alert_client,
            alertmessage_client,
            modelpromise_client,
        })
    }

    #[cfg(test)]
    pub fn new_for_tests() -> Self {
        let remote_address = "REDACTED_THRIFT_HOST:9090";

        let sys_config_client =
            create_multiplexed_client_v2("System", remote_address, SystemServiceSyncClient::new)
                .unwrap();

        let tcpa_client =
            create_multiplexed_client_v2("Tcpa", remote_address, TcpaServiceSyncClient::new)
                .unwrap();

        let industry_client = create_multiplexed_client_v2(
            "Industry",
            remote_address,
            IndustryServiceSyncClient::new,
        )
        .unwrap();

        let messenger_client = create_multiplexed_client_v2(
            "Messaging",
            remote_address,
            MessagingServiceSyncClient::new,
        )
        .unwrap();

        let transcript_client = create_multiplexed_client_v2(
            "Transcript",
            remote_address,
            TranscriptServiceSyncClient::new,
        )
        .unwrap();

        let alert_client =
            create_multiplexed_client_v2("Alert", remote_address, AlertServiceSyncClient::new)
                .unwrap();

        let alertmessage_client = create_multiplexed_client_v2(
            "AlertMessage",
            remote_address,
            AlertMessageServiceSyncClient::new,
        )
        .unwrap();

        let modelpromise_client =
            create_multiplexed_client_v2("Promise", remote_address, PromiseServiceSyncClient::new)
                .unwrap();

        Self {
            sys_config_client,
            tcpa_client,
            industry_client,
            messenger_client,
            transcript_client,
            alert_client,
            alertmessage_client,
            modelpromise_client,
        }
    }
}
