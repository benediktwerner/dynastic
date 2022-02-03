#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlaceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<u64>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq,  Eq, Hash,Serialize, Deserialize)]
pub struct PlaceDescription {
    pub id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<ConfidenceLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<()>, // TODO
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<TextValue>,
    pub place_type: Option<Uri>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<PlaceReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal_description: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial_description: Option<Uri>,
}
