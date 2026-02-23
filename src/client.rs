use crate::error::Error;
use crate::models::{CrimeCategory, CrimeLastUpdated, Force, ForceDetail};

const BASE_URL: &str = "https://data.police.uk/api";

/// An async client for the UK Police API.
///
/// # Example
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), uk_police_api::Error> {
/// let client = uk_police_api::Client::new();
/// let forces = client.forces().await?;
/// # Ok(())
/// # }
/// ```
pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Returns a list of all police forces.
    pub async fn forces(&self) -> Result<Vec<Force>, Error> {
        let url = format!("{}/forces", self.base_url);
        let forces = self.http.get(&url).send().await?.json().await?;
        Ok(forces)
    }

    /// Returns details for a specific police force.
    pub async fn force(&self, id: &str) -> Result<ForceDetail, Error> {
        let url = format!("{}/forces/{}", self.base_url, id);
        let force = self.http.get(&url).send().await?.json().await?;
        Ok(force)
    }

    /// Returns a list of crime categories. Optionally filtered by date (format: `YYYY-MM`).
    pub async fn crime_categories(&self, date: Option<&str>) -> Result<Vec<CrimeCategory>, Error> {
        let mut url = format!("{}/crime-categories", self.base_url);
        if let Some(date) = date {
            url.push_str(&format!("?date={date}"));
        }
        let categories = self.http.get(&url).send().await?.json().await?;
        Ok(categories)
    }

    /// Returns the date when crime data was last updated.
    pub async fn crime_last_updated(&self) -> Result<CrimeLastUpdated, Error> {
        let url = format!("{}/crime-last-updated", self.base_url);
        let updated = self.http.get(&url).send().await?.json().await?;
        Ok(updated)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_client(uri: &str) -> Client {
        Client {
            http: reqwest::Client::new(),
            base_url: uri.to_string(),
        }
    }

    #[tokio::test]
    async fn test_forces() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/forces"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "id": "met", "name": "Metropolitan Police" },
                { "id": "kent", "name": "Kent Police" }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let forces = client.forces().await.unwrap();

        assert_eq!(forces.len(), 2);
        assert_eq!(forces[0].id, "met");
        assert_eq!(forces[1].name, "Kent Police");
    }

    #[tokio::test]
    async fn test_force() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/forces/metropolitan"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "metropolitan",
                "name": "Metropolitan Police Service",
                "description": "The Met",
                "url": "https://www.met.police.uk/",
                "telephone": "101",
                "engagement_methods": [
                    {
                        "type": "twitter",
                        "title": "twitter",
                        "description": null,
                        "url": "https://x.com/Metpoliceuk"
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let force = client.force("metropolitan").await.unwrap();

        assert_eq!(force.id, "metropolitan");
        assert_eq!(force.telephone, Some("101".to_string()));
        assert_eq!(force.engagement_methods.len(), 1);
        assert_eq!(force.engagement_methods[0].kind, "twitter");
    }

    #[tokio::test]
    async fn test_crime_categories() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/crime-categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "url": "burglary", "name": "Burglary" },
                { "url": "drugs", "name": "Drugs" }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let categories = client.crime_categories(None).await.unwrap();

        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].url, "burglary");
    }

    #[tokio::test]
    async fn test_crime_last_updated() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/crime-last-updated"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({ "date": "2025-12-01" })),
            )
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let updated = client.crime_last_updated().await.unwrap();

        assert_eq!(updated.date, "2025-12-01");
    }
}
