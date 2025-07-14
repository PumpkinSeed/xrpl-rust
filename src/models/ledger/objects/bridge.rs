
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, NoFlags, XChainBridge, XRPAmount};

use super::{CommonFields, LedgerEntryType, LedgerObject};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Bridge {
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    pub account: String,
    pub signature_reward: XRPAmount,
    #[serde(rename = "XChainAccountClaimCount")]
    pub xchain_account_claim_count: u64,
    #[serde(rename = "XChainAccountCreateCount")]
    pub xchain_account_create_count: u64,
    pub xchain_bridge: XChainBridge,
    #[serde(rename = "XChainClaimID")]
    pub xchain_claim_id: String,
    pub min_account_create_amount: Option<XRPAmount>,
}

impl Model for Bridge {}

impl LedgerObject<NoFlags> for Bridge {
    fn get_ledger_entry_type(&self) -> super::LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl Bridge {
    pub fn new(
        index: Option<String>,
        ledger_index: Option<String>,
        account: String,
        signature_reward: XRPAmount,
        xchain_account_claim_count: u64,
        xchain_account_create_count: u64,
        xchain_bridge: XChainBridge,
        xchain_claim_id: String,
        min_account_create_amount: Option<XRPAmount>,
    ) -> Bridge {
        Bridge {
            common_fields: CommonFields::new(
                Default::default(),
                LedgerEntryType::Bridge,
                index.map(|x| x.to_string()),
                ledger_index.map(|x| x.to_string()),
            ),
            account,
            signature_reward,
            xchain_account_claim_count,
            xchain_account_create_count,
            xchain_bridge,
            xchain_claim_id,
            min_account_create_amount,
        }
    }
}
