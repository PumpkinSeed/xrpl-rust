use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    amount::XRPAmount,
    transactions::{Memo, SignerWrapper, Transaction, TransactionType},
    Model, ValidateCurrencies, XRPLModelException, XRPLModelResult,
};
use crate::models::{FlagCollection, NoFlags};

use super::CommonFields;

/// Finishes an Escrow and delivers XRP from a held payment to the recipient.
///
/// See EscrowFinish:
/// `<https://xrpl.org/escrowfinish.html>`
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone, xrpl_rust_macros::ValidateCurrencies,
)]
#[serde(rename_all = "PascalCase")]
pub struct EscrowFinish {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the EscrowFinish model.
    //
    // See EscrowFinish fields:
    // `<https://xrpl.org/escrowfinish.html#escrowfinish-fields>`
    /// Address of the source account that funded the held payment.
    pub owner: String,
    /// Transaction sequence of EscrowCreate transaction that created the held payment to finish.
    pub offer_sequence: u32,
    /// Hex value matching the previously-supplied PREIMAGE-SHA-256 crypto-condition  of the held payment.
    pub condition: Option<String>,
    /// Hex value of the PREIMAGE-SHA-256 crypto-condition fulfillment  matching the held payment's Condition.
    pub fulfillment: Option<String>,
}

impl Model for EscrowFinish {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self._get_condition_and_fulfillment_error()?;
        self.validate_currencies()
    }
}

impl Transaction<NoFlags> for EscrowFinish {
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

impl EscrowFinishError for EscrowFinish {
    fn _get_condition_and_fulfillment_error(&self) -> XRPLModelResult<()> {
        if (self.condition.is_some() && self.fulfillment.is_none())
            || (self.condition.is_none() && self.condition.is_some())
        {
            Err(XRPLModelException::FieldRequiresField {
                field1: "condition".into(),
                field2: "fulfillment".into(),
            })
        } else {
            Ok(())
        }
    }
}

impl EscrowFinish {
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
        condition: Option<String>,
        fulfillment: Option<String>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                account,
                TransactionType::EscrowFinish,
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
            condition,
            fulfillment,
        }
    }
}

pub trait EscrowFinishError {
    fn _get_condition_and_fulfillment_error(&self) -> XRPLModelResult<()>;
}

#[cfg(test)]
mod test_escrow_finish_errors {

    use crate::models::Model;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_condition_and_fulfillment_error() {
        let escrow_finish = EscrowFinish::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            10,
            Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"
                    .into(),
            ),
            None,
        );

        assert_eq!(
            escrow_finish.validate().unwrap_err().to_string().as_str(),
            "If the field `\"condition\"` is defined, the field `\"fulfillment\"` must also be defined"
        );
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_serde() {
        let default_txn = EscrowFinish::new(
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
            Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"
                    .into(),
            ),
            Some("A0028000".into()),
        );
        let default_json_str = r#"{"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","TransactionType":"EscrowFinish","Flags":0,"SigningPubKey":"","Owner":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","OfferSequence":7,"Condition":"A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100","Fulfillment":"A0028000"}"#;
        // Serialize
        let default_json_value: Value = serde_json::from_str(default_json_str).unwrap();
        // let serialized_string = serde_json::to_string(&default_txn).unwrap();
        let serialized_value = serde_json::to_value(&default_txn).unwrap();
        assert_eq!(serialized_value, default_json_value);

        // Deserialize
        let deserialized: EscrowFinish = serde_json::from_str(default_json_str).unwrap();
        assert_eq!(default_txn, deserialized);
    }
}
