mod issued_currency_amount;
mod xrp_amount;

pub use issued_currency_amount::*;
pub use xrp_amount::*;

use alloc::string::ToString;
use core::convert::TryInto;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::{models::Model, utils::XRP_DROPS};

use super::{XRPLModelException, XRPLModelResult};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
pub enum Amount {
    IssuedCurrencyAmount(IssuedCurrencyAmount),
    XRPAmount(XRPAmount),
}

impl TryInto<BigDecimal> for Amount {
    type Error = XRPLModelException;

    fn try_into(self) -> XRPLModelResult<BigDecimal, Self::Error> {
        match self {
            Amount::IssuedCurrencyAmount(amount) => amount.try_into(),
            Amount::XRPAmount(amount) => amount.try_into(),
        }
    }
}

impl Model for Amount {
    fn get_errors(&self) -> XRPLModelResult<()> {
        match self {
            Amount::IssuedCurrencyAmount(amount) => amount.get_errors(),
            Amount::XRPAmount(amount) => amount.get_errors(),
        }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Self::XRPAmount("0".into())
    }
}

impl Amount {
    pub fn is_xrp(&self) -> bool {
        match self {
            Amount::IssuedCurrencyAmount(_) => false,
            Amount::XRPAmount(_) => true,
        }
    }

    pub fn is_issued_currency(&self) -> bool {
        !self.is_xrp()
    }
}

impl From<IssuedCurrencyAmount> for Amount {
    fn from(value: IssuedCurrencyAmount) -> Self {
        Self::IssuedCurrencyAmount(value)
    }
}

impl From<XRPAmount> for Amount {
    fn from(value: XRPAmount) -> Self {
        Self::XRPAmount(value)
    }
}

impl<'a> From<&'a str> for Amount {
    fn from(value: &'a str) -> Self {
        Self::XRPAmount(value.into())
    }
}

impl From<u32> for Amount {
    fn from(value: u32) -> Self {
        Self::XRPAmount(value.to_string().into())
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self::XRPAmount(value.to_string().into())
    }
}

impl From<f64> for Amount {
    fn from(value: f64) -> Self {
        let drops = XRP_DROPS as f64;
        let result = value * drops;

        Self::XRPAmount(result.to_string().into())
    }
}

impl From<BigDecimal> for Amount {
    fn from(value: BigDecimal) -> Self {
        Self::XRPAmount((value * XRP_DROPS).to_string().into())
    }
}
