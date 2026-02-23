use serde::{Deserialize, Serialize};

/// A summary of a police force.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Force {
    /// Unique force identifier.
    pub id: String,
    /// Force name.
    pub name: String,
}

/// Detailed information about a specific police force.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForceDetail {
    /// Unique force identifier.
    pub id: String,
    /// Force name.
    pub name: String,
    /// Description of the force.
    pub description: Option<String>,
    /// Force website URL.
    pub url: Option<String>,
    /// Force telephone number.
    pub telephone: Option<String>,
    /// Ways to keep informed about the force.
    pub engagement_methods: Vec<EngagementMethod>,
}

/// A senior officer of a police force.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeniorOfficer {
    /// Name of the officer.
    pub name: String,
    /// Force rank (e.g. "Chief Constable").
    pub rank: String,
    /// Officer biography, if available.
    pub bio: Option<String>,
    /// Contact details for the officer.
    pub contact_details: ContactDetails,
}

/// Contact details for a senior officer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactDetails {
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub mobile: Option<String>,
    pub fax: Option<String>,
    pub web: Option<String>,
    pub address: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
    pub myspace: Option<String>,
    pub bebo: Option<String>,
    pub flickr: Option<String>,
    #[serde(rename = "google-plus")]
    pub google_plus: Option<String>,
    pub forum: Option<String>,
    #[serde(rename = "e-messaging")]
    pub e_messaging: Option<String>,
    pub blog: Option<String>,
    pub rss: Option<String>,
}

/// A way to engage with a police force (e.g. Twitter, Facebook, website).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngagementMethod {
    /// The type of engagement method (e.g. "facebook", "twitter", "rss").
    #[serde(rename = "type")]
    pub kind: String,
    /// Method title.
    pub title: Option<String>,
    /// Method description.
    pub description: Option<String>,
    /// Method website URL.
    pub url: Option<String>,
}
