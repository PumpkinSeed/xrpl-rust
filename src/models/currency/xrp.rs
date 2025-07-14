use crate::models::currency::ToAmount;
use crate::models::{amount::XRPAmount, XRPLModelException};
use crate::models::{Model, XRPLModelResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct XRP {
    pub currency: String,
}

impl Model for XRP {
    fn get_errors(&self) -> XRPLModelResult<()> {
        if self.currency != "XRP" {
            Err(XRPLModelException::InvalidValue {
                field: "currency".into(),
                expected: "XRP".into(),
                found: self.currency.clone().into(),
            })
        } else {
            Ok(())
        }
    }
}

impl ToAmount<XRPAmount> for XRP {
    fn to_amount(&self, value: String) -> XRPAmount {
        XRPAmount(value)
    }
}

impl XRP {
    pub fn new() -> Self {
        Self {
            currency: "XRP".into(),
        }
    }
}

impl From<XRPAmount> for XRP {
    fn from(_value: XRPAmount) -> Self {
        Self {
            currency: "XRP".into(),
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let xrp = XRP::new();
        let xrp_json = serde_json::to_string(&xrp).unwrap();
        let actual = xrp_json.as_str();
        let expected = r#"{"currency":"XRP"}"#;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize() {
        let xrp_json = r#"{"currency":"XRP"}"#;
        let actual = serde_json::from_str(xrp_json).unwrap();
        let expected = XRP::new();

        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod test_amount_currency_conversion {
    use super::*;

    #[test]
    fn test_currency_to_amount() {
        let xrp = XRP::new();
        let actual = xrp.to_amount("10".into());
        let expected: XRPAmount = "10".into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_currency_from_amount() {
        let xrp_amount: XRPAmount = "10".into();
        let actual = XRP::from(xrp_amount);
        let expected = XRP::new();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_amount_into_currency() {
        let xrp_amount: XRPAmount = "10".into();
        let actual: XRP = xrp_amount.into();
        let expected = XRP::new();

        assert_eq!(expected, actual);
    }
}
