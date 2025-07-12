use crate::models::FlagCollection;
use crate::models::Model;
use crate::models::{ledger::objects::LedgerEntryType, NoFlags};

use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

/// The `FeeSettings` object type contains the current base transaction cost and reserve amounts
/// as determined by fee voting. Each ledger version contains at most one `FeeSettings` object.
///
/// `<https://xrpl.org/feesettings.html#feesettings>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FeeSettings {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the FeeSettings model.
    //
    // See FeeSettings fields:
    // `<https://xrpl.org/feesettings.html#feesettings-fields>`
    /// The transaction cost of the "reference transaction" in drops of XRP as hexadecimal.
    pub base_fee: String,
    /// The BaseFee translated into "fee units".
    pub reference_fee_units: u32,
    /// The base reserve for an account in the XRP Ledger, as drops of XRP.
    pub reserve_base: u32,
    /// The incremental owner reserve for owning objects, as drops of XRP.
    pub reserve_increment: u32,
}

impl Model for FeeSettings {}

impl LedgerObject<NoFlags> for FeeSettings {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl FeeSettings {
    pub fn new(
        index: Option<String>,
        ledger_index: Option<String>,
        base_fee: String,
        reference_fee_units: u32,
        reserve_base: u32,
        reserve_increment: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                FlagCollection::default(),
                LedgerEntryType::FeeSettings,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            base_fee,
            reference_fee_units,
            reserve_base,
            reserve_increment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let fee_settings = FeeSettings::new(
            Some("4BC50C9B0D8515D3EAAE1E74B29A95804346C491EE1A95BF25E4AAB854A6A651".to_string()),
            None,
            "000000000000000A".to_string(),
            10,
            20000000,
            5000000,
        );
        let serialized = serde_json::to_string(&fee_settings).unwrap();

        let deserialized: FeeSettings = serde_json::from_str(&serialized).unwrap();

        assert_eq!(fee_settings, deserialized);
    }
}
