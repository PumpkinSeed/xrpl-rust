use crate::models::FlagCollection;
use crate::models::Model;
use crate::models::{ledger::objects::LedgerEntryType, NoFlags};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

/// The `DirectoryNode` object type provides a list of links to other objects in the ledger's state
/// tree. A single conceptual Directory　takes the form of a doubly linked list, with one or more
/// `DirectoryNode` objects each containing up to 32 IDs of other objects. The first object is called
/// the root of the directory, and all objects other than the root object can be added or deleted
/// as necessary.
///
/// There are two kinds of Directories:
/// - `Owner` directories list other objects owned by an account, such as `RippleState` (trust line)
/// or `Offer` objects.
/// - `Offer` directories list the offers available in the decentralized exchange. A single `Offer`
/// directory contains all the offers that have the same exchange rate for the same token.
///
/// `<https://xrpl.org/directorynode.html#directorynode>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DirectoryNode {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the DirectoryNode model.
    //
    // See DirectoryNode fields:
    // `<https://xrpl.org/directorynode.html#directorynode-fields>`
    /// (`Offer` Directories only) DEPRECATED. Do not use.
    pub exchange_rate: Option<String>,
    /// The contents of this `Directory`: an array of IDs of other objects.
    pub indexes: Vec<String>,
    /// The ID of root object for this directory.
    pub root_index: String,
    /// If this `Directory` consists of multiple pages, this ID links to the next object in the chain,
    /// wrapping around at the end.
    pub index_next: Option<u64>,
    /// If this `Directory` consists of multiple pages, this ID links to the previous object in the
    /// chain, wrapping around at the beginning.
    pub index_previous: Option<u64>,
    /// (Owner Directories only) The address of the account that owns the objects in this directory.
    pub owner: Option<String>,
    /// (`Offer` `Directories` only) The currency code of the `TakerGets` amount from the offers in this
    /// directory.
    pub taker_gets_currency: Option<String>,
    /// (`Offer` `Directories` only) The currency code of the `TakerPays` amount from the offers in this
    /// (`Offer` `Directories` only) The issuer of the `TakerGets` amount from the offers in this
    /// directory.
    pub taker_gets_issuer: Option<String>,
    /// directory.
    pub taker_pays_currency: Option<String>,
    /// (`Offer` `Directories` only) The issuer of the `TakerPays` amount from the offers in this
    /// directory.
    pub taker_pays_issuer: Option<String>,
}

impl Model for DirectoryNode {}

impl LedgerObject<NoFlags> for DirectoryNode {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl DirectoryNode {
    pub fn new(
        index: Option<String>,
        ledger_index: Option<String>,
        indexes: Vec<String>,
        root_index: String,
        exchange_rate: Option<String>,
        index_next: Option<u64>,
        index_previous: Option<u64>,
        owner: Option<String>,
        taker_gets_currency: Option<String>,
        taker_gets_issuer: Option<String>,
        taker_pays_currency: Option<String>,
        taker_pays_issuer: Option<String>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                FlagCollection::default(),
                LedgerEntryType::DirectoryNode,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            exchange_rate,
            indexes,
            root_index,
            index_next,
            index_previous,
            owner,
            taker_gets_currency,
            taker_gets_issuer,
            taker_pays_currency,
            taker_pays_issuer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serde() {
        let directory_node = DirectoryNode::new(
            Some("1BBEF97EDE88D40CEE2ADE6FEF121166AFE80D99EBADB01A4F069BA8FF484000".to_string()),
            None,
            vec!["AD7EAE148287EF12D213A251015F86E6D4BD34B3C4A0A1ED9A17198373F908AD".to_string()],
            "1BBEF97EDE88D40CEE2ADE6FEF121166AFE80D99EBADB01A4F069BA8FF484000".to_string(),
            Some("4F069BA8FF484000".to_string()),
            None,
            None,
            None,
            Some("0000000000000000000000000000000000000000".to_string()),
            Some("0000000000000000000000000000000000000000".to_string()),
            Some("0000000000000000000000004A50590000000000".to_string()),
            Some("5BBC0F22F61D9224A110650CFE21CC0C4BE13098".to_string()),
        );
        let serialized = serde_json::to_string(&directory_node).unwrap();

        let deserialized: DirectoryNode = serde_json::from_str(&serialized).unwrap();

        assert_eq!(directory_node, deserialized);
    }
}
