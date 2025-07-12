use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    transactions::exceptions::XRPLXChainCreateBridgeException, Amount, FlagCollection, Model,
    NoFlags, ValidateCurrencies, XChainBridge, XRPAmount, XRPLModelResult, XRP,
};

use super::{CommonFields, Memo, Signer, Transaction, TransactionType};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, xrpl_rust_macros::ValidateCurrencies)]
#[serde(rename_all = "PascalCase")]
pub struct XChainCreateBridge {
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    pub signature_reward: Amount,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge,
    pub min_account_create_amount: Option<XRPAmount>,
}

impl Model for XChainCreateBridge {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self.validate_currencies()?;
        self.get_same_door_error()?;
        self.get_account_door_mismatch_error()?;
        self.get_cross_currency_bridge_not_allowed_error()?;
        self.get_min_account_create_amount_for_iou_error()
    }
}

impl Transaction<NoFlags> for XChainCreateBridge {
    fn get_transaction_type(&self) -> &super::TransactionType {
        self.common_fields.get_transaction_type()
    }

    fn get_common_fields(&self) -> &CommonFields<NoFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<NoFlags> {
        &mut self.common_fields
    }
}

impl XChainCreateBridge {
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
        signature_reward: Amount,
        xchain_bridge: XChainBridge,
        min_account_create_amount: Option<XRPAmount>,
    ) -> XChainCreateBridge {
        XChainCreateBridge {
            common_fields: CommonFields::new(
                account,
                TransactionType::XChainCreateBridge,
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
            signature_reward,
            xchain_bridge,
            min_account_create_amount,
        }
    }

    fn get_same_door_error(&self) -> XRPLModelResult<()> {
        let bridge = &self.xchain_bridge;
        if bridge.issuing_chain_door == bridge.locking_chain_door {
            Err(XRPLXChainCreateBridgeException::SameDoorAccounts.into())
        } else {
            Ok(())
        }
    }

    fn get_account_door_mismatch_error(&self) -> XRPLModelResult<()> {
        let bridge = &self.xchain_bridge;
        if ![&bridge.issuing_chain_door, &bridge.locking_chain_door]
            .contains(&&self.common_fields.account.to_string())
        {
            Err(XRPLXChainCreateBridgeException::AccountDoorMismatch.into())
        } else {
            Ok(())
        }
    }

    fn get_cross_currency_bridge_not_allowed_error(&self) -> XRPLModelResult<()> {
        let bridge = &self.xchain_bridge;
        if (bridge.locking_chain_issue == XRP::new().into())
            != (bridge.issuing_chain_issue == XRP::new().into())
        {
            Err(XRPLXChainCreateBridgeException::CrossCurrencyBridgeNotAllowed.into())
        } else {
            Ok(())
        }
    }

    fn get_min_account_create_amount_for_iou_error(&self) -> XRPLModelResult<()> {
        let bridge = &self.xchain_bridge;
        if self.min_account_create_amount.is_some()
            && bridge.locking_chain_issue != XRP::new().into()
        {
            Err(XRPLXChainCreateBridgeException::MinAccountCreateAmountForIOU.into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test_xchain_create_bridge {
    use super::XChainCreateBridge;
    use crate::models::{Amount, IssuedCurrency, Model, XChainBridge, XRPAmount, XRP};
    use alloc::borrow::Cow;

    const ACCOUNT: &str = "r9LqNeG6qHxjeUocjvVki2XR35weJ9mZgQ";
    const ACCOUNT2: &str = "rpZc4mVfWUif9CRoHRKKcmhu1nx2xktxBo";
    const FEE: &str = "0.00001";
    const SEQUENCE: u32 = 19048;
    const ISSUER: &str = "rGWrZyQqhTp9Xu7G5Pkayo7bXjH4k4QYpf";
    const GENESIS: &str = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";

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

    #[test]
    fn test_successful_xrp_xrp_bridge() {
        let bridge = xrp_bridge();
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from("200").into(),
            bridge,
            Some(XRPAmount::from("1000000")),
        );
        assert!(txn.validate().is_ok());
    }

    #[test]
    fn test_successful_iou_iou_bridge() {
        let bridge = iou_bridge();
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from("200").into(),
            bridge,
            None,
        );
        assert!(txn.validate().is_ok());
    }

    #[test]
    #[should_panic]
    fn test_same_door_accounts() {
        let bridge = XChainBridge {
            locking_chain_door: ACCOUNT.to_string(),
            locking_chain_issue: IssuedCurrency {
                currency: "USD".to_string(),
                issuer: ISSUER.to_string(),
            }
            .into(),
            issuing_chain_door: ACCOUNT.to_string(),
            issuing_chain_issue: IssuedCurrency {
                currency: "USD".to_string(),
                issuer: ACCOUNT.to_string(),
            }
            .into(),
        };
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from("200").into(),
            bridge,
            None,
        );
        txn.validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_xrp_iou_bridge() {
        let bridge = XChainBridge {
            locking_chain_door: ACCOUNT.to_string(),
            locking_chain_issue: XRP::new().into(),
            issuing_chain_door: ACCOUNT.to_string(),
            issuing_chain_issue: IssuedCurrency {
                currency: "USD".to_string(),
                issuer: ACCOUNT.to_string(),
            }
            .into(),
        };
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from("200").into(),
            bridge,
            None,
        );
        txn.validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_iou_xrp_bridge() {
        let bridge = XChainBridge {
            locking_chain_door: ACCOUNT.to_string(),
            locking_chain_issue: IssuedCurrency {
                currency: "USD".to_string(),
                issuer: ISSUER.to_string(),
            }
            .into(),
            issuing_chain_door: ACCOUNT.to_string(),
            issuing_chain_issue: XRP::new().into(),
        };
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from("200").into(),
            bridge,
            None,
        );
        txn.validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_account_not_in_bridge() {
        let bridge = XChainBridge {
            locking_chain_door: ACCOUNT.to_string(),
            locking_chain_issue: XRP::new().into(),
            issuing_chain_door: ACCOUNT2.to_string(),
            issuing_chain_issue: XRP::new().into(),
        };
        let txn = XChainCreateBridge::new(
            GENESIS.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from("200").into(),
            bridge,
            None,
        );
        txn.validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_iou_iou_min_account_create_amount() {
        let bridge = iou_bridge();
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            XRPAmount::from("200").into(),
            bridge,
            Some(XRPAmount::from("1000000")),
        );
        txn.validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_signature_reward() {
        let bridge = xrp_bridge();
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            Amount::from("hello"),
            bridge,
            Some(XRPAmount::from("1000000")),
        );
        txn.validate().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_min_account_create_amount() {
        let bridge = xrp_bridge();
        let txn = XChainCreateBridge::new(
            ACCOUNT.to_string(),
            None,
            Some(XRPAmount::from(FEE)),
            None,
            None,
            Some(SEQUENCE),
            None,
            None,
            None,
            Amount::from("-200"),
            bridge,
            Some(XRPAmount::from("hello")),
        );
        txn.validate().unwrap();
    }
}
