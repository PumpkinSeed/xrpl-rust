use core::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::{metadata::TransactionMetadata, tx::TxVersionMap};
use crate::models::{XRPLModelException, XRPLModelResult};

/// Result type for NFTokenMint transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NFTokenMintResult {
    /// The NFTokenID of the minted token
    pub nftoken_id: String,
    /// The complete transaction metadata
    #[serde(flatten)]
    pub meta: TransactionMetadata,
}

/// Result type for NFTokenCreateOffer transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NFTokenCreateOfferResult {
    /// The OfferID of the created offer
    pub offer_id: String,
    /// The complete transaction metadata
    #[serde(flatten)]
    pub meta: TransactionMetadata,
}

/// Result type for NFTokenCancelOffer transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NFTokenCancelOfferResult {
    /// The NFTokenIDs of all tokens affected by the cancellation
    pub nftoken_ids: Vec<String>,
    /// The complete transaction metadata
    #[serde(flatten)]
    pub meta: TransactionMetadata,
}

/// Result type for NFTokenAcceptOffer transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NFTokenAcceptOfferResult {
    /// The NFTokenID of the accepted token
    pub nftoken_id: String,
    /// The complete transaction metadata
    #[serde(flatten)]
    pub meta: TransactionMetadata,
}

/// Macro to implement TryFrom<TxVersionMap> for NFToken result types
macro_rules! impl_try_from_tx_version_map {
    ($result_type:ident, $field_name:ident, $field_type:ty) => {
        impl TryFrom<TxVersionMap> for $result_type {
            type Error = XRPLModelException;

            fn try_from(tx: TxVersionMap) -> XRPLModelResult<Self> {
                // Extract metadata based on the version
                let meta = match &tx {
                    TxVersionMap::Default(tx) => tx.meta.clone(),
                    TxVersionMap::V1(tx) => tx.meta.clone(),
                };

                if let Some(meta) = meta {
                    if let Some(field_value) = meta.$field_name.clone() {
                        return Ok($result_type {
                            $field_name: field_value,
                            meta,
                        });
                    }
                }

                return Err(XRPLModelException::MissingField(
                    stringify!($field_name).into(),
                ));
            }
        }
    };
}

impl_try_from_tx_version_map!(NFTokenMintResult, nftoken_id, String);
impl_try_from_tx_version_map!(NFTokenCreateOfferResult, offer_id, String);
impl_try_from_tx_version_map!(
    NFTokenCancelOfferResult,
    nftoken_ids,
    Vec<String>
);
impl_try_from_tx_version_map!(NFTokenAcceptOfferResult, nftoken_id, String);
