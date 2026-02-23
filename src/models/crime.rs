use serde::Deserialize;

/// A latitude/longitude pair.
#[derive(Debug, Clone)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

/// A geographic area to search for crimes.
#[derive(Debug, Clone)]
pub enum Area {
    /// Search within a 1 mile radius of a point.
    Point(Coordinate),
    /// Search within a custom polygon defined by a list of coordinates.
    Custom(Vec<Coordinate>),
}

/// A category of crime (e.g. "Burglary", "Drugs").
#[derive(Debug, Deserialize)]
pub struct CrimeCategory {
    /// Category identifier (slug format, e.g. "anti-social-behaviour").
    pub url: String,
    /// Human-readable category name.
    pub name: String,
}

/// The date when crime data was last updated
#[derive(Debug, Deserialize)]
pub struct CrimeLastUpdated {
    /// Month of the latest crime data in ISO date format.
    /// The day is irrelevant and is only there to keep a standard formatted date.
    pub date: String,
}

/// A street-level crime record.
#[derive(Debug, Deserialize)]
pub struct Crime {
    /// Crime category (e.g. "anti-social-behaviour", "burglary").
    pub category: String,
    /// 64-character unique identifier for the crime.
    pub persistent_id: String,
    /// For BTP locations, the type of location at which this crime was recorded.
    pub location_subtype: String,
    /// API identifier for the crime. Not a police identifier.
    pub id: u64,
    /// Approximate location of the incident.
    pub location: Location,
    /// Extra information about the crime (if applicable).
    pub context: String,
    /// Month the crime was recorded (format: `YYYY-MM`).
    pub month: String,
    /// Either "Force" or "BTP" (British Transport Police).
    pub location_type: String,
    /// The latest recorded outcome for the crime, if available.
    pub outcome_status: Option<OutcomeStatus>,
}

/// Approximate location of a crime.
#[derive(Debug, Deserialize)]
pub struct Location {
    /// Latitude.
    pub latitude: String,
    /// The approximate street the crime occurred on.
    pub street: Street,
    /// Longitude.
    pub longitude: String,
}

/// A street associated with a crime location.
#[derive(Debug, Deserialize)]
pub struct Street {
    /// Unique identifier for the street.
    pub id: u64,
    /// Name of the location. This is only an approximation.
    pub name: String,
}

/// The outcome of a crime.
#[derive(Debug, Deserialize)]
pub struct OutcomeStatus {
    /// Category of the outcome.
    pub category: String,
    /// Date of the outcome (format: `YYYY-MM`).
    pub date: String,
}
