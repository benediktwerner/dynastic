#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fact {
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
    pub fact_type: FactType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<FactQualifier>,
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactQualifier {
    Age(u8),
    Cause(String),
    Religion(String),
    Transport(String),
    NonConsensual(#[serde(default, skip_serializing_if = "Option::is_none")] Option<String>),
    Custom(
        Uri,
        #[serde(default, skip_serializing_if = "Option::is_none")] Option<String>,
    ),
}

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactType {
    Adoption,
    AdultChristening,
    Amnesty,
    AncestralHall,
    AncestralPoem,
    Apprenticeship,
    Arrest,
    Award,
    Baptism,
    BarMitzvah,
    BatMitzvah,
    Birth,
    BirthNotice,
    Blessing,
    Branch,
    Burial,
    Caste,
    Census,
    Christening,
    Circumcision,
    Clan,
    Confirmation,
    Court,
    Cremation,
    Death,
    Education,
    EducationEnrollment,
    Emigration,
    Enslavement,
    Ethnicity,
    Excommunication,
    FirstCommunion,
    Funeral,
    GenderChange,
    GenerationNumber,
    Graduation,
    Heimat,
    Immigration,
    Imprisonment,
    Inquest,
    LandTransaction,
    Language,
    Living,
    MaritalStatus,
    Medical,
    MilitaryAward,
    MilitaryDischarge,
    MilitaryDraftRegistration,
    MilitaryInduction,
    MilitaryService,
    Mission,
    MoveFrom,
    MoveTo,
    MultipleBirth,
    NationalId,
    Nationality,
    Naturalization,
    NumberOfChildren,
    NumberOfMarriages,
    Obituary,
    OfficialPosition,
    Occupation,
    Ordination,
    Pardon,
    PhysicalDescription,
    Probate,
    Property,
    Race,
    Religion,
    Residence,
    Retirement,
    Stillbirth,
    TaxAssessment,
    Tribe,
    Will,
    Visit,
    Yahrzeit,
    Annulment,
    CommonLawMarriage,
    CivilUnion,
    Divorce,
    DivorceFiling,
    DomesticPartnership,
    Engagement,
    Marriage,
    MarriageBanns,
    MarriageContract,
    MarriageLicense,
    MarriageNotice,
    Separation,
    AdoptiveParent,
    BiologicalParent,
    ChildOrder,
    EnteringHeir,
    ExitingHeir,
    FosterParent,
    GuardianParent,
    StepParent,
    SociologicalParent,
    SurrogateParent,
    Custom(Uri),
}
