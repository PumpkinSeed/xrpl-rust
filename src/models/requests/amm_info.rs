use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Currency, Model};

use super::{CommonFields, Request};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AMMInfo {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    pub amm_account: Option<String>,
    pub asset: Option<Currency>,
    pub asset2: Option<Currency>,
}

impl Model for AMMInfo {}

impl Request for AMMInfo {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl AMMInfo {
    pub fn new(
        id: Option<String>,
        amm_account: Option<String>,
        asset: Option<Currency>,
        asset2: Option<Currency>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: super::RequestMethod::AMMInfo,
                id,
            },
            amm_account,
            asset,
            asset2,
        }
    }
}
