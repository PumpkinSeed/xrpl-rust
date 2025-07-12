use crate::models::ledger::objects::LedgerEntryType;
use crate::models::FlagCollection;
use crate::models::{amount::Amount, Model};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

use super::{CommonFields, LedgerObject};

#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum NFTokenOfferFlag {
    /// If enabled, the `NFTokenOffer` is a sell offer. Otherwise, the `NFTokenOffer` is a buy offer.
    LsfSellNFToken = 0x00000001,
}

/// The `NFTokenOffer` object represents an offer to buy, sell or transfer an `NFToken` object.
/// The owner of a `NFToken` can use `NFTokenCreateOffer` to start a transaction.
///
/// `<https://xrpl.org/nftokenoffer.html#nftokenoffer>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenOffer {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NFTokenOfferFlag>,
    // The custom fields for the NFTokenOffer model.
    //
    // See NFTokenOffer fields:
    // `<https://xrpl.org/nftokenoffer.html#nftokenoffer-fields>`
    /// Amount expected or offered for the `NFToken`. If the token has the `lsfOnlyXRP` flag set,
    /// the amount must be specified in XRP. Sell offers that specify assets other than XRP
    /// must specify a non-zero amount. Sell offers that specify XRP can be 'free'
    /// (that is, the Amount field can be equal to "0").
    pub amount: Amount,
    /// The `NFTokenID` of the `NFToken` object referenced by this offer.
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: String,
    /// Owner of the account that is creating and owns the offer. Only the current Owner
    /// of an `NFToken` can create an offer to sell an `NFToken`, but any account can create
    /// an offer to buy an NFToken.
    pub owner: String,
    /// Identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    /// Index of the ledger that contains the transaction that most recently modified this object.
    pub previous_txn_lgr_seq: u32,
    /// The `AccountID` for which this offer is intended. If present, only that account can
    /// accept the offer.
    pub destination: Option<String>,
    /// The time after which the offer is no longer active. The value is the number of
    /// seconds since the Ripple Epoch.
    pub expiration: Option<u32>,
    /// Internal bookkeeping, indicating the page inside the token buy or sell offer directory,
    /// as appropriate, where this token is being tracked. This field allows the efficient
    /// deletion of offers.
    #[serde(rename = "NFTokenOfferNode")]
    pub nftoken_offer_node: Option<String>,
    /// Internal bookkeeping, indicating the page inside the owner directory where this token
    /// is being tracked. This field allows the efficient deletion of offers.
    pub owner_node: Option<String>,
}

impl Model for NFTokenOffer {}

impl LedgerObject<NFTokenOfferFlag> for NFTokenOffer {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl NFTokenOffer {
    pub fn new(
        flags: FlagCollection<NFTokenOfferFlag>,
        index: Option<String>,
        ledger_index: Option<String>,
        amount: Amount,
        nftoken_id: String,
        owner: String,
        previous_txn_id: String,
        previous_txn_lgr_seq: u32,
        destination: Option<String>,
        expiration: Option<u32>,
        nftoken_offer_node: Option<String>,
        owner_node: Option<String>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                flags,
                LedgerEntryType::NFTokenOffer,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            amount,
            nftoken_id,
            owner,
            previous_txn_id,
            previous_txn_lgr_seq,
            destination,
            expiration,
            nftoken_offer_node,
            owner_node,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serde() {
        let nftoken_offer = NFTokenOffer::new(
            vec![NFTokenOfferFlag::LsfSellNFToken].into(),
            Some("AEBABA4FAC212BF28E0F9A9C3788A47B085557EC5D1429E7A8266FB859C863B3".to_string()),
            None,
            Amount::XRPAmount("1000000".into()),
            "00081B5825A08C22787716FA031B432EBBC1B101BB54875F0002D2A400000000".to_string(),
            "rhRxL3MNvuKEjWjL7TBbZSDacb8PmzAd7m".to_string(),
            "BFA9BE27383FA315651E26FDE1FA30815C5A5D0544EE10EC33D3E92532993769".to_string(),
            75443565,
            None,
            None,
            Some("0".to_string()),
            Some("17".to_string()),
        );
        let serialized = serde_json::to_string(&nftoken_offer).unwrap();

        let deserialized: NFTokenOffer = serde_json::from_str(&serialized).unwrap();

        assert_eq!(nftoken_offer, deserialized);
    }
}
