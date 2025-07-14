use crate::models::FlagCollection;
use crate::models::Model;
use crate::models::{ledger::objects::LedgerEntryType, NoFlags};

use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

/// A `DepositPreauth` object tracks a preauthorization from one account to another.
/// `DepositPreauth` transactions create these objects.
///
/// `<https://xrpl.org/depositpreauth-object.html#depositpreauth>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DepositPreauth {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the DepositPreauth model.
    //
    // See DepositPreauth fields:
    // `<https://xrpl.org/depositpreauth-object.html#depositpreauth-fields>`
    /// The account that granted the preauthorization.
    pub account: String,
    /// The account that received the preauthorization.
    pub authorize: String,
    /// A hint indicating which page of the sender's owner directory links to this object, in case
    /// the directory consists of multiple pages.
    pub owner_node: String,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    /// The index of the ledger that contains the transaction that most recently modified this object.
    pub previous_txn_lgr_seq: u32,
}

impl Model for DepositPreauth {}

impl LedgerObject<NoFlags> for DepositPreauth {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl DepositPreauth {
    pub fn new(
        index: Option<String>,
        ledger_index: Option<String>,
        account: String,
        authorize: String,
        owner_node: String,
        previous_txn_id: String,
        previous_txn_lgr_seq: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                FlagCollection::default(),
                LedgerEntryType::DepositPreauth,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            account,
            authorize,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let deposit_preauth = DepositPreauth::new(
            Some("4A255038CC3ADCC1A9C91509279B59908251728D0DAADB248FFE297D0F7E068C".to_string()),
            None,
            "rsUiUMpnrgxQp24dJYZDhmV4bE3aBtQyt8".to_string(),
            "rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de".to_string(),
            "0000000000000000".to_string(),
            "3E8964D5A86B3CD6B9ECB33310D4E073D64C865A5B866200AD2B7E29F8326702".to_string(),
            7,
        );
        let serialized = serde_json::to_string(&deposit_preauth).unwrap();

        let deserialized: DepositPreauth = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deposit_preauth, deserialized);
    }
}
