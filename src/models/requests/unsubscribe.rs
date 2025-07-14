use alloc::vec::Vec;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    currency::Currency,
    default_false,
    requests::{subscribe::StreamParameter, RequestMethod},
    Model,
};

use super::{CommonFields, Request};

/// Format for elements in the `books` array for Unsubscribe only.
///
/// See Unsubscribe:
/// `<https://xrpl.org/unsubscribe.html>`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct UnsubscribeBook {
    pub taker_gets: Currency,
    pub taker_pays: Currency,
    #[serde(default = "default_false")]
    pub both: Option<bool>,
}

/// The unsubscribe command tells the server to stop
/// sending messages for a particular subscription or set
/// of subscriptions.
///
/// Note: WebSocket API only.
///
/// See Unsubscribe:
/// `<https://xrpl.org/unsubscribe.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Unsubscribe {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// Array of unique account addresses to stop receiving updates
    /// for, in the XRP Ledger's base58 format. (This only stops
    /// those messages if you previously subscribed to those accounts
    /// specifically. You cannot use this to filter accounts out of
    /// the general transactions stream.)
    pub accounts: Option<Vec<String>>,
    /// Like accounts, but for accounts_proposed subscriptions that
    /// included not-yet-validated transactions.
    pub accounts_proposed: Option<Vec<String>>,
    /// Array of objects defining order books to unsubscribe
    /// from, as explained below.
    pub books: Option<Vec<UnsubscribeBook>>,
    #[serde(skip_serializing)]
    pub broken: Option<String>,
    /// Array of string names of generic streams to unsubscribe
    /// from, including ledger, server, transactions,
    /// and transactions_proposed.
    pub streams: Option<Vec<StreamParameter>>,
}

impl Model for Unsubscribe {}

impl Request for Unsubscribe {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl Unsubscribe {
    pub fn new(
        id: Option<String>,
        accounts: Option<Vec<String>>,
        accounts_proposed: Option<Vec<String>>,
        books: Option<Vec<UnsubscribeBook>>,
        broken: Option<String>,
        streams: Option<Vec<StreamParameter>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::Unsubscribe,
                id,
            },
            books,
            streams,
            accounts,
            accounts_proposed,
            broken,
        }
    }
}
