use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::requests::Marker;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NFTHistory {
    pub nft_id: String,
    pub ledger_index_min: u32,
    pub ledger_index_max: u32,
    pub transactions: Vec<NFTHistoryTransaction>,
    pub limit: Option<u32>,
    pub marker: Option<Marker>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NFTHistoryTransaction {
    pub hash: String,
    pub ledger_index: u32,
    pub meta: NFTHistoryTransactionMeta,
    pub validated: bool,
    pub tx: Option<Value>,
    pub tx_blob: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NFTHistoryTransactionMeta {
    Json(Value),
    Blob(String),
}

// impl TryFrom<XRPLResult> for NFTHistory {
//     type Error = crate::models::XRPLModelException;
//
//     fn try_from(result: XRPLResult) -> crate::models::XRPLModelResult<Self> {
//         match result {
//             XRPLResult::NFTHistory(nft_history) => Ok(nft_history),
//             res => Err(XRPLResultException::UnexpectedResultType(
//                 "NFTHistory".to_string(),
//                 res.get_name(),
//             )
//             .into()),
//         }
//     }
// }
//
// impl TryFrom<XRPLResponse> for NFTHistory {
//     type Error = crate::models::XRPLModelException;
//
//     fn try_from(response: XRPLResponse) -> crate::models::XRPLModelResult<Self> {
//         match response.result {
//             Some(result) => NFTHistory::try_from(result),
//             None => Err(crate::models::XRPLModelException::MissingField(
//                 "result".to_string(),
//             )),
//         }
//     }
// }
