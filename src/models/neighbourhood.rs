use serde::{Deserialize, Serialize};

use super::force::ContactDetails;

/// A neighbourhood summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Neighbourhood {
    /// Force-specific neighbourhood identifier.
    /// Note: this identifier is not unique across forces.
    pub id: String,
    /// Neighbourhood name.
    pub name: String,
}

/// Detailed information about a specific neighbourhood.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeighbourhoodDetail {
    /// Force-specific neighbourhood identifier.
    pub id: String,
    /// Neighbourhood name.
    pub name: String,
    /// Description of the neighbourhood.
    pub description: Option<String>,
    /// Estimated population of the neighbourhood.
    pub population: Option<String>,
    /// URL for the neighbourhood on the force's website.
    pub url_force: Option<String>,
    /// Contact details for the neighbourhood policing team.
    pub contact_details: ContactDetails,
    /// Approximate centre point of the neighbourhood.
    pub centre: LatLng,
    /// Related links.
    pub links: Vec<Link>,
    /// Locations associated with the neighbourhood (e.g. police stations).
    pub locations: Vec<NeighbourhoodLocation>,
}

/// A latitude/longitude pair as strings (as returned by the API).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatLng {
    /// Latitude.
    pub latitude: String,
    /// Longitude.
    pub longitude: String,
}

/// A link associated with a neighbourhood.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    /// Link URL.
    pub url: Option<String>,
    /// Link title.
    pub title: Option<String>,
    /// Link description.
    pub description: Option<String>,
}

/// A location associated with a neighbourhood (e.g. a police station).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeighbourhoodLocation {
    /// Location name.
    pub name: Option<String>,
    /// Latitude.
    pub latitude: Option<String>,
    /// Longitude.
    pub longitude: Option<String>,
    /// Postcode.
    pub postcode: Option<String>,
    /// Street address.
    pub address: Option<String>,
    /// Telephone number.
    pub telephone: Option<String>,
    /// Location type (e.g. "station").
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Location description.
    pub description: Option<String>,
}

/// A neighbourhood event (e.g. community meeting, surgery).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeighbourhoodEvent {
    /// Event title.
    pub title: Option<String>,
    /// Event description.
    pub description: Option<String>,
    /// Event address.
    pub address: Option<String>,
    /// Event type.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Start date in ISO format.
    pub start_date: Option<String>,
    /// End date in ISO format.
    pub end_date: Option<String>,
    /// Contact details for the event.
    pub contact_details: Option<ContactDetails>,
}

/// A neighbourhood policing priority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeighbourhoodPriority {
    /// The issue raised.
    pub issue: Option<String>,
    /// Date the priority was agreed upon (ISO format).
    #[serde(rename = "issue-date")]
    pub issue_date: Option<String>,
    /// Action taken to address the priority.
    pub action: Option<String>,
    /// Date action was last taken (ISO format).
    #[serde(rename = "action-date")]
    pub action_date: Option<String>,
}

/// Result of locating a neighbourhood by coordinates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocateNeighbourhoodResult {
    /// Force identifier.
    pub force: String,
    /// Neighbourhood identifier.
    pub neighbourhood: String,
}
