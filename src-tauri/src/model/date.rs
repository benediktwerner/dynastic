use std::num::NonZeroU8;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Date {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formal: Option<GedcomxDate>,
    // TODO: allow additional date in other calendar (Julian, Hebrew, etc.)
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GedcomxDate {
    Simple(GedcomxDateSimple),
    Range(GedcomxDateRange),
    Recurring(GedcomxDateRecurring),
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GedcomxDateSimple {
    pub date: GregorianDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Time>,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub approximate: bool,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GregorianDate {
    pub year: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<NonZeroU8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<NonZeroU8>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Time {
    pub hours: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tz_offset_minutes: Option<i32>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GedcomxDateRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<GregorianDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<GregorianDateTimeOrDuration>,
    pub approximate: bool,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GedcomxDateRecurring {
    pub start: GregorianDateTime,
    pub end: GregorianDateTimeOrDuration,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GregorianDateTime {
    pub date: GregorianDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Time>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GregorianDateTimeOrDuration {
    DateTime(GregorianDateTime),
    Duration(GregorianDuration),
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GregorianDuration {
    pub years: u32,
    pub months: u8,
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}
