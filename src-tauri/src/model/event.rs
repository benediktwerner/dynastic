#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct  Event {
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
    pub media: Vec<SourceReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<EventRole>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<()>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<ConfidenceLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
    pub person: PersonReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_role_type: Option<EventRoleType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventRoleType {
    Principal,
    Participant,
    Official,
    Witness,
    Custom(Uri),
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    Adoption,
    AdultChristening,
    Annulment,
    Baptism,
    BarMitzvah,
    BatMitzvah,
    Birth,
    Blessing,
    Burial,
    Census,
    Christening,
    Circumcision,
    Confirmation,
    Cremation,
    Death,
    Divorce,
    DivorceFiling,
    Education,
    Engagement,
    Emigration,
    Excommunication,
    FirstCommunion,
    Funeral,
    Immigration,
    LandTransaction,
    Marriage,
    MilitaryAward,
    MilitaryDischarge,
    Mission,
    MoveFrom,
    MoveTo,
    Naturalization,
    Ordination,
    Retirement,
    Custom(Uri),
}
