#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<()>, // TODO
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<ConfidenceLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<NameType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name_forms: Vec<NameForm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NameType {
    BirthName,
    MarriedName,
    AlsoKnownAs,
    Nickname,
    AdoptiveName,
    FormalName,
    ReligiousName,
    Custom(Uri),
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NameForm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<NamePart>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NamePart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub part_type: Option<NamePartType>,
    pub value: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<NamePartQualifier>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamePartType {
    Prefix,
    Suffix,
    Given,
    Surname,
    Custom(Uri),
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NamePartQualifier {
    Title,
    Primary,
    Secondary,
    Middle,
    Familiar,
    Religious,
    Family,
    Maiden,
    Patronymic,
    Matronymic,
    Geographic,
    Occupational,
    Characteristic,
    Postnom,
    Particle,
    RootName(String),
    Custom(Uri, Option<String>),
}
