use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Amount, FlagCollection, Model, NoFlags, ValidateCurrencies, XChainBridge};

use super::{CommonFields, Transaction, TransactionType};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, xrpl_rust_macros::ValidateCurrencies)]
#[serde(rename_all = "PascalCase")]
pub struct XChainAddClaimAttestation {
    #[serde(flatten)]
    pub common_fields: CommonFields<NoFlags>,
    pub amount: Amount,
    pub attestation_reward_account: String,
    pub attestation_signer_account: String,
    pub other_chain_source: String,
    pub public_key: String,
    pub signature: String,
    pub was_locking_chain_send: u8,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge,
    #[serde(rename = "XChainClaimID")]
    pub xchain_claim_id: String,
    pub destination: Option<String>,
}

impl Model for XChainAddClaimAttestation {
    fn get_errors(&self) -> crate::models::XRPLModelResult<()> {
        self.validate_currencies()
    }
}

impl Transaction<NoFlags> for XChainAddClaimAttestation {
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

impl XChainAddClaimAttestation {
    pub fn new(
        account: String,
        account_txn_id: Option<String>,
        fee: Option<crate::models::XRPAmount>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<super::Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<super::SignerWrapper>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        amount: Amount,
        attestation_reward_account: String,
        attestation_signer_account: String,
        other_chain_source: String,
        public_key: String,
        signature: String,
        was_locking_chain_send: u8,
        xchain_bridge: XChainBridge,
        xchain_claim_id: String,
        destination: Option<String>,
    ) -> XChainAddClaimAttestation {
        XChainAddClaimAttestation {
            common_fields: CommonFields::new(
                account,
                TransactionType::XChainAddClaimAttestation,
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
            other_chain_source,
            public_key,
            signature,
            was_locking_chain_send,
            xchain_bridge,
            xchain_claim_id,
            destination,
        }
    }
}
