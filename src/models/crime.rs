use serde::{Deserialize, Serialize};

/// A latitude/longitude pair.
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

/// A geographic area to search for crimes or outcomes.
#[derive(Debug, Clone, PartialEq)]
pub enum Area {
    /// Search within a 1 mile radius of a point.
    Point(Coordinate),
    /// Search within a custom polygon defined by a list of coordinates.
    Custom(Vec<Coordinate>),
    /// Search at a specific location ID (returned by other API methods).
    LocationId(u64),
}

/// A category of crime (e.g. "Burglary", "Drugs").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrimeCategory {
    /// Category identifier (slug format, e.g. "anti-social-behaviour").
    pub url: String,
    /// Human-readable category name.
    pub name: String,
}

/// The date when crime data was last updated
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrimeLastUpdated {
    /// Month of the latest crime data in ISO date format.
    /// The day is irrelevant and is only there to keep a standard formatted date.
    pub date: String,
}

/// A crime record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crime {
    /// Crime category (e.g. "anti-social-behaviour", "burglary").
    pub category: String,
    /// 64-character unique identifier for the crime.
    pub persistent_id: String,
    /// For BTP locations, the type of location at which this crime was recorded.
    pub location_subtype: String,
    /// API identifier for the crime. Not a police identifier.
    pub id: u64,
    /// Approximate location of the incident. `None` for crimes with no location.
    pub location: Option<Location>,
    /// Extra information about the crime (if applicable).
    pub context: String,
    /// Month the crime was recorded (format: `YYYY-MM`).
    pub month: String,
    /// Either "Force" or "BTP" (British Transport Police). `None` for crimes with no location.
    pub location_type: Option<String>,
    /// The latest recorded outcome for the crime, if available.
    pub outcome_status: Option<OutcomeStatus>,
}

/// Approximate location of a crime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// Latitude.
    pub latitude: String,
    /// The approximate street the crime occurred on.
    pub street: Street,
    /// Longitude.
    pub longitude: String,
}

/// A street associated with a crime location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Street {
    /// Unique identifier for the street.
    pub id: u64,
    /// Name of the location. This is only an approximation.
    pub name: String,
}

/// The latest outcome of a crime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeStatus {
    /// Category of the outcome.
    pub category: OutcomeCategory,
    /// Date of the outcome (format: `YYYY-MM`).
    pub date: String,
}

/// Outcome category. Deserializes from both kebab-case codes (e.g. "local-resolution")
/// and full names (e.g. "Local resolution") as different API endpoints use different formats.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutcomeCategory {
    #[serde(rename = "awaiting-court-result", alias = "Awaiting court outcome")]
    AwaitingCourtResult,
    #[serde(
        rename = "court-result-unavailable",
        alias = "Court result unavailable"
    )]
    CourtResultUnavailable,
    #[serde(rename = "unable-to-proceed", alias = "Court case unable to proceed")]
    UnableToProceed,
    #[serde(rename = "local-resolution", alias = "Local resolution")]
    LocalResolution,
    #[serde(
        rename = "no-further-action",
        alias = "Investigation complete; no suspect identified"
    )]
    NoFurtherAction,
    #[serde(
        rename = "deprived-of-property",
        alias = "Offender deprived of property"
    )]
    DeprivedOfProperty,
    #[serde(rename = "fined", alias = "Offender fined")]
    Fined,
    #[serde(
        rename = "absolute-discharge",
        alias = "Offender given absolute discharge"
    )]
    AbsoluteDischarge,
    #[serde(rename = "cautioned", alias = "Offender given a caution")]
    Cautioned,
    #[serde(
        rename = "drugs-possession-warning",
        alias = "Offender given a drugs possession warning"
    )]
    DrugsPossessionWarning,
    #[serde(
        rename = "penalty-notice-issued",
        alias = "Offender given a penalty notice"
    )]
    PenaltyNoticeIssued,
    #[serde(
        rename = "community-penalty",
        alias = "Offender given community sentence"
    )]
    CommunityPenalty,
    #[serde(
        rename = "conditional-discharge",
        alias = "Offender given conditional discharge"
    )]
    ConditionalDischarge,
    #[serde(
        rename = "suspended-sentence",
        alias = "Offender given suspended prison sentence"
    )]
    SuspendedSentence,
    #[serde(rename = "imprisoned", alias = "Offender sent to prison")]
    Imprisoned,
    #[serde(
        rename = "other-court-disposal",
        alias = "Offender otherwise dealt with"
    )]
    OtherCourtDisposal,
    #[serde(
        rename = "compensation",
        alias = "Offender ordered to pay compensation"
    )]
    Compensation,
    #[serde(
        rename = "sentenced-in-another-case",
        alias = "Suspect charged as part of another case"
    )]
    SentencedInAnotherCase,
    #[serde(rename = "charged", alias = "Suspect charged")]
    Charged,
    #[serde(rename = "not-guilty", alias = "Defendant found not guilty")]
    NotGuilty,
    #[serde(
        rename = "sent-to-crown-court",
        alias = "Defendant sent to Crown Court"
    )]
    SentToCrownCourt,
    #[serde(rename = "unable-to-prosecute", alias = "Unable to prosecute suspect")]
    UnableToProsecute,
    #[serde(
        rename = "formal-action-not-in-public-interest",
        alias = "Formal action is not in the public interest"
    )]
    FormalActionNotInPublicInterest,
    #[serde(
        rename = "action-taken-by-another-organisation",
        alias = "Action to be taken by another organisation"
    )]
    ActionTakenByAnotherOrganisation,
    #[serde(
        rename = "further-investigation-not-in-public-interest",
        alias = "Further investigation is not in the public interest"
    )]
    FurtherInvestigationNotInPublicInterest,
    #[serde(
        rename = "further-action-not-in-public-interest",
        alias = "Further action is not in the public interest"
    )]
    FurtherActionNotInPublicInterest,
    #[serde(rename = "under-investigation", alias = "Under investigation")]
    UnderInvestigation,
    #[serde(
        rename = "status-update-unavailable",
        alias = "Status update unavailable"
    )]
    StatusUpdateUnavailable,
}

/// Outcome category detail object returned by outcome endpoints.
/// Contains both the machine-readable code and human-readable name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeDetail {
    /// Machine-readable category code.
    pub code: OutcomeCategory,
    /// Human-readable category name.
    pub name: String,
}

/// A street-level outcome record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    /// The outcome category.
    pub category: OutcomeDetail,
    /// Date of the outcome (format: `YYYY-MM`).
    pub date: String,
    /// Identifier for the suspect/offender, where available.
    pub person_id: Option<String>,
    /// The crime this outcome relates to.
    pub crime: Crime,
}

/// All outcomes for a specific crime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrimeOutcomes {
    /// The crime.
    pub crime: Crime,
    /// List of outcomes for this crime.
    pub outcomes: Vec<CrimeOutcome>,
}

/// An individual outcome within a [`CrimeOutcomes`] response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrimeOutcome {
    /// The outcome category.
    pub category: OutcomeDetail,
    /// Date of the outcome (format: `YYYY-MM`).
    pub date: String,
    /// Identifier for the suspect/offender, where available.
    pub person_id: Option<String>,
}
