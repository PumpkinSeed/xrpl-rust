use serde::{Deserialize, Serialize};

/// Represents an AccountRoot ledger object in the XRP Ledger.
/// This object type represents a single account, its settings, and XRP balance.
///
/// See AccountRoot:
/// `<https://xrpl.org/accountroot.html>`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    /// The identifying address of this account
    pub account: String,
    /// The identifying hash of the transaction that most recently modified
    /// this object
    #[serde(rename = "AccountTxnID")]
    pub account_txn_id: String,
    /// The account's current XRP balance in drops
    pub balance: String,
    /// The domain associated with this account. The raw domain value is a
    /// hex string representing the ASCII for the domain
    pub domain: Option<String>,
    /// Hash of an email address to be used for generating an avatar image
    pub email_hash: Option<String>,
    /// Various boolean flags enabled for this account
    pub flags: u32,
    /// The type of ledger object. For AccountRoot objects, this is always
    /// "AccountRoot"
    pub ledger_entry_type: String,
    /// Public key for sending encrypted messages to this account
    pub message_key: Option<String>,
    /// Number of objects this account owns in the ledger, which contributes
    /// to its owner reserve
    pub owner_count: u32,
    /// Identifying hash of the previous transaction that modified this object
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    /// Ledger index of the ledger containing the previous transaction that
    /// modified this object
    pub previous_txn_lgr_seq: u32,
    /// The identifying address of a key pair that can be used to authorize
    /// transactions for this account instead of the master key
    pub regular_key: Option<String>,
    /// The sequence number of the next valid transaction for this account
    pub sequence: u32,
    /// The rate to charge when users transfer this account's issued currencies,
    /// represented as billionths of a unit. A value of 0 means no fee
    pub transfer_rate: Option<u32>,
    /// The unique ID of this ledger entry
    #[serde(rename = "index")]
    pub index: String,
}

/// Response format for the ledger_entry method, which returns a single ledger
/// object from the XRP Ledger in its raw format.
///
/// See Ledger Entry:
/// `<https://xrpl.org/ledger_entry.html>`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LedgerEntry {
    /// The unique ID of this ledger entry.
    pub index: String,
    /// The ledger index of the ledger that was used when retrieving this data.
    pub ledger_index: Option<u32>,
    /// The identifying hash of the ledger version used to retrieve this data
    pub ledger_hash: Option<String>,
    /// Object containing the data of this ledger entry, according to the
    /// ledger format. Omitted if "binary": true specified.
    pub node: Option<Node>,
    /// The binary representation of the ledger object, as hexadecimal.
    /// Only present if "binary": true specified.
    pub node_binary: Option<String>,
    /// (Clio server only) The ledger index where the ledger entry object was
    /// deleted. Only present if include_deleted parameter is set.
    pub deleted_ledger_index: Option<String>,
    /// Whether this data is from a validated ledger version
    pub validated: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_entry_deserialize() {
        let json = r#"{
            "index": "13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8",
            "ledger_hash": "31850E8E48E76D1064651DF39DF4E9542E8C90A9A9B629F4DE339EB3FA74F726",
            "ledger_index": 61966146,
            "node": {
                "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                "AccountTxnID": "4E0AA11CBDD1760DE95B68DF2ABBE75C9698CEB548BEA9789053FCB3EBD444FB",
                "Balance": "424021949",
                "Domain": "6D64756F31332E636F6D",
                "EmailHash": "98B4375E1D753E5B91627516F6D70977",
                "Flags": 9568256,
                "LedgerEntryType": "AccountRoot",
                "MessageKey": "0000000000000000000000070000000300",
                "OwnerCount": 12,
                "PreviousTxnID": "4E0AA11CBDD1760DE95B68DF2ABBE75C9698CEB548BEA9789053FCB3EBD444FB",
                "PreviousTxnLgrSeq": 61965653,
                "RegularKey": "rD9iJmieYHn8jTtPjwwkW2Wm9sVDvPXLoJ",
                "Sequence": 385,
                "TransferRate": 4294967295,
                "index": "13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8"
            },
            "validated": true
        }"#;

        let result: LedgerEntry = serde_json::from_str(json).unwrap();

        assert_eq!(
            result.index,
            "13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8"
        );
        assert_eq!(result.ledger_index, Some(61966146));
        assert_eq!(
            result.ledger_hash,
            Some("31850E8E48E76D1064651DF39DF4E9542E8C90A9A9B629F4DE339EB3FA74F726".into())
        );
        assert_eq!(result.validated, Some(true));

        let node = result.node.unwrap();
        assert_eq!(node.account, "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn");
        assert_eq!(
            node.account_txn_id,
            "4E0AA11CBDD1760DE95B68DF2ABBE75C9698CEB548BEA9789053FCB3EBD444FB"
        );
        assert_eq!(node.balance, "424021949");
        assert_eq!(node.domain, Some("6D64756F31332E636F6D".into()));
        assert_eq!(
            node.email_hash,
            Some("98B4375E1D753E5B91627516F6D70977".into())
        );
        assert_eq!(node.flags, 9568256);
        assert_eq!(node.ledger_entry_type, "AccountRoot");
        assert_eq!(
            node.message_key,
            Some("0000000000000000000000070000000300".into())
        );
        assert_eq!(node.owner_count, 12);
        assert_eq!(
            node.previous_txn_id,
            "4E0AA11CBDD1760DE95B68DF2ABBE75C9698CEB548BEA9789053FCB3EBD444FB"
        );
        assert_eq!(node.previous_txn_lgr_seq, 61965653);
        assert_eq!(
            node.regular_key,
            Some("rD9iJmieYHn8jTtPjwwkW2Wm9sVDvPXLoJ".into())
        );
        assert_eq!(node.sequence, 385);
        assert_eq!(node.transfer_rate, Some(4294967295));
        assert_eq!(
            node.index,
            "13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8"
        );
    }
}
