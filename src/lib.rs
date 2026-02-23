//! # uk-police-api
//!
//! An async Rust client for the [UK Police API](https://data.police.uk/docs/).
//!
//! Provides access to police force information, crime categories, and crime data
//! availability across England, Wales, and Northern Ireland.
//!
//! # Example
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), uk_police_api::Error> {
//! let client = uk_police_api::Client::new();
//!
//! let forces = client.forces().await?;
//! let met = client.force("metropolitan").await?;
//! let categories = client.crime_categories(None).await?;
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
pub mod models;

pub use client::Client;
pub use error::Error;
pub use models::{
    Area, Coordinate, Crime, CrimeCategory, CrimeLastUpdated, EngagementMethod, Force, ForceDetail,
    Location, OutcomeStatus, Street,
};
