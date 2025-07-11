use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, Request};

/// The ping command returns an acknowledgement, so that
/// clients can test the connection status and latency.
///
/// See Ping:
/// `<https://xrpl.org/ping.html#ping>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Ping {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
}

impl Model for Ping {}

impl<'a> Request<'a> for Ping {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl Ping {
    pub fn new(id: Option<String>) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::Ping,
                id,
            },
        }
    }
}
