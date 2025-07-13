use super::{clients::XRPLAsyncClient, exceptions::XRPLHelperResult};
use crate::asynch::exceptions::XRPLHelperException;
use crate::models::XRPLModelException;
use crate::{
    core::addresscodec::{is_valid_xaddress, xaddress_to_classic_address},
    models::{
        ledger::objects::account_root::AccountRoot,
        requests::{account_info::AccountInfo, account_tx::AccountTx},
        results::{self},
        XRPAmount,
    },
};
use alloc::borrow::ToOwned;
use alloc::string::ToString;
use crate::models::results::XRPLResponse;

pub async fn does_account_exist<C>(
    address: String,
    client: &C,
    ledger_index: Option<String>,
) -> XRPLHelperResult<bool>
where
    C: XRPLAsyncClient,
{
    match get_account_root(address, client, ledger_index.unwrap_or("validated".into())).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub async fn get_next_valid_seq_number(
    address: String,
    client: &impl XRPLAsyncClient,
    ledger_index: Option<String>,
) -> XRPLHelperResult<u32> {
    let account_info =
        get_account_root(address, client, ledger_index.unwrap_or("current".into())).await?;
    Ok(account_info.sequence)
}

pub async fn get_xrp_balance<'a: 'b, 'b, C>(
    address: String,
    client: &'a C,
    ledger_index: Option<String>,
) -> XRPLHelperResult<XRPAmount>
where
    C: XRPLAsyncClient,
{
    let account_info =
        get_account_root(address, client, ledger_index.unwrap_or("validated".into())).await?;
    match account_info.balance {
        Some(balance) => Ok(balance),
        None => Ok(0.into()),
    }
}

pub async fn get_account_root<'a: 'b, 'b, C>(
    address: String,
    client: &'a C,
    ledger_index: String,
) -> XRPLHelperResult<AccountRoot>
where
    C: XRPLAsyncClient,
{
    let mut classic_address = address;
    if is_valid_xaddress(&classic_address) {
        classic_address = xaddress_to_classic_address(&classic_address)?.0.into();
    }
    let request = AccountInfo::new(
        None,
        classic_address.to_string(),
        None,
        Some(ledger_index.to_string().into()),
        None,
        None,
        None,
    )
    .into();
    let response = client.request(request).await?;
    let response: XRPLResponse<'a, results::account_info::AccountInfoVersionMap> = serde_json::from_str(&response)?;

    let account_info = match response.result {
        Some(result) => result,
        None => {
            return Err(XRPLHelperException::from(XRPLModelException::MissingField(
                "result".to_string(),
            )))
        }
    };
    let account_root = account_info.get_account_root().to_owned();

    Ok(account_root)
}

pub async fn get_latest_transaction<'a: 'b, 'b, C>(
    mut address: String,
    client: &C,
) -> XRPLHelperResult<results::account_tx::AccountTxVersionMap>
where
    C: XRPLAsyncClient,
{
    if is_valid_xaddress(&address) {
        address = xaddress_to_classic_address(&address)?.0.into();
    }
    let account_tx = AccountTx::new(
        None,
        address.to_string(),
        None,
        Some("validated".into()),
        None,
        None,
        None,
        None,
        Some(1),
        None,
    );
    let response_raw = client.request(account_tx.into()).await?;
    let response: results::account_tx::AccountTxVersionMap =
        serde_json::from_str(&response_raw)?; // TODO probably it is XRPLResponse<'a, results::account_tx::AccountTxVersionMap>

    Ok(response)
}
