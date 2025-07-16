use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::CommonFields;
use crate::models::{
    transactions::{Transaction, TransactionType},
    Model,
};
use crate::models::{FlagCollection, NoFlags, ValidateCurrencies};

use super::{Memo, SignerWrapper};

/// Cancels an Escrow and returns escrowed XRP to the sender.
///
/// See EscrowCancel:
/// `<https://xrpl.org/escrowcancel.html>`
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone, xrpl_rust_macros::ValidateCurrencies,
)]
#[serde(rename_all = "PascalCase")]
pub struct EscrowCancel {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the EscrowCancel model.
    //
    // See EscrowCancel fields:
    // `<https://xrpl.org/escrowcancel.html#escrowcancel-flags>`
    /// Address of the source account that funded the escrow payment.
    pub owner: String,
    /// Transaction sequence (or Ticket number) of EscrowCreate transaction that created the escrow to cancel.
    pub offer_sequence: u32,
}

impl Model for EscrowCancel {
    fn get_errors(&self) -> crate::models::XRPLModelResult<()> {
        self.validate_currencies()
    }
}

impl Transaction<NoFlags> for EscrowCancel {
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

impl EscrowCancel {
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
        owner: String,
        offer_sequence: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                account,
                TransactionType::EscrowCancel,
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
            owner,
            offer_sequence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let default_txn = EscrowCancel::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            7,
        );
        let default_json_str = r#"{"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","TransactionType":"EscrowCancel","Flags":0,"SigningPubKey":"","Owner":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","OfferSequence":7}"#;
        // Serialize
        let default_json_value = serde_json::to_value(default_json_str).unwrap();
        let serialized_string = serde_json::to_string(&default_txn).unwrap();
        let serialized_value = serde_json::to_value(&serialized_string).unwrap();
        assert_eq!(serialized_value, default_json_value);

        // Deserialize
        let deserialized: EscrowCancel = serde_json::from_str(default_json_str).unwrap();
        assert_eq!(default_txn, deserialized);
    }
}
