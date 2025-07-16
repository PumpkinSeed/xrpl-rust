use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    Amount, FlagCollection, Model, NoFlags, ValidateCurrencies, XChainBridge, XRPAmount,
};

use super::{CommonFields, Memo, SignerWrapper, Transaction, TransactionType};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, xrpl_rust_macros::ValidateCurrencies)]
#[serde(rename_all = "PascalCase")]
pub struct XChainCommit {
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    pub amount: Amount,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge,
    #[serde(rename = "XChainClaimID")]
    pub xchain_claim_id: String,
    pub other_chain_destination: Option<String>,
}

impl Model for XChainCommit {
    fn get_errors(&self) -> crate::models::XRPLModelResult<()> {
        self.validate_currencies()
    }
}

impl Transaction<NoFlags> for XChainCommit {
    fn get_common_fields(&self) -> &CommonFields<NoFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<NoFlags> {
        &mut self.common_fields
    }

    fn get_transaction_type(&self) -> &super::TransactionType {
        self.common_fields.get_transaction_type()
    }
}

impl XChainCommit {
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
        amount: Amount,
        xchain_bridge: XChainBridge,
        xchain_claim_id: String,
        other_chain_destination: Option<String>,
    ) -> XChainCommit {
        XChainCommit {
            common_fields: CommonFields::new(
                account,
                TransactionType::XChainCommit,
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
            amount,
            other_chain_destination,
            xchain_bridge,
            xchain_claim_id,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use serde_json::Value;

    use crate::models::transactions::xchain_commit::XChainCommit;

    const EXAMPLE_JSON: &str = r#"{
        "Account": "rMTi57fNy2UkUb4RcdoUeJm7gjxVQvxzUo",
        "Flags": 0,
        "TransactionType": "XChainCommit",
        "XChainBridge": {
            "LockingChainDoor": "rMAXACCrp3Y8PpswXcg3bKggHX76V3F8M4",
            "LockingChainIssue": {
                "currency": "XRP"
            },
            "IssuingChainDoor": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
            "IssuingChainIssue": {
                "currency": "XRP"
            }
        },
        "Amount": "10000",
        "XChainClaimID": "13f"
    }"#;

    #[test]
    fn test_deserialize() {
        let json = EXAMPLE_JSON;
        let deserialized: Result<XChainCommit, _> = serde_json::from_str(json);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_serialize() {
        let attestation: XChainCommit = serde_json::from_str(EXAMPLE_JSON).unwrap();
        let actual = serde_json::to_value(&attestation).unwrap();
        let expected: Value = serde_json::from_str(EXAMPLE_JSON).unwrap();

        assert_eq!(actual, expected);
    }
}

