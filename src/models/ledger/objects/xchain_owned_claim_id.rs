use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Amount, Model, NoFlags, XChainBridge};

use super::{CommonFields, LedgerEntryType, LedgerObject, XChainClaimProofSig};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct XChainOwnedClaimID {
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    pub account: String,
    pub other_chain_source: String,
    pub signature_reward: Amount,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge,
    #[serde(rename = "XChainClaimAttestations")]
    pub xchain_claim_attestations: Vec<XChainClaimProofSig>,
    pub xchain_claim_id: String,
}

impl Model for XChainOwnedClaimID {}

impl LedgerObject<NoFlags> for XChainOwnedClaimID {
    fn get_ledger_entry_type(&self) -> super::LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl XChainOwnedClaimID {
    pub fn new(
        index: Option<String>,
        ledger_index: Option<String>,
        account: String,
        other_chain_source: String,
        signature_reward: Amount,
        xchain_bridge: XChainBridge,
        xchain_claim_attestations: Vec<XChainClaimProofSig>,
        xchain_claim_id: String,
    ) -> XChainOwnedClaimID {
        XChainOwnedClaimID {
            common_fields: CommonFields::new(
                Default::default(),
                LedgerEntryType::XChainOwnedClaimID,
                index,
                ledger_index,
            ),
            account,
            other_chain_source,
            signature_reward,
            xchain_bridge,
            xchain_claim_attestations,
            xchain_claim_id,
        }
    }
}
