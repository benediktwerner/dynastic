#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod agent;
mod date;
mod event;
mod fact;
mod name;
mod person;
mod place;
mod relationship;
mod source;

pub use agent::*;
pub use source::*;
pub use date::*;
pub use event::*;
pub use fact::*;
pub use name::*;
pub use person::*;
pub use place::*;
pub use relationship::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Root {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
    pub persons: Vec<Person>,
    pub relationships: Vec<Relationship>,
    pub sources: Vec<Source>,
    pub agents: Vec<Agent>,
    pub events: Vec<Event>,
    pub places: Vec<PlaceDescription>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Uri(String);

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Lang(String);

type Timestamp = chrono::DateTime<chrono::Utc>;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Note {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Attribution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributor: Option<AgentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<AgentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    pub value: String,
}

fn is_false(val: &bool) -> bool {
    !val
}
