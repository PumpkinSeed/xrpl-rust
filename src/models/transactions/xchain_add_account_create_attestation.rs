use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    Amount, FlagCollection, Model, NoFlags, ValidateCurrencies, XChainBridge, XRPAmount,
};

use super::{CommonFields, Memo, Signer, Transaction, TransactionType};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, xrpl_rust_macros::ValidateCurrencies)]
#[serde(rename_all = "PascalCase")]
pub struct XChainAddAccountCreateAttestation {
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    pub amount: Amount,
    pub attestation_reward_account: String,
    pub attestation_signer_account: String,
    pub destination: String,
    pub other_chain_source: String,
    pub public_key: String,
    pub signature: String,
    pub signature_reward: Amount,
    pub was_locking_chain_send: u8,
    #[serde(rename = "XChainAccountCreateCount")]
    pub xchain_account_create_count: String,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge,
}

impl Model for XChainAddAccountCreateAttestation {
    fn get_errors(&self) -> crate::models::XRPLModelResult<()> {
        self.validate_currencies()
    }
}

impl Transaction<NoFlags> for XChainAddAccountCreateAttestation {
    fn get_transaction_type(&self) -> &super::TransactionType {
        self.common_fields.get_transaction_type()
    }

    fn get_common_fields(&self) -> &super::CommonFields<NoFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut super::CommonFields<NoFlags> {
        &mut self.common_fields
    }
}

impl XChainAddAccountCreateAttestation {
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
        amount: Amount,
        attestation_reward_account: String,
        attestation_signer_account: String,
        destination: String,
        other_chain_source: String,
        public_key: String,
        signature: String,
        signature_reward: Amount,
        was_locking_chain_send: u8,
        xchain_account_create_count: String,
        xchain_bridge: XChainBridge,
    ) -> XChainAddAccountCreateAttestation {
        XChainAddAccountCreateAttestation {
            common_fields: CommonFields::new(
                account,
                TransactionType::XChainAddAccountCreateAttestation,
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
            attestation_reward_account,
            attestation_signer_account,
            destination,
            other_chain_source,
            public_key,
            signature,
            signature_reward,
            was_locking_chain_send,
            xchain_account_create_count,
            xchain_bridge,
        }
    }
}

#[cfg(test)]
mod test_serde {
    const EXAMPLE_JSON: &str = r#"{
        "Account": "rDr5okqGKmMpn44Bbhe5WAfDQx8e9XquEv",
        "Flags": 0,
        "TransactionType": "XChainAddAccountCreateAttestation",
        "OtherChainSource": "rUzB7yg1LcFa7m3q1hfrjr5w53vcWzNh3U",
        "Destination": "rJMfWNVbyjcCtds8kpoEjEbYQ41J5B6MUd",
        "Amount": "2000000000",
        "PublicKey": "EDF7C3F9C80C102AF6D241752B37356E91ED454F26A35C567CF6F8477960F66614",
        "Signature": "F95675BA8FDA21030DE1B687937A79E8491CE51832D6BEEBC071484FA5AF5B8A0E9AFF11A4AA46F09ECFFB04C6A8DAE8284AF3ED8128C7D0046D842448478500",
        "WasLockingChainSend": 1,
        "AttestationRewardAccount": "rpFp36UHW6FpEcZjZqq5jSJWY6UCj3k4Es",
        "AttestationSignerAccount": "rpWLegmW9WrFBzHUj7brhQNZzrxgLj9oxw",
        "XChainAccountCreateCount": "2",
        "SignatureReward": "204",
        "XChainBridge": {
            "LockingChainDoor": "r3nCVTbZGGYoWvZ58BcxDmiMUU7ChMa1eC",
            "LockingChainIssue": {
                "currency": "XRP"
            },
            "IssuingChainDoor": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
            "IssuingChainIssue": {
                "currency": "XRP"
            }
        },
        "Fee": "20"
    }"#;
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_deserialize() {
        let json = EXAMPLE_JSON;
        let deserialized: Result<XChainAddAccountCreateAttestation, _> = serde_json::from_str(json);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_serialize() {
        let attestation: XChainAddAccountCreateAttestation =
            serde_json::from_str(EXAMPLE_JSON).unwrap();
        let actual = serde_json::to_value(&attestation).unwrap();
        let expected: Value = serde_json::from_str(EXAMPLE_JSON).unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod test_xchain_claim {
    use crate::models::{
        transactions::xchain_claim::XChainClaim, Amount, IssuedCurrency, IssuedCurrencyAmount,
        Model, XChainBridge, XRPAmount, XRP,
    };
    use alloc::string::ToString;

    const ACCOUNT: &str = "r9LqNeG6qHxjeUocjvVki2XR35weJ9mZgQ";
    const ACCOUNT2: &str = "rpZc4mVfWUif9CRoHRKKcmhu1nx2xktxBo";
    const FEE: &str = "0.00001";
    const SEQUENCE: u32 = 19048;
    const ISSUER: &str = "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf";
    const GENESIS: &str = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";
    const DESTINATION: &str = "rJrRMgiRgrU6hDF4pgu5DXQdWyPbY35ErN";
    const CLAIM_ID: u64 = 3;
    const XRP_AMOUNT: &str = "123456789";

    fn xrp_bridge() -> XChainBridge {
        XChainBridge {
            locking_chain_door: ACCOUNT.to_string(),
            locking_chain_issue: XRP::new().into(),
            issuing_chain_door: GENESIS.to_string(),
            issuing_chain_issue: XRP::new().into(),
        }
    }

    fn iou_bridge() -> XChainBridge {
        XChainBridge {
            locking_chain_door: ACCOUNT.to_string(),
            locking_chain_issue: IssuedCurrency {
                currency: "USD".to_string(),
                issuer: ISSUER.to_string(),
            }
            .into(),
            issuing_chain_door: ACCOUNT2.to_string(),
            issuing_chain_issue: IssuedCurrency {
                currency: "USD".to_string(),
                issuer: ACCOUNT2.to_string(),
            }
            .into(),
        }
    }

    fn iou_amount() -> Amount {
        IssuedCurrencyAmount {
            currency: "USD".to_string(),
            issuer: ISSUER.to_string(),
            value: "123".to_string(),
        }
        .into()
    }

    #[test]
    fn test_successful_claim_xrp() {
        let claim = XChainClaim::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from(XRP_AMOUNT).into(),
            DESTINATION.to_string(),
            xrp_bridge(),
            CLAIM_ID.to_string().into(),
            None,
        );
        assert!(claim.validate().is_ok());
    }

    #[test]
    fn test_successful_claim_iou() {
        let claim = XChainClaim::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            iou_amount(),
            DESTINATION.to_string(),
            iou_bridge(),
            CLAIM_ID.to_string().into(),
            None,
        );
        assert!(claim.validate().is_ok());
    }

    #[test]
    fn test_successful_claim_destination_tag() {
        let claim = XChainClaim::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            Some(12345),
            None,
            XRPAmount::from(XRP_AMOUNT).into(),
            DESTINATION.to_string(),
            xrp_bridge(),
            CLAIM_ID.to_string().into(),
            None,
        );
        assert!(claim.validate().is_ok());
    }

    #[test]
    fn test_successful_claim_str_claim_id() {
        let claim_id_str = CLAIM_ID.to_string();
        let claim = XChainClaim::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from(XRP_AMOUNT).into(),
            DESTINATION.to_string(),
            xrp_bridge(),
            claim_id_str.as_str().into(),
            None,
        );
        assert!(claim.validate().is_ok());
    }

    #[test]
    #[should_panic]
    fn test_xrp_bridge_iou_amount() {
        let claim = XChainClaim::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            iou_amount(),
            DESTINATION.to_string(),
            xrp_bridge(),
            CLAIM_ID.to_string().into(),
            None,
        );
        claim.validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_iou_bridge_xrp_amount() {
        let claim = XChainClaim::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from(XRP_AMOUNT).into(),
            DESTINATION.to_string(),
            iou_bridge(),
            CLAIM_ID.to_string().into(),
            None,
        );
        claim.validate().unwrap();
    }
}
