use crate::models::{Model, XRPLModelException, XRPLModelResult};

use bigdecimal::BigDecimal;
use core::convert::TryInto;
use core::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct IssuedCurrencyAmount {
    pub currency: String,
    pub issuer: String,
    pub value: String,
}

impl Model for IssuedCurrencyAmount {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self.value.parse::<f64>()?;

        Ok(())
    }
}

impl IssuedCurrencyAmount {
    pub fn new(currency: String, issuer: String, value: String) -> Self {
        Self {
            currency,
            issuer,
            value,
        }
    }
}

impl TryInto<BigDecimal> for IssuedCurrencyAmount {
    type Error = XRPLModelException;

    fn try_into(self) -> XRPLModelResult<BigDecimal, Self::Error> {
        Ok(BigDecimal::from_str(&self.value)?)
    }
}

impl PartialOrd for IssuedCurrencyAmount {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IssuedCurrencyAmount {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
