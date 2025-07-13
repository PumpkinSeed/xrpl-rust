use serde::{Deserialize, Serialize};

/// See Ping:
/// `<https://xrpl.org/ping.html#ping>`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Ping {}
