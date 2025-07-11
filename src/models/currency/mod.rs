pub mod issued_currency;
pub mod xrp;

use crate::models::Model;
use alloc::borrow::Cow;
pub use issued_currency::*;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
pub use xrp::*;

use super::{IssuedCurrencyAmount, XRPAmount};

pub trait ToAmount<A> {
    fn to_amount(&self, value: String) -> A;
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
pub enum Currency{
    IssuedCurrency(IssuedCurrency),
    XRP(XRP),
}

impl Model for Currency {
    fn get_errors(&self) -> crate::models::XRPLModelResult<()> {
        match self {
            Currency::IssuedCurrency(issued_currency) => issued_currency.get_errors(),
            Currency::XRP(xrp) => xrp.get_errors(),
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self::XRP(XRP::new())
    }
}

impl From<IssuedCurrency> for Currency {
    fn from(value: IssuedCurrency) -> Self {
        Self::IssuedCurrency(value)
    }
}

impl From<XRP> for Currency {
    fn from(value: XRP) -> Self {
        Self::XRP(value)
    }
}

impl From<IssuedCurrencyAmount> for Currency {
    fn from(value: IssuedCurrencyAmount) -> Self {
        IssuedCurrency::new(value.currency, value.issuer).into()
    }
}

impl From<XRPAmount> for Currency {
    fn from(_value: XRPAmount) -> Self {
        XRP::new().into()
    }
}

impl From<&IssuedCurrencyAmount> for Currency {
    fn from(value: &IssuedCurrencyAmount) -> Self {
        IssuedCurrency::new(value.currency.clone(), value.issuer.clone()).into()
    }
}

impl From<&XRPAmount> for Currency {
    fn from(_value: &XRPAmount) -> Self {
        XRP::new().into()
    }
}
