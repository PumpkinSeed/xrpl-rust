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
pub mod exceptions;
pub mod fee;
pub mod gateway_balances;
pub mod ledger;
pub mod ledger_closed;
pub mod ledger_current;
pub mod ledger_data;
pub mod ledger_entry;
pub mod manifest;
pub mod metadata;
pub mod nft_buy_offers;
pub mod nft_history;
pub mod nft_info;
pub mod nft_offer;
pub mod nft_sell_offers;
pub mod nftoken;
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

use super::{requests::XRPLRequest, Amount, XRPLModelException, XRPLModelResult};
use alloc::{
    format,
    string::{String, ToString},
};
use core::convert::{TryFrom, TryInto};
use exceptions::XRPLResultException;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{value::Index, Map, Value};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NftOffer {
    pub amount: Amount,
    pub flags: u32,
    pub nft_offer_index: String,
    pub owner: String,
    pub destination: Option<String>,
    pub expiration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XRPLOtherResult(Value);

impl TryFrom<XRPLResult> for XRPLOtherResult {
    type Error = XRPLModelException;

    fn try_from(result: XRPLResult) -> XRPLModelResult<Self> {
        match result {
            XRPLResult::Other(value) => Ok(value),
            res => Err(XRPLResultException::UnexpectedResultType(
                "Other".to_string(),
                res.get_name(),
            )
            .into()),
        }
    }
}

impl From<Value> for XRPLOtherResult {
    fn from(value: Value) -> Self {
        XRPLOtherResult(value)
    }
}

impl From<XRPLOtherResult> for Value {
    fn from(val: XRPLOtherResult) -> Self {
        val.0
    }
}

impl XRPLOtherResult {
    pub fn get(&self, index: impl Index) -> Option<&Value> {
        self.0.get(index)
    }

    pub fn try_get_typed<I, T>(&self, index: I) -> XRPLModelResult<T>
    where
        I: Index,
        T: DeserializeOwned,
    {
        let value = self
            .0
            .get(index)
            .ok_or(XRPLResultException::IndexNotFound)?;

        Ok(serde_json::from_value(value.clone())?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum XRPLResult {
    AccountChannels(account_channels::AccountChannels),
    AccountInfo(account_info::AccountInfoVersionMap),
    AccountCurrencies(account_currencies::AccountCurrencies),
    AccountLines(account_lines::AccountLines),
    AccountObjects(account_objects::AccountObjects),
    AccountNfts(account_nfts::AccountNfts),
    AccountOffers(account_offers::AccountOffers),
    AccountTx(account_tx::AccountTxVersionMap),
    AMMInfo(amm_info::AMMInfo),
    BookOffers(book_offers::BookOffers),
    ChannelAuthorize(channel_authorize::ChannelAuthorize),
    ChannelVerify(channel_verify::ChannelVerify),
    DepositAuthorized(deposit_authorize::DepositAuthorized),
    Fee(fee::Fee),
    GatewayBalances(gateway_balances::GatewayBalances),
    Ledger(ledger::Ledger),
    LedgerClosed(ledger_closed::LedgerClosed),
    LedgerCurrent(ledger_current::LedgerCurrent),
    LedgerData(ledger_data::LedgerData),
    LedgerEntry(ledger_entry::LedgerEntry),
    Manifest(manifest::Manifest),
    NFTInfo(nft_info::NFTInfo),
    NFTBuyOffers(nft_buy_offers::NFTBuyOffers),
    NFTSellOffers(nft_sell_offers::NFTSellOffers),
    NFTokenMintResult(nftoken::NFTokenMintResult),
    NoRippleCheck(no_ripple_check::NoRippleCheck),
    PathFind(path_find::PathFind),
    Random(random::Random),
    RipplePathFind(ripple_path_find::RipplePathFind),
    ServerInfo(server_info::ServerInfo),
    ServerState(server_state::ServerState),
    Submit(submit::Submit),
    SubmitMultisigned(submit_multisigned::SubmitMultisigned),
    TransactionEntry(transaction_entry::TransactionEntry),
    Tx(tx::TxVersionMap),
    Subscribe(subscribe::Subscribe),
    Unsubscribe(unsubscribe::Unsubscribe),
    Ping(ping::Ping),
    Other(XRPLOtherResult),
}

macro_rules! impl_from_result_no_lt {
    ($module_name:ident, $variant:ident) => {
        impl From<$module_name::$variant> for XRPLResult {
            fn from(value: $module_name::$variant) -> Self {
                XRPLResult::$variant(value)
            }
        }
    };
}

impl_from_result_no_lt!(account_channels, AccountChannels);
impl_from_result_no_lt!(account_currencies, AccountCurrencies);
impl_from_result_no_lt!(account_lines, AccountLines);
impl_from_result_no_lt!(account_objects, AccountObjects);
impl_from_result_no_lt!(account_nfts, AccountNfts);
impl_from_result_no_lt!(account_offers, AccountOffers);
impl_from_result_no_lt!(amm_info, AMMInfo);
impl_from_result_no_lt!(book_offers, BookOffers);
impl_from_result_no_lt!(channel_authorize, ChannelAuthorize);
impl_from_result_no_lt!(channel_verify, ChannelVerify);
impl_from_result_no_lt!(deposit_authorize, DepositAuthorized);
impl_from_result_no_lt!(fee, Fee);
impl_from_result_no_lt!(gateway_balances, GatewayBalances);
impl_from_result_no_lt!(ledger, Ledger);
impl_from_result_no_lt!(ledger_closed, LedgerClosed);
impl_from_result_no_lt!(ledger_current, LedgerCurrent);
impl_from_result_no_lt!(ledger_data, LedgerData);
impl_from_result_no_lt!(ledger_entry, LedgerEntry);
impl_from_result_no_lt!(manifest, Manifest);
impl_from_result_no_lt!(nft_info, NFTInfo);
impl_from_result_no_lt!(nft_buy_offers, NFTBuyOffers);
impl_from_result_no_lt!(nft_sell_offers, NFTSellOffers);
impl_from_result_no_lt!(nftoken, NFTokenMintResult);
impl_from_result_no_lt!(no_ripple_check, NoRippleCheck);
impl_from_result_no_lt!(path_find, PathFind);
impl_from_result_no_lt!(random, Random);
impl_from_result_no_lt!(ripple_path_find, RipplePathFind);
impl_from_result_no_lt!(server_info, ServerInfo);
impl_from_result_no_lt!(server_state, ServerState);
impl_from_result_no_lt!(submit, Submit);
impl_from_result_no_lt!(submit_multisigned, SubmitMultisigned);
impl_from_result_no_lt!(transaction_entry, TransactionEntry);
impl_from_result_no_lt!(ping, Ping);
impl_from_result_no_lt!(subscribe, Subscribe);
impl_from_result_no_lt!(unsubscribe, Unsubscribe);

impl From<Value> for XRPLResult {
    fn from(value: Value) -> Self {
        XRPLResult::Other(XRPLOtherResult(value))
    }
}

impl From<XRPLOtherResult> for XRPLResult {
    fn from(other: XRPLOtherResult) -> Self {
        XRPLResult::Other(other)
    }
}

macro_rules! impl_try_from_result_no_lt {
    ($module_name:ident, $type:ident, $variant:ident) => {
        impl TryFrom<XRPLResult> for $module_name::$type {
            type Error = XRPLModelException;

            fn try_from(result: XRPLResult) -> XRPLModelResult<Self> {
                match result {
                    XRPLResult::$variant(value) => Ok(value),
                    res => Err(XRPLResultException::UnexpectedResultType(
                        stringify!($variant).to_string(),
                        res.get_name(),
                    )
                    .into()),
                }
            }
        }
    };
}

impl_try_from_result_no_lt!(account_channels, AccountChannels, AccountChannels);
impl_try_from_result_no_lt!(account_currencies, AccountCurrencies, AccountCurrencies);
impl_try_from_result_no_lt!(account_lines, AccountLines, AccountLines);
impl_try_from_result_no_lt!(account_objects, AccountObjects, AccountObjects);
impl_try_from_result_no_lt!(account_nfts, AccountNfts, AccountNfts);
impl_try_from_result_no_lt!(account_offers, AccountOffers, AccountOffers);
impl_try_from_result_no_lt!(amm_info, AMMInfo, AMMInfo);
impl_try_from_result_no_lt!(book_offers, BookOffers, BookOffers);
impl_try_from_result_no_lt!(channel_authorize, ChannelAuthorize, ChannelAuthorize);
impl_try_from_result_no_lt!(channel_verify, ChannelVerify, ChannelVerify);
impl_try_from_result_no_lt!(deposit_authorize, DepositAuthorized, DepositAuthorized);
impl_try_from_result_no_lt!(fee, Fee, Fee);
impl_try_from_result_no_lt!(gateway_balances, GatewayBalances, GatewayBalances);
impl_try_from_result_no_lt!(ledger, Ledger, Ledger);
impl_try_from_result_no_lt!(ledger_closed, LedgerClosed, LedgerClosed);
impl_try_from_result_no_lt!(ledger_current, LedgerCurrent, LedgerCurrent);
impl_try_from_result_no_lt!(ledger_data, LedgerData, LedgerData);
impl_try_from_result_no_lt!(ledger_entry, LedgerEntry, LedgerEntry);
impl_try_from_result_no_lt!(manifest, Manifest, Manifest);
impl_try_from_result_no_lt!(nft_buy_offers, NFTBuyOffers, NFTBuyOffers);
impl_try_from_result_no_lt!(nft_sell_offers, NFTSellOffers, NFTSellOffers);
impl_try_from_result_no_lt!(nftoken, NFTokenMintResult, NFTokenMintResult);
impl_try_from_result_no_lt!(no_ripple_check, NoRippleCheck, NoRippleCheck);
impl_try_from_result_no_lt!(path_find, PathFind, PathFind);
impl_try_from_result_no_lt!(random, Random, Random);
impl_try_from_result_no_lt!(ripple_path_find, RipplePathFind, RipplePathFind);
impl_try_from_result_no_lt!(server_info, ServerInfo, ServerInfo);
impl_try_from_result_no_lt!(server_state, ServerState, ServerState);
impl_try_from_result_no_lt!(submit, Submit, Submit);
impl_try_from_result_no_lt!(submit_multisigned, SubmitMultisigned, SubmitMultisigned);
impl_try_from_result_no_lt!(transaction_entry, TransactionEntry, TransactionEntry);
impl_try_from_result_no_lt!(ping, Ping, Ping);
impl_try_from_result_no_lt!(subscribe, Subscribe, Subscribe);
impl_try_from_result_no_lt!(unsubscribe, Unsubscribe, Unsubscribe);

impl TryInto<Value> for XRPLResult {
    type Error = XRPLModelException;

    fn try_into(self) -> XRPLModelResult<Value> {
        match self {
            XRPLResult::Other(XRPLOtherResult(value)) => Ok(value),
            res => Ok(serde_json::to_value(res)?),
        }
    }
}

impl XRPLResult {
    pub(crate) fn get_name(&self) -> String {
        match self {
            XRPLResult::AccountChannels(_) => "AccountChannels".to_string(),
            XRPLResult::AccountInfo(_) => "AccountInfo".to_string(),
            XRPLResult::AccountCurrencies(_) => "AccountCurrencies".to_string(),
            XRPLResult::AccountLines(_) => "AccountLines".to_string(),
            XRPLResult::AccountObjects(_) => "AccountObjects".to_string(),
            XRPLResult::AccountNfts(_) => "AccountNfts".to_string(),
            XRPLResult::AccountOffers(_) => "AccountOffers".to_string(),
            XRPLResult::AccountTx(_) => "AccountTx".to_string(),
            XRPLResult::AMMInfo(_) => "AMMInfo".to_string(),
            XRPLResult::BookOffers(_) => "BookOffers".to_string(),
            XRPLResult::ChannelAuthorize(_) => "ChannelAuthorize".to_string(),
            XRPLResult::ChannelVerify(_) => "ChannelVerify".to_string(),
            XRPLResult::DepositAuthorized(_) => "DepositAuthorized".to_string(),
            XRPLResult::Fee(_) => "Fee".to_string(),
            XRPLResult::GatewayBalances(_) => "GatewayBalances".to_string(),
            XRPLResult::Ledger(_) => "Ledger".to_string(),
            XRPLResult::LedgerClosed(_) => "LedgerClosed".to_string(),
            XRPLResult::LedgerCurrent(_) => "LedgerCurrent".to_string(),
            XRPLResult::LedgerData(_) => "LedgerData".to_string(),
            XRPLResult::LedgerEntry(_) => "LedgerEntry".to_string(),
            XRPLResult::Manifest(_) => "Manifest".to_string(),
            XRPLResult::NFTInfo(_) => "NFTInfo".to_string(),
            XRPLResult::NFTBuyOffers(_) => "NFTBuyOffers".to_string(),
            XRPLResult::NFTSellOffers(_) => "NFTSellOffers".to_string(),
            XRPLResult::NFTokenMintResult(_) => "NFTokenMintResult".to_string(),
            XRPLResult::NoRippleCheck(_) => "NoRippleCheck".to_string(),
            XRPLResult::PathFind(_) => "PathFind".to_string(),
            XRPLResult::Ping(_) => "Ping".to_string(),
            XRPLResult::Random(_) => "Random".to_string(),
            XRPLResult::RipplePathFind(_) => "RipplePathFind".to_string(),
            XRPLResult::ServerInfo(_) => "ServerInfo".to_string(),
            XRPLResult::ServerState(_) => "ServerState".to_string(),
            XRPLResult::Submit(_) => "Submit".to_string(),
            XRPLResult::SubmitMultisigned(_) => "SubmitMultisigned".to_string(),
            XRPLResult::TransactionEntry(_) => "TransactionEntry".to_string(),
            XRPLResult::Subscribe(_) => "Subscribe".to_string(),
            XRPLResult::Tx(_) => "Tx".to_string(),
            XRPLResult::Unsubscribe(_) => "Unsubscribe".to_string(),
            XRPLResult::Other(_) => "Other".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    Response,
    LedgerClosed,
    Transaction,
}

#[derive(Debug, Clone, Serialize)]
pub struct XRPLResponse<T: Clone + DeserializeOwned + Serialize> {
    pub id: Option<String>,
    pub error: Option<String>,
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
    pub forwarded: Option<bool>,
    pub request: Option<XRPLRequest>,
    pub result: Option<T>,
    pub status: Option<ResponseStatus>,
    pub r#type: Option<ResponseType>,
    pub warning: Option<String>,
    pub warnings: Option<Vec<XRPLWarning>>,
}

fn is_subscription_stream_item(item: &Map<String, Value>) -> bool {
    item.get("result").is_none() && item.get("error_code").is_none()
}

impl<'de, T: Clone + DeserializeOwned + Serialize> Deserialize<'de> for XRPLResponse<T> {
    fn deserialize<D>(deserializer: D) -> XRPLModelResult<XRPLResponse<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: add validation for fields that can not coexist in the same response
        let mut map = serde_json::Map::deserialize(deserializer)?;
        if map.is_empty() {
            return Err(serde::de::Error::custom("Empty response"));
        }
        if is_subscription_stream_item(&map) {
            let map_as_value = Value::Object(map);
            Ok(XRPLResponse {
                id: None,
                error: None,
                error_code: None,
                error_message: None,
                forwarded: None,
                request: None,
                result: serde_json::from_value(map_as_value).map_err(serde::de::Error::custom)?,
                status: None,
                r#type: None,
                warning: None,
                warnings: None,
            })
        } else {
            let mut response = XRPLResponse {
                id: map
                    .remove("id")
                    .and_then(|v| serde_json::from_value(v).ok()),
                error: map
                    .remove("error")
                    .and_then(|v| serde_json::from_value(v).ok()),
                error_code: map
                    .remove("error_code")
                    .and_then(|v| serde_json::from_value(v).ok()),
                error_message: map
                    .remove("error_message")
                    .and_then(|v| serde_json::from_value(v).ok()),
                forwarded: map.remove("forwarded").and_then(|v| v.as_bool()),
                request: map
                    .remove("request")
                    .and_then(|v| serde_json::from_value(v).ok()),
                result: None,
                status: map
                    .remove("status")
                    .and_then(|v| serde_json::from_value(v).ok()),
                r#type: map
                    .remove("type")
                    .and_then(|v| serde_json::from_value(v).ok()),
                warning: map
                    .remove("warning")
                    .and_then(|v| serde_json::from_value(v).ok()),
                warnings: map
                    .remove("warnings")
                    .and_then(|v| serde_json::from_value(v).ok()),
            };

            let result_serde_value = map.remove("result");
            match result_serde_value {
                Some(v) => {
                    response.result = match T::deserialize(&v) {
                        Ok(data) => Some(data),
                        Err(e) => return Err(serde::de::Error::custom(format!("{}", e))),
                    };
                }
                None => {
                    // If the result is not present, we can still return the response
                    response.result = None;
                }
            }

            Ok(response)
        }
    }
}

impl<T: Clone + DeserializeOwned + Serialize> XRPLResponse<T> {
    pub fn is_success(&self) -> bool {
        if let Some(status) = &self.status {
            status == &ResponseStatus::Success
        } else if let Some(result) = &self.result {
            match serde_json::to_value(result) {
                Ok(value) => match value.get("status") {
                    Some(Value::String(status)) => status == "success",
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XRPLWarning {
    pub id: String,
    pub message: String,
    pub forwarded: Option<bool>,
}
