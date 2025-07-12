use crate::models::ledger::objects::LedgerEntryType;
use crate::models::FlagCollection;
use crate::models::NoFlags;
use crate::models::{amount::Amount, Model};

use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

/// A Check object describes a check, similar to a paper personal check, which can be cashed by its
/// destination to get money from its sender.
///
/// `<https://xrpl.org/check.html#check>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Check {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the Check model.
    //
    // See Check fields:
    // `<https://xrpl.org/check.html#check-fields>`
    /// The sender of the `Check`. Cashing the `Check` debits this address's balance.
    pub account: String,
    /// The intended recipient of the `Check`. Only this address can cash the `Check`, using a
    /// `CheckCash` transaction.
    pub destination: String,
    /// A hint indicating which page of the sender's owner directory links to this object, in case
    /// the directory consists of multiple pages.
    pub owner_node: String,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    /// The index of the ledger that contains the transaction that most recently modified this object.
    pub previous_txn_lgr_seq: u32,
    /// The maximum amount of currency this Check can debit the sender. If the Check is successfully
    /// cashed, the destination is credited in the same currency for up to this amount.
    pub send_max: Amount,
    /// The sequence number of the `CheckCreate` transaction that created this check.
    pub sequence: u32,
    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages.
    pub destination_node: Option<String>,
    /// An arbitrary tag to further specify the destination for this `Check`, such as a hosted
    /// recipient at the destination address.
    pub destination_tag: Option<u32>,
    /// Indicates the time after which this `Check` is considered expired.
    pub expiration: Option<u32>,
    /// Arbitrary 256-bit hash provided by the sender as a specific reason or identifier for this Check.
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Option<String>,
    /// An arbitrary tag to further specify the source for this Check, such as a hosted recipient at
    /// the sender's address.
    pub source_tag: Option<u32>,
}

impl Model for Check {}

impl LedgerObject<NoFlags> for Check {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl Check {
    pub fn new(
        index: Option<String>,
        ledger_index: Option<String>,
        account: String,
        destination: String,
        owner_node: String,
        previous_txn_id: String,
        previous_txn_lgr_seq: u32,
        send_max: Amount,
        sequence: u32,
        destination_node: Option<String>,
        destination_tag: Option<u32>,
        expiration: Option<u32>,
        invoice_id: Option<String>,
        source_tag: Option<u32>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                FlagCollection::default(),
                LedgerEntryType::Check,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            account,
            destination,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            send_max,
            sequence,
            destination_node,
            destination_tag,
            expiration,
            invoice_id,
            source_tag,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let check = Check::new(
            Some("49647F0D748DC3FE26BDACBC57F251AADEFFF391403EC9BF87C97F67E9977FB0".to_string()),
            None,
            "rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo".to_string(),
            "rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy".to_string(),
            "0000000000000000".to_string(),
            "5463C6E08862A1FAE5EDAC12D70ADB16546A1F674930521295BC082494B62924".to_string(),
            6,
            Amount::XRPAmount("100000000".into()),
            2,
            Some("0000000000000000".to_string()),
            Some(1),
            Some(570113521),
            Some("46060241FABCF692D4D934BA2A6C4427CD4279083E38C77CBE642243E43BE291".to_string()),
            None,
        );
        let serialized = serde_json::to_string(&check).unwrap();

        let deserialized: Check = serde_json::from_str(&serialized).unwrap();

        assert_eq!(check, deserialized);
    }
}
