use alloc::borrow::Cow;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::RequestMethod, Model, XRPLModelException, XRPLModelResult};

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Request};

/// Required fields for requesting a DepositPreauth if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct DepositPreauth<'a> {
    pub authorized: Cow<'a, str>,
    pub owner: Cow<'a, str>,
}

/// Required fields for requesting a DirectoryNode if not
/// querying by object ID.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Directory<'a> {
    pub dir_root: Cow<'a, str>,
    pub owner: Cow<'a, str>,
    pub sub_index: Option<u8>,
}

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Escrow<'a> {
    pub owner: Cow<'a, str>,
    pub seq: u64,
}

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Offer<'a> {
    pub account: Cow<'a, str>,
    pub seq: u64,
}

/// Required fields for requesting a Ticket, if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Ticket<'a> {
    pub owner: Cow<'a, str>,
    pub ticket_sequence: u64,
}

/// Required fields for requesting a RippleState.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct RippleState<'a> {
    pub account: Cow<'a, str>,
    pub currency: Cow<'a, str>,
}

/// The ledger_entry method returns a single ledger object
/// from the XRP Ledger in its raw format. See ledger formats
/// for information on the different types of objects you can
/// retrieve.
///
/// See Ledger Formats:
/// `<https://xrpl.org/ledger-data-formats.html#ledger-data-formats>`
///
/// See Ledger Entry:
/// `<https://xrpl.org/ledger_entry.html#ledger_entry>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LedgerEntry<'a> {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    pub account_root: Option<String>,
    /// If true, return the requested ledger object's contents as a
    /// hex string in the XRP Ledger's binary format. Otherwise, return
    /// data in JSON format. The default is false.
    pub binary: Option<bool>,
    pub check: Option<String>,
    pub deposit_preauth: Option<DepositPreauth<'a>>,
    pub directory: Option<Directory<'a>>,
    pub escrow: Option<Escrow<'a>>,
    pub index: Option<String>,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest>,
    pub offer: Option<Offer<'a>>,
    pub payment_channel: Option<String>,
    pub ripple_state: Option<RippleState<'a>>,
    pub ticket: Option<Ticket<'a>>,
}

impl Model for LedgerEntry {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self._get_field_error()
    }
}

impl LedgerEntryError for LedgerEntry {
    fn _get_field_error(&self) -> XRPLModelResult<()> {
        let mut signing_methods: u32 = 0;
        for method in [
            self.index.clone(),
            self.account_root.clone(),
            self.check.clone(),
        ] {
            if method.is_some() {
                signing_methods += 1
            }
        }
        if self.directory.is_some() {
            signing_methods += 1
        }
        if self.offer.is_some() {
            signing_methods += 1
        }
        if self.ripple_state.is_some() {
            signing_methods += 1
        }
        if self.escrow.is_some() {
            signing_methods += 1
        }
        if self.payment_channel.is_some() {
            signing_methods += 1
        }
        if self.deposit_preauth.is_some() {
            signing_methods += 1
        }
        if self.ticket.is_some() {
            signing_methods += 1
        }
        if signing_methods != 1 {
            Err(XRPLModelException::ExpectedOneOf(&[
                "index",
                "account_root",
                "check",
                "directory",
                "offer",
                "ripple_state",
                "escrow",
                "payment_channel",
                "deposit_preauth",
                "ticket",
            ]))
        } else {
            Ok(())
        }
    }
}

impl Request for LedgerEntry {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl LedgerEntry {
    pub fn new(
        id: Option<String>,
        account_root: Option<String>,
        binary: Option<bool>,
        check: Option<String>,
        deposit_preauth: Option<DepositPreauth<'a>>,
        directory: Option<Directory<'a>>,
        escrow: Option<Escrow<'a>>,
        index: Option<String>,
        ledger_hash: Option<String>,
        ledger_index: Option<LedgerIndex>,
        offer: Option<Offer<'a>>,
        payment_channel: Option<String>,
        ripple_state: Option<RippleState<'a>>,
        ticket: Option<Ticket<'a>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::LedgerEntry,
                id,
            },
            index,
            account_root,
            check,
            payment_channel,
            deposit_preauth,
            directory,
            escrow,
            offer,
            ripple_state,
            ticket,
            binary,
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
        }
    }
}

pub trait LedgerEntryError {
    #[allow(clippy::result_large_err)]
    fn _get_field_error(&self) -> XRPLModelResult<()>;
}

#[cfg(test)]
mod test_ledger_entry_errors {
    use super::Offer;
    use crate::models::Model;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_fields_error() {
        let ledger_entry = LedgerEntry::new(
            None,
            Some("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Offer {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
                seq: 359,
            }),
            None,
            None,
            None,
        );
        let _expected = XRPLModelException::ExpectedOneOf(&[
            "index",
            "account_root",
            "check",
            "directory",
            "offer",
            "ripple_state",
            "escrow",
            "payment_channel",
            "deposit_preauth",
            "ticket",
        ]);
        assert_eq!(
            ledger_entry.validate().unwrap_err().to_string().as_str(),
            "Expected one of: index, account_root, check, directory, offer, ripple_state, escrow, payment_channel, deposit_preauth, ticket"
        );
    }

    #[test]
    fn test_serde() {
        let req = LedgerEntry::new(
            None,
            Some("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Offer {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
                seq: 359,
            }),
            None,
            None,
            None,
        );
        let serialized = serde_json::to_string(&req).unwrap();

        let deserialized: LedgerEntry = serde_json::from_str(&serialized).unwrap();

        assert_eq!(req, deserialized);
    }
}
