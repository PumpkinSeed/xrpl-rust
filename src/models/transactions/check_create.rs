use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::CommonFields;
use crate::models::{
    amount::Amount,
    transactions::{Transaction, TransactionType},
    Model,
};
use crate::models::{FlagCollection, NoFlags, ValidateCurrencies};

use super::{Memo, SignerWrapper};

/// Create a Check object in the ledger, which is a deferred
/// payment that can be cashed by its intended destination.
///
/// See CheckCreate:
/// `<https://xrpl.org/checkcreate.html>`
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone, xrpl_rust_macros::ValidateCurrencies,
)]
#[serde(rename_all = "PascalCase")]
pub struct CheckCreate {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the CheckCreate model.
    //
    // See CheckCreate fields:
    // `<https://xrpl.org/checkcreate.html#checkcreate-fields>`
    /// The unique address of the account that can cash the Check.
    pub destination: String,
    /// Maximum amount of source currency the Check is allowed to debit the sender,
    /// including transfer fees on non-XRP currencies. The Check can only credit
    /// the destination with the same currency (from the same issuer, for non-XRP
    /// currencies). For non-XRP amounts, the nested field names MUST be lower-case.
    pub send_max: Amount,
    /// Arbitrary tag that identifies the reason for the Check, or a hosted recipient to pay.
    pub destination_tag: Option<u32>,
    /// Time after which the Check is no longer valid, in seconds since the Ripple Epoch.
    pub expiration: Option<u32>,
    /// Arbitrary 256-bit hash representing a specific reason or identifier for this Check.
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Option<String>,
}

impl Model for CheckCreate {
    fn get_errors(&self) -> crate::models::XRPLModelResult<()> {
        self.validate_currencies()
    }
}

impl Transaction<NoFlags> for CheckCreate {
    fn get_transaction_type(&self) -> &TransactionType {
        self.common_fields.get_transaction_type()
    }

    fn get_common_fields(&self) -> &CommonFields<NoFlags> {
        self.common_fields.get_common_fields()
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<NoFlags> {
        self.common_fields.get_mut_common_fields()
    }
}

impl CheckCreate {
    pub fn new(
        account: String,
        account_txn_id: Option<String>,
        fee: Option<XRPAmount>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<SignerWrapper>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        destination: String,
        send_max: Amount,
        destination_tag: Option<u32>,
        expiration: Option<u32>,
        invoice_id: Option<String>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                account,
                TransactionType::CheckCreate,
                account_txn_id,
                fee,
                Some(FlagCollection::default()),
                last_ledger_sequence,
                memos,
                None,
                sequence,
                signers,
                None,
                source_tag,
                ticket_sequence,
                None,
            ),
            destination,
            send_max,
            destination_tag,
            expiration,
            invoice_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let default_txn = CheckCreate::new(
            "rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo".into(),
            None,
            Some("12".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            "rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy".into(),
            "100000000".into(),
            Some(1),
            Some(570113521),
            Some("6F1DFD1D0FE8A32E40E1F2C05CF1C15545BAB56B617F9C6C2D63A6B704BEF59B".into()),
        );
        let default_json_str = r#"{"Account":"rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo","TransactionType":"CheckCreate","Fee":"12","Flags":0,"SigningPubKey":"","Destination":"rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy","SendMax":"100000000","DestinationTag":1,"Expiration":570113521,"InvoiceID":"6F1DFD1D0FE8A32E40E1F2C05CF1C15545BAB56B617F9C6C2D63A6B704BEF59B"}"#;
        // Serialize
        let default_json_value = serde_json::to_value(default_json_str).unwrap();
        let serialized_string = serde_json::to_string(&default_txn).unwrap();
        let serialized_value = serde_json::to_value(&serialized_string).unwrap();
        assert_eq!(serialized_value, default_json_value);

        // Deserialize
        let deserialized: CheckCreate = serde_json::from_str(default_json_str).unwrap();
        assert_eq!(default_txn, deserialized);
    }
}
