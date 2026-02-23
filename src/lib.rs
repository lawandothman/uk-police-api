//! # uk-police-api
//!
//! An async Rust client for the [UK Police API](https://data.police.uk/docs/).
//!
//! # Example
//!
//! ```no_run
//! use uk_police_api::{Client, Area, Coordinate};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), uk_police_api::Error> {
//! let client = Client::new();
//!
//! // What crimes happened near Big Ben last month?
//! let area = Area::Point(Coordinate { lat: 51.5007, lng: -0.1246 });
//! let crimes = client.street_level_crimes("all-crime", &area, None).await?;
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
pub mod models;

pub use client::Client;
pub use error::Error;
pub use models::{
    Area, ContactDetails, Coordinate, Crime, CrimeCategory, CrimeLastUpdated, CrimeOutcome,
    CrimeOutcomes, EngagementMethod, Force, ForceDetail, LatLng, Link, LocateNeighbourhoodResult,
    Location, Neighbourhood, NeighbourhoodDetail, NeighbourhoodEvent, NeighbourhoodLocation,
    NeighbourhoodPriority, Outcome, OutcomeCategory, OutcomeDetail, OutcomeObject, OutcomeStatus,
    SeniorOfficer, StopAndSearch, StopAndSearchType, Street,
};
