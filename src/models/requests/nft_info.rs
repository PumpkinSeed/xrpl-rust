use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::Model;

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Request, RequestMethod};

/// The `nft_info` method retrieves all the information about the
/// NFToken
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct NFTInfo {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    /// The unique identifier of an NFToken.
    /// The request returns past transactions of this NFToken.
    pub nft_id: String,
}

impl Model for NFTInfo {}

impl Request for NFTInfo {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl NFTInfo {
    pub fn new(
        id: Option<String>,
        nft_id: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::NFTInfo,
                id,
            },
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
            nft_id,
        }
    }
}
