#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Source {
    pub id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<SourceType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub citations: Vec<SourceCitation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<Uri>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mediator: Option<AgentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<AgentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<AgentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_of: Option<SourceReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub titles: Vec<TextValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<Uri>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub descriptions: Vec<TextValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<AgentReference>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceType {
    Collection,
    PhysicalArtifact,
    DigitalArtifact,
    Record,
    Custom(Uri),
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceCitation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    pub value: String,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceReference {
    pub source: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<SourceReferenceQualifier>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceReferenceQualifier {
    /// start index, end index (both inclusive)
    CharacterRegion(u64, u64),
    /// x1,y1,x2,y2 where 0 is the top-left and u32::MAX is the bottom-right
    RelativeRectangleRegion(u32, u32, u32, u32),
    /// x,y,w,h in pixels
    AbsoluteRectangleRegion(u32, u32, u32, u32),
    /// start,end in milliseconds
    TimeRegion(u64, u64),
    /// start,end 0-based both inclusive
    Pages(u32, u32),
}
