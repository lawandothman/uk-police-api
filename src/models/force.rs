use serde::Deserialize;

/// A summary of a police force.
#[derive(Debug, Deserialize)]
pub struct Force {
    /// Unique force identifier.
    pub id: String,
    /// Force name.
    pub name: String,
}

/// Detailed information about a specific police force.
#[derive(Debug, Deserialize)]
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

/// A way to engage with a police force (e.g. Twitter, Facebook, website).
#[derive(Debug, Deserialize)]
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
