use crate::models::{Model, XRPLModelException, XRPLModelResult};
use alloc::string::{String, ToString};
use bigdecimal::BigDecimal;
use core::str::FromStr;
use core::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents an amount of XRP in Drops.
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct XRPAmount(pub String);

impl Model for XRPAmount {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self.0.parse::<u32>()?;

        Ok(())
    }
}

impl Default for XRPAmount {
    fn default() -> Self {
        Self("0".into())
    }
}

impl Display for XRPAmount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// implement Deserializing from Cow<str>, &str, String, Decimal, f64, u32, and Value
impl<'de, 'a> Deserialize<'de> for XRPAmount {
    fn deserialize<D>(deserializer: D) -> XRPLModelResult<XRPAmount, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let amount_string = Value::deserialize(deserializer)?;
        XRPAmount::try_from(amount_string).map_err(serde::de::Error::custom)
    }
}

impl<'a> From<&'a str> for XRPAmount {
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

impl From<String> for XRPAmount {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<BigDecimal> for XRPAmount {
    fn from(value: BigDecimal) -> Self {
        Self(value.to_string().into())
    }
}

impl From<f64> for XRPAmount {
    fn from(value: f64) -> Self {
        Self(value.to_string().into())
    }
}

impl From<u32> for XRPAmount {
    fn from(value: u32) -> Self {
        Self(value.to_string().into())
    }
}

impl TryFrom<Value> for XRPAmount {
    type Error = XRPLModelException;

    fn try_from(value: Value) -> XRPLModelResult<Self, Self::Error> {
        match serde_json::to_string(&value) {
            Ok(amount_string) => {
                let amount_string = amount_string.clone().replace("\"", "");
                Ok(Self(amount_string.into()))
            }
            Err(serde_error) => Err(serde_error.into()),
        }
    }
}

impl TryInto<f64> for XRPAmount {
    type Error = XRPLModelException;

    fn try_into(self) -> XRPLModelResult<f64, Self::Error> {
        Ok(self.0.parse::<f64>()?)
    }
}

impl TryInto<u32> for XRPAmount {
    type Error = XRPLModelException;

    fn try_into(self) -> XRPLModelResult<u32, Self::Error> {
        Ok(self.0.parse::<u32>()?)
    }
}

impl TryInto<BigDecimal> for XRPAmount {
    type Error = XRPLModelException;

    fn try_into(self) -> XRPLModelResult<BigDecimal, Self::Error> {
        Ok(BigDecimal::from_str(&self.0)?)
    }
}

impl PartialOrd for XRPAmount {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for XRPAmount {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let self_decimal: BigDecimal = self.clone().try_into().unwrap();
        let other_decimal: BigDecimal = other.clone().try_into().unwrap();
        self_decimal.cmp(&other_decimal)
    }
}
