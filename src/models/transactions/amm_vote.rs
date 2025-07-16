use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    Currency, FlagCollection, Model, NoFlags, ValidateCurrencies, XRPAmount, XRPLModelException,
    XRPLModelResult,
};

use super::{CommonFields, Memo, SignerWrapper, Transaction, TransactionType};

pub const AMM_VOTE_MAX_TRADING_FEE: u16 = 1000;

/// Vote on the trading fee for an Automated Market Maker (AMM) instance.
///
/// Up to 8 accounts can vote in proportion to the amount of the AMM's LP Tokens
/// they hold.
/// Each new vote re-calculates the AMM's trading fee based on a weighted average
/// of the votes.
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone, xrpl_rust_macros::ValidateCurrencies,
)]
#[serde(rename_all = "PascalCase")]
pub struct AMMVote {
    pub common_fields: CommonFields<NoFlags>,
    /// The definition for one of the assets in the AMM's pool.
    pub asset: Currency,
    /// The definition for the other asset in the AMM's pool.
    #[serde(rename = "Asset2")]
    pub asset2: Currency,
    /// The proposed fee to vote for, in units of 1/100,000; a value of 1 is equivalent
    /// to 0.001%.
    /// The maximum value is 1000, indicating a 1% fee.
    pub trading_fee: Option<u16>,
}

impl Model for AMMVote {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self.validate_currencies()?;
        if let Some(trading_fee) = self.trading_fee {
            if trading_fee > AMM_VOTE_MAX_TRADING_FEE {
                return Err(XRPLModelException::ValueTooHigh {
                    field: "trading_fee".into(),
                    max: AMM_VOTE_MAX_TRADING_FEE.into(),
                    found: trading_fee.into(),
                });
            }
        }

        Ok(())
    }
}

impl Transaction<NoFlags> for AMMVote {
    fn get_transaction_type(&self) -> &TransactionType {
        self.common_fields.get_transaction_type()
    }

    fn get_common_fields(&self) -> &CommonFields<NoFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<NoFlags> {
        self.common_fields.get_mut_common_fields()
    }
}

impl<'a> AMMVote {
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
        asset: Currency,
        asset2: Currency,
        trading_fee: Option<u16>,
    ) -> AMMVote {
        AMMVote {
            common_fields: CommonFields::new(
                account,
                TransactionType::AMMVote,
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
            asset,
            asset2,
            trading_fee,
        }
    }
}
