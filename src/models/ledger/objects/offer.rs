use crate::models::ledger::objects::LedgerEntryType;
use crate::models::FlagCollection;
use crate::models::{amount::Amount, Model};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{AsRefStr, Display, EnumIter};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum OfferFlag {
    /// The object was placed as a passive Offer.
    LsfPassive = 0x00010000,
    /// The object was placed as a sell Offer.
    LsfSell = 0x00020000,
}

/// The Offer ledger entry describes an Offer to exchange currencies in the XRP Ledger's
/// decentralized exchange. (In finance, this is more traditionally known as an order.)
/// An OfferCreate transaction only creates an Offer entry in the ledger when the Offer
/// cannot be fully executed immediately by consuming other Offers already in the ledger.
///
/// `<https://xrpl.org/offer.html#offer>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Offer {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<OfferFlag>,
    // The custom fields for the Offer model.
    //
    // See Offer fields:
    // `<https://xrpl.org/offer.html#offer-fields>`
    /// The address of the account that owns this `Offer`.
    pub account: String,
    /// The ID of the `Offer Directory` that links to this Offer.
    pub book_directory: String,
    /// A hint indicating which page of the offer directory links to this object, in case
    /// the directory consists of multiple pages.
    pub book_node: String,
    /// A hint indicating which page of the owner directory links to this object, in case
    /// the directory consists of multiple pages.
    pub owner_node: String,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    /// The index of the ledger that contains the transaction that most recently modified
    /// this object.
    pub previous_txn_lgr_seq: u32,
    /// The `Sequence` value of the `OfferCreate` transaction that created this `Offer` object.
    /// Used in combination with the `Account` to identify this `Offer`.
    pub sequence: u32,
    /// The remaining amount and type of currency being provided by the `Offer` creator.
    pub taker_gets: Amount,
    /// The remaining amount and type of currency requested by the `Offer` creator.
    pub taker_pays: Amount,
    /// Indicates the time after which this Offer is considered unfunded.
    pub expiration: Option<u32>,
}

impl Model for Offer {}

impl LedgerObject<OfferFlag> for Offer {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl Offer {
    pub fn new(
        flags: FlagCollection<OfferFlag>,
        index: Option<String>,
        ledger_index: Option<String>,
        account: String,
        book_directory: String,
        book_node: String,
        owner_node: String,
        previous_txn_id: String,
        previous_txn_lgr_seq: u32,
        sequence: u32,
        taker_gets: Amount,
        taker_pays: Amount,
        expiration: Option<u32>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                flags,
                LedgerEntryType::Offer,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            account,
            book_directory,
            book_node,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            sequence,
            taker_gets,
            taker_pays,
            expiration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::amount::IssuedCurrencyAmount;
    use alloc::vec;

    #[test]
    fn test_serde() {
        let offer = Offer::new(
            vec![OfferFlag::LsfSell].into(),
            Some("96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797".to_string()),
            None,
            "rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt".to_string(),
            "ACC27DE91DBA86FC509069EAF4BC511D73128B780F2E54BF5E07A369E2446000".to_string(),
            "0000000000000000".to_string(),
            "0000000000000000".to_string(),
            "F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF".to_string(),
            14524914,
            866,
            Amount::IssuedCurrencyAmount(IssuedCurrencyAmount::new(
                "XAG".into(),
                "r9Dr5xwkeLegBeXq6ujinjSBLQzQ1zQGjH".into(),
                "37".into(),
            )),
            Amount::XRPAmount("79550000000".into()),
            None,
        );
        let serialized = serde_json::to_string(&offer).unwrap();

        let deserialized: Offer = serde_json::from_str(&serialized).unwrap();

        assert_eq!(offer, deserialized);
    }
}
