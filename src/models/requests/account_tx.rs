use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Marker, Request};

/// This request retrieves from the ledger a list of
/// transactions that involved the specified account.
///
/// See Account Tx:
/// `<https://xrpl.org/account_tx.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountTx {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// A unique identifier for the account, most commonly the
    /// account's address.
    pub account: String,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    /// Defaults to false. If set to true, returns transactions
    /// as hex strings instead of JSON.
    pub binary: Option<bool>,
    /// Defaults to false. If set to true, returns values indexed
    /// with the oldest ledger first. Otherwise, the results are
    /// indexed with the newest ledger first.
    /// (Each page of results may not be internally ordered, but
    /// the pages are overall ordered.)
    pub forward: Option<bool>,
    /// Use to specify the earliest ledger to include transactions
    /// from. A value of -1 instructs the server to use the earliest
    /// validated ledger version available.
    pub ledger_index_min: Option<u32>,
    /// Use to specify the most recent ledger to include transactions
    /// from. A value of -1 instructs the server to use the most
    /// recent validated ledger version available.
    pub ledger_index_max: Option<u32>,
    /// Default varies. Limit the number of transactions to retrieve.
    /// The server is not required to honor this value.
    pub limit: Option<u16>,
    /// Value from a previous paginated response. Resume retrieving
    /// data where that response left off. This value is stable even
    /// if there is a change in the server's range of available
    /// ledgers.
    pub marker: Option<Marker>,
}

impl Model for AccountTx {}

impl Request for AccountTx {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl AccountTx {
    pub fn new(
        id: Option<String>,
        account: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
        binary: Option<bool>,
        forward: Option<bool>,
        ledger_index_min: Option<u32>,
        ledger_index_max: Option<u32>,
        limit: Option<u16>,
        marker: Option<Marker>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::AccountTx,
                id,
            },
            account,
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
            binary,
            forward,
            ledger_index_min,
            ledger_index_max,
            limit,
            marker,
        }
    }
}
