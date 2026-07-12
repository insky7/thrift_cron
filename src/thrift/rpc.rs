use super::autogen::{MessagingServiceSyncClient, SystemServiceSyncClient, TcpaServiceSyncClient};
use crate::thrift::{
    AlertServiceSyncClient,
    autogen::{
        AlertMessageServiceSyncClient, IndustryServiceSyncClient, PromiseServiceSyncClient,
        TranscriptServiceSyncClient,
    },
};
use std::io::{Read, Write};
use thrift::{
    protocol::{TBinaryInputProtocol, TBinaryOutputProtocol, TMultiplexedOutputProtocol},
    transport::{TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel},
};

// todo can this be async, is it worth it?
/// RPC setup for rust
/// First we need to create a multiplexed client in order to order our service names
pub fn create_multiplexed_client<C, F>(
    service_name: &'static str,
    client_factory: F,
) -> thrift::Result<C>
where
    F: FnOnce(
        TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
        TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
    ) -> C,
{
    let mut channel = TTcpChannel::new();

    // todo dot envy var to handle UAT vs Prod vs QA thrift server ip's
    channel.open("REDACTED_THRIFT_HOST:9090")?;

    let (i_chan, o_chan) = channel.split()?;

    let i_tran = TBufferedReadTransport::new(Box::new(i_chan) as Box<dyn Read>);
    let o_tran = TBufferedWriteTransport::new(Box::new(o_chan) as Box<dyn Write>);

    let i_prot = TBinaryInputProtocol::new(i_tran, true);
    let o_prot = TBinaryOutputProtocol::new(o_tran, true);
    let multiplexed = TMultiplexedOutputProtocol::new(service_name, o_prot);

    Ok(client_factory(i_prot, multiplexed))
}

pub fn create_multiplexed_client_v2<C, F, A>(
    service_name: &'static str,
    remote_address: A,
    client_factory: F,
) -> thrift::Result<C>
where
    F: FnOnce(
        TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
        TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
    ) -> C,
    A: std::net::ToSocketAddrs,
{
    let mut channel = TTcpChannel::new();

    // todo dot envy var to handle UAT vs Prod vs QA thrift server ip's
    channel.open(remote_address)?;

    let (i_chan, o_chan) = channel.split()?;

    let i_tran = TBufferedReadTransport::new(Box::new(i_chan) as Box<dyn Read>);
    let o_tran = TBufferedWriteTransport::new(Box::new(o_chan) as Box<dyn Write>);

    let i_prot = TBinaryInputProtocol::new(i_tran, true);
    let o_prot = TBinaryOutputProtocol::new(o_tran, true);
    let multiplexed = TMultiplexedOutputProtocol::new(service_name, o_prot);

    Ok(client_factory(i_prot, multiplexed))
}

// sys_config thrift client
pub type SysConfigClient = SystemServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_sys_config_client() -> thrift::Result<SysConfigClient> {
    create_multiplexed_client("System", SystemServiceSyncClient::new)
}
// tcpa thrift client
pub type TcpaClient = TcpaServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_tcpa_client() -> thrift::Result<TcpaClient> {
    create_multiplexed_client("Tcpa", TcpaServiceSyncClient::new)
}
// industry thrift client
pub type IndustryClient = IndustryServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_industry_client() -> thrift::Result<IndustryClient> {
    create_multiplexed_client("Industry", IndustryServiceSyncClient::new)
}

// messenger thrift client
pub type MessengerClient = MessagingServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_thrift_messenger_client() -> thrift::Result<MessengerClient> {
    create_multiplexed_client("Messaging", MessagingServiceSyncClient::new)
}
// transcript thrift client (for saving)
pub type TranscriptClient = TranscriptServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_thrift_transcript_client() -> thrift::Result<TranscriptClient> {
    create_multiplexed_client("Transcript", TranscriptServiceSyncClient::new)
}
pub type AlertClient = AlertServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_alert_client() -> thrift::Result<AlertClient> {
    create_multiplexed_client("Alert", AlertServiceSyncClient::new)
}
pub type AlertMessageClient = AlertMessageServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_alertmessage_client() -> thrift::Result<AlertMessageClient> {
    create_multiplexed_client("AlertMessage", AlertMessageServiceSyncClient::new)
}
pub type ModelPromiseClient = PromiseServiceSyncClient<
    TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
    TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
>;
pub fn create_modelpromise_client() -> thrift::Result<ModelPromiseClient> {
    create_multiplexed_client("Promise", PromiseServiceSyncClient::new)
}
// pub fn create_sentiments_client() -> thrift::Result<
//     PromiseServiceSyncClient<
//         TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
//         TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
//     >,
// > {
//     create_multiplexed_client("AlertMessage", PromiseServiceSyncClient::new)
// }
// pub fn create_messagebroker_client() -> thrift::Result<
//     PromiseServiceSyncClient<
//         TBinaryInputProtocol<TBufferedReadTransport<Box<dyn Read>>>,
//         TMultiplexedOutputProtocol<TBinaryOutputProtocol<TBufferedWriteTransport<Box<dyn Write>>>>,
//     >,
// > {
//     create_multiplexed_client("AlertMessage", PromiseServiceSyncClient::new)
// }
