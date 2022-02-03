#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Agent {
    pub id: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<TextValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Uri>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openid: Option<Uri>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<OnlineAccount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Email>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phones: Vec<Phone>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonReference>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Phone(String);

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentReference(u64);

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub street: Vec<String>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OnlineAccount {
    pub service_homepage: Uri,
    pub account_name: String,
}
