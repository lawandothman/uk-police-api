use serde::Deserialize;

use super::crime::Location;

/// Type of stop and search.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum StopAndSearchType {
    #[serde(rename = "Person search")]
    Person,
    #[serde(rename = "Vehicle search")]
    Vehicle,
    #[serde(rename = "Person and Vehicle search")]
    PersonAndVehicle,
}

/// A stop and search record.
#[derive(Debug, Deserialize)]
pub struct StopAndSearch {
    /// Type of search performed.
    #[serde(rename = "type")]
    pub kind: Option<StopAndSearchType>,
    /// Whether a person was searched.
    pub involved_person: Option<bool>,
    /// Date and time of the stop (ISO 8601 format).
    pub datetime: Option<String>,
    /// Whether this was part of a policing operation.
    pub operation: Option<bool>,
    /// Name of the policing operation, if applicable.
    pub operation_name: Option<String>,
    /// Approximate location of the stop.
    pub location: Option<Location>,
    /// Gender of the person stopped.
    pub gender: Option<String>,
    /// Age range of the person stopped.
    pub age_range: Option<String>,
    /// Self-defined ethnicity of the person stopped.
    pub self_defined_ethnicity: Option<String>,
    /// Officer-defined ethnicity of the person stopped.
    pub officer_defined_ethnicity: Option<String>,
    /// Legislation under which the stop was conducted.
    pub legislation: Option<String>,
    /// Object of the search (e.g. "Controlled drugs").
    pub object_of_search: Option<String>,
    /// Outcome of the stop. `None` if nothing was found.
    /// The API may return `false` instead of `null` when nothing was found.
    #[serde(default, deserialize_with = "deserialize_outcome")]
    pub outcome: Option<String>,
    /// Whether the outcome was linked to the object of search.
    pub outcome_linked_to_object_of_search: Option<bool>,
    /// Whether removal of more than outer clothing was required.
    pub removal_of_more_than_outer_clothing: Option<bool>,
    /// Outcome object with id and name (returned by some endpoints).
    #[serde(default)]
    pub outcome_object: Option<OutcomeObject>,
}

/// Outcome identifier returned by the stops-by-force endpoint.
#[derive(Debug, Deserialize)]
pub struct OutcomeObject {
    /// Outcome identifier.
    pub id: Option<String>,
    /// Outcome name.
    pub name: Option<String>,
}

/// Deserializes the `outcome` field which can be a string, `false`, or `null`.
fn deserialize_outcome<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrBool {
        String(String),
        #[allow(dead_code)]
        Bool(bool),
    }

    match Option::<StringOrBool>::deserialize(deserializer)? {
        Some(StringOrBool::String(s)) => Ok(Some(s)),
        _ => Ok(None),
    }
}
