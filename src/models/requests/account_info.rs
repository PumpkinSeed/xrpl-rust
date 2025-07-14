use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Request};

/// This request retrieves information about an account, its
/// activity, and its XRP balance. All information retrieved
/// is relative to a particular version of the ledger.
///
/// See Account Info:
/// `<https://xrpl.org/account_info.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountInfo {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// A unique identifier for the account, most commonly the
    /// account's Address.
    pub account: String,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    /// If true, then the account field only accepts a public
    /// key or XRP Ledger address. Otherwise, account can be
    /// a secret or passphrase (not recommended).
    /// The default is false.
    pub strict: Option<bool>,
    /// If true, and the FeeEscalation amendment is enabled,
    /// also returns stats about queued transactions associated
    /// with this account. Can only be used when querying for the
    /// data from the current open ledger. New in: rippled 0.33.0
    /// Not available from servers in Reporting Mode.
    pub queue: Option<bool>,
    /// If true, and the MultiSign amendment is enabled, also
    /// returns any SignerList objects associated with this account.
    pub signer_lists: Option<bool>,
}

impl Model for AccountInfo {}

impl Request for AccountInfo {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl AccountInfo {
    pub fn new(
        id: Option<String>,
        account: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
        strict: Option<bool>,
        queue: Option<bool>,
        signer_lists: Option<bool>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::AccountInfo,
                id,
            },
            account,
            ledger_lookup: if ledger_hash.is_some() || ledger_index.is_some() {
                Some(LookupByLedgerRequest {
                    ledger_hash,
                    ledger_index,
                })
            } else {
                None
            },
            strict,
            queue,
            signer_lists,
        }
    }
}
