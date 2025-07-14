use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Request};

/// The deposit_authorized command indicates whether one account
/// is authorized to send payments directly to another.
///
/// See Deposit Authorization:
/// `<https://xrpl.org/depositauth.html#deposit-authorization>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DepositAuthorized {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// The recipient of a possible payment.
    pub destination_account: String,
    /// The sender of a possible payment.
    pub source_account: String,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
}

impl Model for DepositAuthorized {}

impl Request for DepositAuthorized {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl DepositAuthorized {
    pub fn new(
        id: Option<String>,
        destination_account: String,
        source_account: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::DepositAuthorized,
                id,
            },
            source_account,
            destination_account,
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
        }
    }
}
