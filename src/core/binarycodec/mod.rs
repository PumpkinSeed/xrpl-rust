//! Functions for encoding objects into the XRP Ledger's
//! canonical binary format and decoding them.

pub mod definitions;
pub mod types;

use types::{AccountId, STObject};

use alloc::{borrow::Cow, string::String, vec::Vec};
use core::convert::TryFrom;
use hex::ToHex;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod binary_wrappers;
pub mod exceptions;
pub(crate) mod test_cases;
pub mod utils;

use crate::core::binarycodec::types::{SerializedType, XRPLType};
use crate::XRPLSerdeJsonError;
pub use binary_wrappers::*;

use super::exceptions::XRPLCoreResult;

const TRANSACTION_SIGNATURE_PREFIX: i32 = 0x53545800;
const TRANSACTION_MULTISIG_PREFIX: [u8; 4] = (0x534D5400u32).to_be_bytes();

pub fn encode<T>(signed_transaction: &T) -> XRPLCoreResult<String>
where
    T: Serialize,
{
    serialize_json(signed_transaction, None, None, false)
}

pub fn decode<T>(hex_string: &str) -> XRPLCoreResult<T>
where
    T: DeserializeOwned,
{
    let bytes = hex::decode(hex_string)?;

    let obj = STObject::new(Some(bytes.as_ref()))?;

    let value = obj.try_to_value()?;

    let data = serde_json::to_string(&value).unwrap();
    println!("Decoded JSON: {}", data);

    let result: T = serde_json::from_str(&*data).unwrap();

    Ok(result)
}

pub fn encode_for_signing<T>(prepared_transaction: &T) -> XRPLCoreResult<String>
where
    T: Serialize,
{
    serialize_json(
        prepared_transaction,
        Some(TRANSACTION_SIGNATURE_PREFIX.to_be_bytes().as_ref()),
        None,
        true,
    )
}

pub fn encode_for_multisigning<T>(
    prepared_transaction: &T,
    signing_account: Cow<'_, str>,
) -> XRPLCoreResult<String>
where
    T: Serialize,
{
    let signing_account_id = match AccountId::try_from(signing_account.as_ref()) {
        Ok(account_id) => account_id,
        Err(e) => {
            return Err(e);
        }
    };

    serialize_json(
        prepared_transaction,
        Some(TRANSACTION_MULTISIG_PREFIX.as_ref()),
        Some(signing_account_id.as_ref()),
        true,
    )
}

fn serialize_json<T>(
    prepared_transaction: &T,
    prefix: Option<&[u8]>,
    suffix: Option<&[u8]>,
    signing_only: bool,
) -> XRPLCoreResult<String>
where
    T: Serialize,
{
    let mut buffer = Vec::new();
    if let Some(p) = prefix {
        buffer.extend(p);
    }

    let json_value =
        serde_json::to_value(prepared_transaction).map_err(XRPLSerdeJsonError::from)?;
    // dbg!(&json_value);
    let st_object = STObject::try_from_value(json_value, signing_only)?;
    buffer.extend(st_object.as_ref());

    if let Some(s) = suffix {
        buffer.extend(s);
    }
    let hex_string = buffer.encode_hex_upper::<String>();

    Ok(hex_string)
}

#[cfg(test)]
mod tests {
    use crate::core::binarycodec::decode;
    use crate::models::{transactions, FlagCollection, XRPAmount};
    use alloc::borrow::Cow;
    use alloc::{format, vec};

    #[test]
    fn test_encode() {
        let common_fields = transactions::CommonFields {
            account: Cow::from("rp9Dq8zhMWb55WrDdcoAJZ5ytrcGsboJxe"),
            transaction_type: transactions::TransactionType::NFTokenMint,
            sequence: Some(0),
            flags: FlagCollection::new(vec![]), // Default flags
            ticket_sequence: Some(184),
            account_txn_id: None,
            fee: Some(XRPAmount(Cow::from("10"))),
            last_ledger_sequence: Some(20000),
            memos: None,
            network_id: None,
            signers: None,
            signing_pub_key: Some(Cow::from("EDC259073A91691115F183983D4305718367AA13329D46FB7B5F588305149E7D0E")),
            source_tag: None,
            txn_signature: Some(Cow::from("3D563759600D8EFDDACCF9F21E7EC160B51873C6C4F6F6A7DBEA30970F64CD4129196D271010A3A350BF15278B6A5640FAA771FDB9EA8223831633939BEC4902")),
        };

        let nft_mint_tx = transactions::nftoken_mint::NFTokenMint {
            common_fields,
            nftoken_taxon: 0,
            issuer: None,
            transfer_fee: None,
            uri: Some(Cow::from("12345678")),
        };

        let encoded = super::encode(&nft_mint_tx).unwrap();
        let expected = "12001922000000002400000000201B00004E202029000000B8202A0000000068400000000000000A7321EDC259073A91691115F183983D4305718367AA13329D46FB7B5F588305149E7D0E74403D563759600D8EFDDACCF9F21E7EC160B51873C6C4F6F6A7DBEA30970F64CD4129196D271010A3A350BF15278B6A5640FAA771FDB9EA8223831633939BEC490275041234567881140C862C3DD406C9F3F757F8C479292BDCD11A2A11";
        println!("Encoded transaction: {}", encoded);

        let decoded: transactions::nftoken_mint::NFTokenMint = decode(&encoded).unwrap();
        println!("Decoded transaction: {:?}", decoded);
    }
}
