use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, Request};

/// The random command provides a random number to be used
/// as a source of entropy for random number generation
/// by clients.
///
/// See Random:
/// `<https://xrpl.org/random.html#random>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Random {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
}

impl Model for Random {}

impl<'a> Request<'a> for Random {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl Random {
    pub fn new(id: Option<String>) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::Random,
                id,
            },
        }
    }
}
