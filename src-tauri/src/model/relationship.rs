#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Relationship {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<RelationshipType>,
    pub person1: PersonReference,
    pub person2: PersonReference,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facts: Vec<Fact>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    AncestorDescendant,
    Couple,
    EnslavedBy,
    Godparent,
    ParentChild,
    Custom(Uri),
}
