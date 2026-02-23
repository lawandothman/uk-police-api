use crate::error::Error;
use crate::models::{Area, Crime, CrimeCategory, CrimeLastUpdated, Force, ForceDetail, Outcome};

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
    fn area_query(area: &Area) -> String {
        match area {
            Area::Point(coord) => format!("lat={}&lng={}", coord.lat, coord.lng),
            Area::Custom(coords) => {
                let poly = coords
                    .iter()
                    .map(|c| format!("{},{}", c.lat, c.lng))
                    .collect::<Vec<_>>()
                    .join(":");
                format!("poly={poly}")
            }
            Area::LocationId(id) => format!("location_id={id}"),
        }
    }

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

    /// Returns street-level crimes within a given area.
    ///
    /// # Arguments
    ///
    /// * `category` - Crime category slug (e.g. "all-crime", "burglary"). See [`Client::crime_categories`].
    /// * `area` - Either a point (1 mile radius) or a custom polygon.
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn street_level_crimes(
        &self,
        category: &str,
        area: &Area,
        date: Option<&str>,
    ) -> Result<Vec<Crime>, Error> {
        let mut url = format!(
            "{}/crimes-street/{}?{}",
            self.base_url,
            category,
            Self::area_query(area)
        );
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let crimes = self.http.get(&url).send().await?.json().await?;
        Ok(crimes)
    }

    /// Returns street-level outcomes at a given location.
    ///
    /// # Arguments
    ///
    /// * `area` - A point (1 mile radius), custom polygon, or specific location ID.
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn street_level_outcomes(
        &self,
        area: &Area,
        date: Option<&str>,
    ) -> Result<Vec<Outcome>, Error> {
        let mut url = format!(
            "{}/outcomes-at-location?{}",
            self.base_url,
            Self::area_query(area)
        );
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let outcomes = self.http.get(&url).send().await?.json().await?;
        Ok(outcomes)
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
    use crate::models::Coordinate;
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

    fn mock_crime_json() -> serde_json::Value {
        serde_json::json!([{
            "category": "anti-social-behaviour",
            "persistent_id": "",
            "location_subtype": "",
            "id": 116208998,
            "location": {
                "latitude": "52.632805",
                "street": { "id": 1738842, "name": "On or near Campbell Street" },
                "longitude": "-1.124819"
            },
            "context": "",
            "month": "2024-01",
            "location_type": "Force",
            "outcome_status": {
                "category": "Investigation complete; no suspect identified",
                "date": "2024-01"
            }
        }])
    }

    #[tokio::test]
    async fn test_street_level_crimes_by_point() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/crimes-street/all-crime"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_crime_json()))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let area = Area::Point(Coordinate {
            lat: 52.629729,
            lng: -1.131592,
        });
        let crimes = client
            .street_level_crimes("all-crime", &area, Some("2024-01"))
            .await
            .unwrap();

        assert_eq!(crimes.len(), 1);
        assert_eq!(crimes[0].category, "anti-social-behaviour");
        assert_eq!(crimes[0].location.street.name, "On or near Campbell Street");
        assert_eq!(
            crimes[0].outcome_status.as_ref().unwrap().category,
            crate::models::OutcomeCategory::NoFurtherAction
        );
    }

    #[tokio::test]
    async fn test_street_level_crimes_by_area() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/crimes-street/all-crime"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_crime_json()))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let area = Area::Custom(vec![
            Coordinate {
                lat: 52.268,
                lng: 0.543,
            },
            Coordinate {
                lat: 52.794,
                lng: 0.238,
            },
            Coordinate {
                lat: 52.130,
                lng: 0.478,
            },
        ]);
        let crimes = client
            .street_level_crimes("all-crime", &area, None)
            .await
            .unwrap();

        assert_eq!(crimes.len(), 1);
        assert_eq!(crimes[0].id, 116208998);
    }

    #[tokio::test]
    async fn test_street_level_outcomes_by_location_id() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/outcomes-at-location"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "category": {
                        "code": "local-resolution",
                        "name": "Local resolution"
                    },
                    "date": "2024-01",
                    "person_id": null,
                    "crime": {
                        "category": "public-order",
                        "persistent_id": "dd6e56f90d1bdd7bc7482af17852369f263203d9a688fac42ec53bf48485d8f1",
                        "location_subtype": "ROAD",
                        "location_type": "Force",
                        "location": {
                            "latitude": "52.637146",
                            "street": { "id": 1737432, "name": "On or near Vaughan Street" },
                            "longitude": "-1.149381"
                        },
                        "context": "",
                        "month": "2024-01",
                        "id": 116202605
                    }
                }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let outcomes = client
            .street_level_outcomes(&Area::LocationId(1737432), Some("2024-01"))
            .await
            .unwrap();

        assert_eq!(outcomes.len(), 1);
        assert_eq!(
            outcomes[0].category.code,
            crate::models::OutcomeCategory::LocalResolution
        );
        assert_eq!(outcomes[0].crime.category, "public-order");
        assert!(outcomes[0].person_id.is_none());
    }

    #[tokio::test]
    async fn test_street_level_outcomes_by_point() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/outcomes-at-location"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let area = Area::Point(Coordinate {
            lat: 52.629729,
            lng: -1.131592,
        });
        let outcomes = client.street_level_outcomes(&area, None).await.unwrap();

        assert!(outcomes.is_empty());
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
