use crate::models::amount::IssuedCurrencyAmount;
use crate::models::currency::ToAmount;
use crate::models::Model;
use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct IssuedCurrency {
    pub currency: String,
    pub issuer: String,
}

impl Model for IssuedCurrency {}

impl ToAmount<IssuedCurrencyAmount> for IssuedCurrency {
    fn to_amount(&self, value: String) -> IssuedCurrencyAmount {
        IssuedCurrencyAmount::new(self.currency.clone(), self.issuer.clone(), value)
    }
}

impl IssuedCurrency {
    pub fn new(currency: String, issuer: String) -> Self {
        Self { currency, issuer }
    }
}

impl From<IssuedCurrencyAmount> for IssuedCurrency {
    fn from(value: IssuedCurrencyAmount) -> Self {
        Self {
            currency: value.currency,
            issuer: value.issuer,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let issued_currency =
            IssuedCurrency::new("TST".into(), "rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd".into());
        let issued_currency_json = serde_json::to_string(&issued_currency).unwrap();
        let actual = issued_currency_json.as_str();
        let expected = r#"{"currency":"TST","issuer":"rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd"}"#;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize() {
        let issued_currency_json =
            r#"{"currency":"TST","issuer":"rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd"}"#;
        let actual = serde_json::from_str(issued_currency_json).unwrap();
        let expected =
            IssuedCurrency::new("TST".into(), "rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd".into());

        assert_eq!(expected, actual);
    }
}
