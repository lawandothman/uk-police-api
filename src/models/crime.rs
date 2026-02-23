use serde::Deserialize;

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
