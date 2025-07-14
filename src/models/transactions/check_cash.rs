use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::{
    amount::Amount,
    transactions::{Memo, Signer, Transaction, TransactionType},
    Model,
};
use crate::models::{
    FlagCollection, NoFlags, ValidateCurrencies, XRPLModelException, XRPLModelResult,
};

use super::CommonFields;

/// Cancels an unredeemed Check, removing it from the ledger without
/// sending any money. The source or the destination of the check can
/// cancel a Check at any time using this transaction type. If the Check
/// has expired, any address can cancel it.
///
/// See CheckCash:
/// `<https://xrpl.org/checkcash.html>`
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone, xrpl_rust_macros::ValidateCurrencies,
)]
#[serde(rename_all = "PascalCase")]
pub struct CheckCash {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    // The custom fields for the CheckCash model.
    //
    // See CheckCash fields:
    // `<https://xrpl.org/checkcash.html#checkcash-fields>`
    /// The ID of the Check ledger object to cash, as a 64-character hexadecimal string.
    #[serde(rename = "CheckID")]
    pub check_id: String,
    /// Redeem the Check for exactly this amount, if possible. The currency must match that of the
    /// SendMax of the corresponding CheckCreate transaction. You must provide either this field or DeliverMin.
    pub amount: Option<Amount>,
    /// Redeem the Check for at least this amount and for as much as possible. The currency must
    /// match that of the SendMax of the corresponding CheckCreate transaction. You must provide
    /// either this field or Amount.
    pub deliver_min: Option<Amount>,
}

impl Model for CheckCash {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self._get_amount_and_deliver_min_error()?;
        self.validate_currencies()
    }
}

impl Transaction<NoFlags> for CheckCash {
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

impl CheckCashError for CheckCash {
    fn _get_amount_and_deliver_min_error(&self) -> XRPLModelResult<()> {
        if (self.amount.is_none() && self.deliver_min.is_none())
            || (self.amount.is_some() && self.deliver_min.is_some())
        {
            Err(XRPLModelException::InvalidFieldCombination {
                field: "amount",
                other_fields: &["deliver_min"],
            })
        } else {
            Ok(())
        }
    }
}

impl CheckCash {
    pub fn new(
        account: String,
        account_txn_id: Option<String>,
        fee: Option<XRPAmount>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        check_id: String,
        amount: Option<Amount>,
        deliver_min: Option<Amount>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                account,
                TransactionType::CheckCash,
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
            check_id,
            amount,
            deliver_min,
        }
    }
}

pub trait CheckCashError {
    fn _get_amount_and_deliver_min_error(&self) -> XRPLModelResult<()>;
}

#[cfg(test)]
mod test_check_cash_error {
    use crate::models::Model;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_amount_and_deliver_min_error() {
        let check_cash = CheckCash::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "".into(),
            None,
            None,
        );

        assert_eq!(
            check_cash.validate().unwrap_err().to_string().as_str(),
            "Invalid field combination: amount with [\"deliver_min\"]"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let default_txn = CheckCash::new(
            "rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy".into(),
            None,
            Some("12".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            "838766BA2B995C00744175F69A1B11E32C3DBC40E64801A4056FCBD657F57334".into(),
            Some("100000000".into()),
            None,
        );
        let default_json_str = r#"{"Account":"rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy","TransactionType":"CheckCash","Fee":"12","Flags":0,"SigningPubKey":"","CheckID":"838766BA2B995C00744175F69A1B11E32C3DBC40E64801A4056FCBD657F57334","Amount":"100000000"}"#;
        // Serialize
        let default_json_value = serde_json::to_value(default_json_str).unwrap();
        let serialized_string = serde_json::to_string(&default_txn).unwrap();
        let serialized_value = serde_json::to_value(&serialized_string).unwrap();
        assert_eq!(serialized_value, default_json_value);

        // Deserialize
        let deserialized: CheckCash = serde_json::from_str(default_json_str).unwrap();
        assert_eq!(default_txn, deserialized);
    }
}
