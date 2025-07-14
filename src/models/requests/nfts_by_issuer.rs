use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::Model;

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Marker, Request, RequestMethod};

/// The `nfts_by_issuer` method retrieves all of the NFTokens
/// issued by an account
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct NFTsByIssuer {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    /// The unique identifier for an account that issues NFTokens
    /// The request returns NFTokens issued by this account.
    pub issuer: String,
    pub limit: Option<u32>,
    pub marker: Option<Marker>,
    pub nft_taxon: Option<u64>,
}

impl Model for NFTsByIssuer {}

impl Request for NFTsByIssuer {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl NFTsByIssuer {
    pub fn new(
        id: Option<String>,
        issuer: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
        limit: Option<u32>,
        marker: Option<Marker>,
        nft_taxon: Option<u64>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::NFTsByIssuer,
                id,
            },
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
            issuer,
            limit,
            marker,
            nft_taxon,
        }
    }
}
