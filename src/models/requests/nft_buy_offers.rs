use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Marker, Request};

/// This method retrieves all of buy offers for the specified NFToken.
///
/// See Nft Buy Offers:
/// `<https://xrpl.org/nft_buy_offers.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct NftBuyOffers {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// The unique identifier of a NFToken object.
    pub nft_id: String,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    /// Limit the number of NFT buy offers to retrieve.
    /// This value cannot be lower than 50 or more than 500.
    /// The default is 250.
    pub limit: Option<u16>,
    /// Value from a previous paginated response.
    /// Resume retrieving data where that response left off.
    pub marker: Option<Marker>,
}

impl Model for NftBuyOffers {}

impl Request for NftBuyOffers {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl NftBuyOffers {
    pub fn new(
        id: Option<String>,
        nft_id: String,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
        limit: Option<u16>,
        marker: Option<Marker>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::NFTBuyOffers,
                id,
            },
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
            nft_id,
            limit,
            marker,
        }
    }
}
