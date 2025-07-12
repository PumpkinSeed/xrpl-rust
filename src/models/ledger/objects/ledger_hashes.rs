use crate::models::FlagCollection;
use crate::models::Model;
use crate::models::{ledger::objects::LedgerEntryType, NoFlags};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

/// The `LedgerHashes` object type contains a history of prior ledgers that led up to this
/// ledger version, in the form of their hashes. Objects of this ledger type are modified
/// automatically when closing a ledger. The `LedgerHashes` objects exist to make it possible
/// to look up a previous ledger's hash with only the current ledger version and at most one
/// lookup of a previous ledger version.
///
/// There are two kinds of LedgerHashes object. Both types have the same fields.
/// Each ledger version contains:
/// - Exactly one "recent history" LedgerHashes object
/// - A number of "previous history" `LedgerHashes` objects based on the current ledger index.
/// Specifically, the XRP Ledger adds a new "previous history" object every 65536 ledger versions.
///
/// `<https://xrpl.org/ledgerhashes.html#ledgerhashes>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LedgerHashes {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the LedgerHashes model.
    //
    // See LedgerHashes fields:
    // `<https://xrpl.org/ledgerhashes.html#ledgerhashes-fields>`
    /// **DEPRECATED** Do not use.
    pub first_ledger_sequence: u32,
    /// An array of up to 256 ledger hashes. The contents depend on which sub-type of `LedgerHashes`
    /// object this is.
    pub hashes: Vec<String>,
    /// The Ledger Index of the last entry in this object's `Hashes` array.
    pub last_ledger_sequence: u32,
}

impl Model for LedgerHashes {}

impl LedgerObject<NoFlags> for LedgerHashes {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl LedgerHashes {
    pub fn new(
        index: Option<String>,
        ledger_index: Option<String>,
        first_ledger_sequence: u32,
        hashes: Vec<String>,
        last_ledger_sequence: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                FlagCollection::default(),
                LedgerEntryType::LedgerHashes,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            first_ledger_sequence,
            last_ledger_sequence,
            hashes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serde() {
        let ledger_hashes = LedgerHashes::new(
            Some("B4979A36CDC7F3D3D5C31A4EAE2AC7D7209DDA877588B9AFC66799692AB0D66B".to_string()),
            None,
            2,
            vec![
                "D638208ADBD04CBB10DE7B645D3AB4BA31489379411A3A347151702B6401AA78".to_string(),
                "254D690864E418DDD9BCAC93F41B1F53B1AE693FC5FE667CE40205C322D1BE3B".to_string(),
                "A2B31D28905E2DEF926362822BC412B12ABF6942B73B72A32D46ED2ABB7ACCFA".to_string(),
                "AB4014846DF818A4B43D6B1686D0DE0644FE711577C5AB6F0B2A21CCEE280140".to_string(),
                "3383784E82A8BA45F4DD5EF4EE90A1B2D3B4571317DBAC37B859836ADDE644C1".to_string(),
            ],
            33872029,
        );
        let serialized = serde_json::to_string(&ledger_hashes).unwrap();

        let deserialized: LedgerHashes = serde_json::from_str(&serialized).unwrap();

        assert_eq!(ledger_hashes, deserialized);
    }
}
