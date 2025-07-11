use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, Request};

/// The ledger_closed method returns the unique identifiers of
/// the most recently closed ledger. (This ledger is not
/// necessarily validated and immutable yet.)
///
/// See Ledger Closed:
/// `<https://xrpl.org/ledger_closed.html#ledger_closed>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LedgerCurrent {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
}

impl Model for LedgerCurrent {}

impl Request for LedgerCurrent {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl LedgerCurrent {
    pub fn new(id: Option<String>) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::LedgerCurrent,
                id,
            },
        }
    }
}
