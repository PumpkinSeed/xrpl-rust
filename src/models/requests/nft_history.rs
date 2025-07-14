use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::Model;

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Marker, Request, RequestMethod};

/// The `nft_history` method retreives a list of transactions that involved the
/// specified NFToken.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct NFTHistory {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    /// The unique identifier of an NFToken.
    /// The request returns past transactions of this NFToken.
    pub nft_id: String,
    pub ledger_index_min: Option<u32>,
    pub ledger_index_max: Option<u32>,
    pub binary: Option<bool>,
    pub forward: Option<bool>,
    pub limit: Option<u32>,
    pub marker: Option<Marker>,
}

impl Model for NFTHistory {}

impl Request for NFTHistory {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl NFTHistory {
    pub fn new(
        id: Option<String>,
        nft_id: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
        ledger_index_min: Option<u32>,
        ledger_index_max: Option<u32>,
        binary: Option<bool>,
        forward: Option<bool>,
        limit: Option<u32>,
        marker: Option<Marker>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::NFTHistory,
                id,
            },
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
            nft_id,
            ledger_index_min,
            ledger_index_max,
            binary,
            forward,
            limit,
            marker,
        }
    }
}
