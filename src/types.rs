use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsMsg {
    pub event: String,
    pub payload: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub url: String,
    pub content: String,
    pub account: Account,
    #[serde(rename = "media_attachments")]
    media_attachments: Vec<Value>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "display_name")]
    pub display_name: String,
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{}: {}", self.account.display_name, self.content)
        )
    }
}
