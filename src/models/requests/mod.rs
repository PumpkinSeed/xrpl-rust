pub mod account_channels;
pub mod account_currencies;
pub mod account_info;
pub mod account_lines;
pub mod account_nfts;
pub mod account_objects;
pub mod account_offers;
pub mod account_tx;
pub mod amm_info;
pub mod book_offers;
pub mod channel_authorize;
pub mod channel_verify;
pub mod deposit_authorize;
pub mod fee;
pub mod gateway_balances;
pub mod ledger;
pub mod ledger_closed;
pub mod ledger_current;
pub mod ledger_data;
pub mod ledger_entry;
pub mod manifest;
pub mod nft_buy_offers;
pub mod nft_history;
pub mod nft_info;
pub mod nft_sell_offers;
pub mod nfts_by_issuer;
pub mod no_ripple_check;
pub mod path_find;
pub mod ping;
pub mod random;
pub mod ripple_path_find;
pub mod server_info;
pub mod server_state;
pub mod submit;
pub mod submit_multisigned;
pub mod subscribe;
pub mod transaction_entry;
pub mod tx;
pub mod unsubscribe;

use alloc::{borrow::Cow, string::String};
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

/// Represents the different options for the `method`
/// field in a request.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RequestMethod {
    // Account methods
    AccountChannels,
    AccountCurrencies,
    AccountInfo,
    AccountLines,
    AccountNfts,
    AccountObjects,
    AccountOffers,
    AccountTx,
    #[serde(rename = "amm_info")]
    AMMInfo,
    GatewayBalances,
    NoRippleCheck,

    // Transaction methods
    Sign,
    SignFor,
    Submit,
    SubmitMultisigned,
    TransactionEntry,
    Tx,

    // Channel methods
    ChannelAuthorize,
    ChannelVerify,

    // Path methods
    BookOffers,
    DepositAuthorized,
    #[serde(rename = "nft_buy_offers")]
    NFTBuyOffers,
    #[serde(rename = "nft_history")]
    NFTHistory,
    #[serde(rename = "nft_info")]
    NFTInfo,
    #[serde(rename = "nft_sell_offers")]
    NFTSellOffers,
    #[serde(rename = "nfts_by_issuer")]
    NFTsByIssuer,
    PathFind,
    RipplePathFind,

    // Ledger methods
    Ledger,
    LedgerClosed,
    LedgerCurrent,
    LedgerData,
    LedgerEntry,

    // Subscription methods
    Subscribe,
    Unsubscribe,

    // Server info methods
    Fee,
    Manifest,
    ServerInfo,
    ServerState,

    // Utility methods
    Ping,
    Random,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum XRPLRequest {
    AccountChannels(account_channels::AccountChannels),
    AccountCurrencies(account_currencies::AccountCurrencies),
    AccountInfo(account_info::AccountInfo),
    AccountLines(account_lines::AccountLines),
    AccountNfts(account_nfts::AccountNfts),
    AccountObjects(account_objects::AccountObjects),
    AccountOffers(account_offers::AccountOffers),
    AccountTx(account_tx::AccountTx),
    AMMInfo(amm_info::AMMInfo),
    GatewayBalances(gateway_balances::GatewayBalances),
    NoRippleCheck(no_ripple_check::NoRippleCheck),
    Submit(submit::Submit),
    SubmitMultisigned(submit_multisigned::SubmitMultisigned),
    TransactionEntry(transaction_entry::TransactionEntry),
    Tx(tx::Tx),
    ChannelAuthorize(channel_authorize::ChannelAuthorize),
    ChannelVerify(channel_verify::ChannelVerify),
    BookOffers(book_offers::BookOffers),
    DepositAuthorized(deposit_authorize::DepositAuthorized),
    NFTBuyOffers(nft_buy_offers::NftBuyOffers),
    NFTHistory(nft_history::NFTHistory),
    NFTInfo(nft_info::NFTInfo),
    NFTSellOffers(nft_sell_offers::NftSellOffers),
    NFTsByIssuer(nfts_by_issuer::NFTsByIssuer),
    PathFind(path_find::PathFind),
    RipplePathFind(ripple_path_find::RipplePathFind),
    Ledger(ledger::Ledger),
    LedgerClosed(ledger_closed::LedgerClosed),
    LedgerCurrent(ledger_current::LedgerCurrent),
    LedgerData(ledger_data::LedgerData),
    LedgerEntry(ledger_entry::LedgerEntry),
    Subscribe(subscribe::Subscribe),
    Unsubscribe(unsubscribe::Unsubscribe),
    Fee(fee::Fee),
    Manifest(manifest::Manifest),
    ServerInfo(server_info::ServerInfo),
    ServerState(server_state::ServerState),
    Ping(ping::Ping),
    Random(random::Random),
}

impl From<account_channels::AccountChannels> for XRPLRequest {
    fn from(request: account_channels::AccountChannels) -> Self {
        XRPLRequest::AccountChannels(request)
    }
}

impl From<account_currencies::AccountCurrencies> for XRPLRequest {
    fn from(request: account_currencies::AccountCurrencies) -> Self {
        XRPLRequest::AccountCurrencies(request)
    }
}

impl From<account_info::AccountInfo> for XRPLRequest {
    fn from(request: account_info::AccountInfo) -> Self {
        XRPLRequest::AccountInfo(request)
    }
}

impl From<account_lines::AccountLines> for XRPLRequest {
    fn from(request: account_lines::AccountLines) -> Self {
        XRPLRequest::AccountLines(request)
    }
}

impl From<account_nfts::AccountNfts> for XRPLRequest {
    fn from(request: account_nfts::AccountNfts) -> Self {
        XRPLRequest::AccountNfts(request)
    }
}

impl From<account_objects::AccountObjects> for XRPLRequest {
    fn from(request: account_objects::AccountObjects) -> Self {
        XRPLRequest::AccountObjects(request)
    }
}

impl From<account_offers::AccountOffers> for XRPLRequest {
    fn from(request: account_offers::AccountOffers) -> Self {
        XRPLRequest::AccountOffers(request)
    }
}

impl From<account_tx::AccountTx> for XRPLRequest {
    fn from(request: account_tx::AccountTx) -> Self {
        XRPLRequest::AccountTx(request)
    }
}

impl From<amm_info::AMMInfo> for XRPLRequest {
    fn from(request: amm_info::AMMInfo) -> Self {
        XRPLRequest::AMMInfo(request)
    }
}

impl From<gateway_balances::GatewayBalances> for XRPLRequest {
    fn from(request: gateway_balances::GatewayBalances) -> Self {
        XRPLRequest::GatewayBalances(request)
    }
}

impl From<no_ripple_check::NoRippleCheck> for XRPLRequest {
    fn from(request: no_ripple_check::NoRippleCheck) -> Self {
        XRPLRequest::NoRippleCheck(request)
    }
}

impl From<submit::Submit> for XRPLRequest {
    fn from(request: submit::Submit) -> Self {
        XRPLRequest::Submit(request)
    }
}

impl From<submit_multisigned::SubmitMultisigned> for XRPLRequest {
    fn from(request: submit_multisigned::SubmitMultisigned) -> Self {
        XRPLRequest::SubmitMultisigned(request)
    }
}

impl From<transaction_entry::TransactionEntry> for XRPLRequest {
    fn from(request: transaction_entry::TransactionEntry) -> Self {
        XRPLRequest::TransactionEntry(request)
    }
}

impl From<tx::Tx> for XRPLRequest {
    fn from(request: tx::Tx) -> Self {
        XRPLRequest::Tx(request)
    }
}

impl From<channel_authorize::ChannelAuthorize> for XRPLRequest {
    fn from(request: channel_authorize::ChannelAuthorize) -> Self {
        XRPLRequest::ChannelAuthorize(request)
    }
}

impl From<channel_verify::ChannelVerify> for XRPLRequest {
    fn from(request: channel_verify::ChannelVerify) -> Self {
        XRPLRequest::ChannelVerify(request)
    }
}

impl From<book_offers::BookOffers> for XRPLRequest {
    fn from(request: book_offers::BookOffers) -> Self {
        XRPLRequest::BookOffers(request)
    }
}

impl From<deposit_authorize::DepositAuthorized> for XRPLRequest {
    fn from(request: deposit_authorize::DepositAuthorized) -> Self {
        XRPLRequest::DepositAuthorized(request)
    }
}

impl From<nft_buy_offers::NftBuyOffers> for XRPLRequest {
    fn from(request: nft_buy_offers::NftBuyOffers) -> Self {
        XRPLRequest::NFTBuyOffers(request)
    }
}

impl From<nft_sell_offers::NftSellOffers> for XRPLRequest {
    fn from(request: nft_sell_offers::NftSellOffers) -> Self {
        XRPLRequest::NFTSellOffers(request)
    }
}

impl From<path_find::PathFind> for XRPLRequest {
    fn from(request: path_find::PathFind) -> Self {
        XRPLRequest::PathFind(request)
    }
}

impl From<ripple_path_find::RipplePathFind> for XRPLRequest {
    fn from(request: ripple_path_find::RipplePathFind) -> Self {
        XRPLRequest::RipplePathFind(request)
    }
}

impl From<ledger::Ledger> for XRPLRequest {
    fn from(request: ledger::Ledger) -> Self {
        XRPLRequest::Ledger(request)
    }
}

impl From<ledger_closed::LedgerClosed> for XRPLRequest {
    fn from(request: ledger_closed::LedgerClosed) -> Self {
        XRPLRequest::LedgerClosed(request)
    }
}

impl From<ledger_current::LedgerCurrent> for XRPLRequest {
    fn from(request: ledger_current::LedgerCurrent) -> Self {
        XRPLRequest::LedgerCurrent(request)
    }
}

impl From<ledger_data::LedgerData> for XRPLRequest {
    fn from(request: ledger_data::LedgerData) -> Self {
        XRPLRequest::LedgerData(request)
    }
}

impl From<ledger_entry::LedgerEntry> for XRPLRequest {
    fn from(request: ledger_entry::LedgerEntry) -> Self {
        XRPLRequest::LedgerEntry(request)
    }
}

impl From<subscribe::Subscribe> for XRPLRequest {
    fn from(request: subscribe::Subscribe) -> Self {
        XRPLRequest::Subscribe(request)
    }
}

impl From<unsubscribe::Unsubscribe> for XRPLRequest {
    fn from(request: unsubscribe::Unsubscribe) -> Self {
        XRPLRequest::Unsubscribe(request)
    }
}

impl From<fee::Fee> for XRPLRequest {
    fn from(request: fee::Fee) -> Self {
        XRPLRequest::Fee(request)
    }
}

impl From<manifest::Manifest> for XRPLRequest {
    fn from(request: manifest::Manifest) -> Self {
        XRPLRequest::Manifest(request)
    }
}

impl From<server_info::ServerInfo> for XRPLRequest {
    fn from(request: server_info::ServerInfo) -> Self {
        XRPLRequest::ServerInfo(request)
    }
}

impl From<server_state::ServerState> for XRPLRequest {
    fn from(request: server_state::ServerState) -> Self {
        XRPLRequest::ServerState(request)
    }
}

impl From<ping::Ping> for XRPLRequest {
    fn from(request: ping::Ping) -> Self {
        XRPLRequest::Ping(request)
    }
}

impl From<random::Random> for XRPLRequest {
    fn from(request: random::Random) -> Self {
        XRPLRequest::Random(request)
    }
}

impl Request for XRPLRequest {
    fn get_common_fields(&self) -> &CommonFields {
        match self {
            XRPLRequest::AccountChannels(request) => request.get_common_fields(),
            XRPLRequest::AccountCurrencies(request) => request.get_common_fields(),
            XRPLRequest::AccountInfo(request) => request.get_common_fields(),
            XRPLRequest::AccountLines(request) => request.get_common_fields(),
            XRPLRequest::AccountNfts(request) => request.get_common_fields(),
            XRPLRequest::AccountObjects(request) => request.get_common_fields(),
            XRPLRequest::AccountOffers(request) => request.get_common_fields(),
            XRPLRequest::AccountTx(request) => request.get_common_fields(),
            XRPLRequest::AMMInfo(request) => request.get_common_fields(),
            XRPLRequest::GatewayBalances(request) => request.get_common_fields(),
            XRPLRequest::NoRippleCheck(request) => request.get_common_fields(),
            XRPLRequest::Submit(request) => request.get_common_fields(),
            XRPLRequest::SubmitMultisigned(request) => request.get_common_fields(),
            XRPLRequest::TransactionEntry(request) => request.get_common_fields(),
            XRPLRequest::Tx(request) => request.get_common_fields(),
            XRPLRequest::ChannelAuthorize(request) => request.get_common_fields(),
            XRPLRequest::ChannelVerify(request) => request.get_common_fields(),
            XRPLRequest::BookOffers(request) => request.get_common_fields(),
            XRPLRequest::DepositAuthorized(request) => request.get_common_fields(),
            XRPLRequest::NFTBuyOffers(request) => request.get_common_fields(),
            XRPLRequest::NFTHistory(request) => request.get_common_fields(),
            XRPLRequest::NFTInfo(request) => request.get_common_fields(),
            XRPLRequest::NFTSellOffers(request) => request.get_common_fields(),
            XRPLRequest::NFTsByIssuer(request) => request.get_common_fields(),
            XRPLRequest::PathFind(request) => request.get_common_fields(),
            XRPLRequest::RipplePathFind(request) => request.get_common_fields(),
            XRPLRequest::Ledger(request) => request.get_common_fields(),
            XRPLRequest::LedgerClosed(request) => request.get_common_fields(),
            XRPLRequest::LedgerCurrent(request) => request.get_common_fields(),
            XRPLRequest::LedgerData(request) => request.get_common_fields(),
            XRPLRequest::LedgerEntry(request) => request.get_common_fields(),
            XRPLRequest::Subscribe(request) => request.get_common_fields(),
            XRPLRequest::Unsubscribe(request) => request.get_common_fields(),
            XRPLRequest::Fee(request) => request.get_common_fields(),
            XRPLRequest::Manifest(request) => request.get_common_fields(),
            XRPLRequest::ServerInfo(request) => request.get_common_fields(),
            XRPLRequest::ServerState(request) => request.get_common_fields(),
            XRPLRequest::Ping(request) => request.get_common_fields(),
            XRPLRequest::Random(request) => request.get_common_fields(),
        }
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        match self {
            XRPLRequest::AccountChannels(request) => request.get_common_fields_mut(),
            XRPLRequest::AccountCurrencies(request) => request.get_common_fields_mut(),
            XRPLRequest::AccountInfo(request) => request.get_common_fields_mut(),
            XRPLRequest::AccountLines(request) => request.get_common_fields_mut(),
            XRPLRequest::AccountNfts(request) => request.get_common_fields_mut(),
            XRPLRequest::AccountObjects(request) => request.get_common_fields_mut(),
            XRPLRequest::AccountOffers(request) => request.get_common_fields_mut(),
            XRPLRequest::AccountTx(request) => request.get_common_fields_mut(),
            XRPLRequest::AMMInfo(request) => request.get_common_fields_mut(),
            XRPLRequest::GatewayBalances(request) => request.get_common_fields_mut(),
            XRPLRequest::NoRippleCheck(request) => request.get_common_fields_mut(),
            XRPLRequest::Submit(request) => request.get_common_fields_mut(),
            XRPLRequest::SubmitMultisigned(request) => request.get_common_fields_mut(),
            XRPLRequest::TransactionEntry(request) => request.get_common_fields_mut(),
            XRPLRequest::Tx(request) => request.get_common_fields_mut(),
            XRPLRequest::ChannelAuthorize(request) => request.get_common_fields_mut(),
            XRPLRequest::ChannelVerify(request) => request.get_common_fields_mut(),
            XRPLRequest::BookOffers(request) => request.get_common_fields_mut(),
            XRPLRequest::DepositAuthorized(request) => request.get_common_fields_mut(),
            XRPLRequest::NFTBuyOffers(request) => request.get_common_fields_mut(),
            XRPLRequest::NFTHistory(request) => request.get_common_fields_mut(),
            XRPLRequest::NFTInfo(request) => request.get_common_fields_mut(),
            XRPLRequest::NFTSellOffers(request) => request.get_common_fields_mut(),
            XRPLRequest::NFTsByIssuer(request) => request.get_common_fields_mut(),
            XRPLRequest::PathFind(request) => request.get_common_fields_mut(),
            XRPLRequest::RipplePathFind(request) => request.get_common_fields_mut(),
            XRPLRequest::Ledger(request) => request.get_common_fields_mut(),
            XRPLRequest::LedgerClosed(request) => request.get_common_fields_mut(),
            XRPLRequest::LedgerCurrent(request) => request.get_common_fields_mut(),
            XRPLRequest::LedgerData(request) => request.get_common_fields_mut(),
            XRPLRequest::LedgerEntry(request) => request.get_common_fields_mut(),
            XRPLRequest::Subscribe(request) => request.get_common_fields_mut(),
            XRPLRequest::Unsubscribe(request) => request.get_common_fields_mut(),
            XRPLRequest::Fee(request) => request.get_common_fields_mut(),
            XRPLRequest::Manifest(request) => request.get_common_fields_mut(),
            XRPLRequest::ServerInfo(request) => request.get_common_fields_mut(),
            XRPLRequest::ServerState(request) => request.get_common_fields_mut(),
            XRPLRequest::Ping(request) => request.get_common_fields_mut(),
            XRPLRequest::Random(request) => request.get_common_fields_mut(),
        }
    }
}

/// The base fields for all request models.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct CommonFields {
    /// The request method.
    pub command: RequestMethod,
    /// The unique request id.
    pub id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct LookupByLedgerRequest {
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<String>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<LedgerIndex>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LedgerSequenceMarker {
    ledger: u32,
    seq: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Marker{
    Int(u32),
    Str(String),
    Sequence(LedgerSequenceMarker),
}

impl From<u32> for Marker {
    fn from(value: u32) -> Self {
        Marker::Int(value)
    }
}

impl<'a> From<&'a str> for Marker {
    fn from(value: &'a str) -> Self {
        Marker::Str(value.to_string())
    }
}

impl From<String> for Marker{
    fn from(value: String) -> Self {
        Marker::Str(value.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum LedgerIndex {
    Int(u32),
    Str(String),
}

impl From<u32> for LedgerIndex {
    fn from(value: u32) -> Self {
        LedgerIndex::Int(value)
    }
}

impl From<&str> for LedgerIndex {
    fn from(value: &str) -> Self {
        LedgerIndex::Str(value.to_string())
    }
}

impl From<String> for LedgerIndex {
    fn from(value: String) -> Self {
        LedgerIndex::Str(value)
    }
}

/// The base trait for all request models.
/// Used to identify the model as a request.
pub trait Request {
    fn get_common_fields(&self) -> &CommonFields;
    fn get_common_fields_mut(&mut self) -> &mut CommonFields;
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundFaucet {
    pub destination: String,
    pub usage_context: Option<String>,
    pub user_agent: Option<String>,
}
