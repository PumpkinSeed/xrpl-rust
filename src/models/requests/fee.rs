use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, Request};

/// The fee command reports the current state of the open-ledger
/// requirements for the transaction cost. This requires the
/// FeeEscalation amendment to be enabled. This is a public
/// command available to unprivileged users.
///
/// See Fee:
/// `<https://xrpl.org/fee.html#fee>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Fee {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
}

impl Model for Fee{}

impl<'a> Request<'a> for Fee {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl Fee {
    pub fn new(id: Option<String>) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::Fee,
                id,
            },
        }
    }
}
