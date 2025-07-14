use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{amount::XRPAmount, requests::RequestMethod, Model};

use super::{CommonFields, Request};

/// The channel_verify method checks the validity of a signature
/// that can be used to redeem a specific amount of XRP from a
/// payment channel.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ChannelVerify {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields,
    /// The amount of XRP, in drops, the provided signature authorizes.
    pub amount: XRPAmount,
    /// The Channel ID of the channel that provides the XRP.
    /// This is a 64-character hexadecimal string.
    pub channel_id: String,
    /// The public key of the channel and the key pair that was used to
    /// create the signature, in hexadecimal or the XRP Ledger's
    /// base58 format.
    pub public_key: String,
    /// The signature to verify, in hexadecimal.
    pub signature: String,
}

impl Model for ChannelVerify {}

impl Request for ChannelVerify {
    fn get_common_fields(&self) -> &CommonFields {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields {
        &mut self.common_fields
    }
}

impl ChannelVerify {
    pub fn new(
        id: Option<String>,
        amount: XRPAmount,
        channel_id: String,
        public_key: String,
        signature: String,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::ChannelVerify,
                id,
            },
            channel_id,
            amount,
            public_key,
            signature,
        }
    }
}
