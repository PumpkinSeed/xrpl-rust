use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, Request};

/// The server_info command asks the server for a
/// human-readable version of various information about the
/// rippled server being queried.
///
/// See Server Info:
/// `<https://xrpl.org/server_info.html#server_info>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerInfo {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
}

impl<'a> Model for ServerInfo {}

impl<'a> Request<'a> for ServerInfo {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl<'a> ServerInfo {
    pub fn new(id: Option<String>) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::ServerInfo,
                id,
            },
        }
    }
}
