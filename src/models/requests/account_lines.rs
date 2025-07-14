use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Request};

/// This request returns information about an account's trust
/// lines, including balances in all non-XRP currencies and
/// assets. All information retrieved is relative to a particular
/// version of the ledger.
///
/// See Account Lines:
/// `<https://xrpl.org/account_lines.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountLines {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// A unique identifier for the account, most commonly the
    /// account's Address.
    pub account: String,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    /// Limit the number of trust lines to retrieve. The server
    /// is not required to honor this value. Must be within the
    /// inclusive range 10 to 400.
    pub limit: Option<u16>,
    /// The Address of a second account. If provided, show only
    /// lines of trust connecting the two accounts.
    pub peer: Option<String>,
}

impl Model for AccountLines {}

impl Request for AccountLines {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl AccountLines {
    pub fn new(
        id: Option<String>,
        account: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
        limit: Option<u16>,
        peer: Option<String>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::AccountLines,
                id,
            },
            account,
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
            limit,
            peer,
        }
    }
}
