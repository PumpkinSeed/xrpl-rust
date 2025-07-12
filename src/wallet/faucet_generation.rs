use super::Wallet;
use crate::asynch::{
    clients::{XRPLAsyncClient, XRPLFaucet},
    exceptions::XRPLHelperResult,
    wallet::generate_faucet_wallet as async_generate_faucet_wallet,
};
use embassy_futures::block_on;
use url::Url;

pub use crate::asynch::wallet::get_faucet_url;

pub fn generate_faucet_wallet<'a, C>(
    client: &C,
    wallet: Option<Wallet>,
    faucet_host: Option<Url>,
    usage_context: Option<String>,
    user_agent: Option<String>,
) -> XRPLHelperResult<Wallet>
where
    C: XRPLFaucet + XRPLAsyncClient,
{
    block_on(async_generate_faucet_wallet(
        client,
        wallet,
        faucet_host,
        usage_context,
        user_agent,
    ))
}
