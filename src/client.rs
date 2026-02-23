use crate::error::Error;
use crate::models::{
    Area, Crime, CrimeCategory, CrimeLastUpdated, CrimeOutcomes, Force, ForceDetail, LatLng,
    LocateNeighbourhoodResult, Neighbourhood, NeighbourhoodDetail, NeighbourhoodEvent,
    NeighbourhoodPriority, Outcome, SeniorOfficer, StopAndSearch,
};

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
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    async fn handle_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, Error> {
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status, body });
        }
        Ok(response.json().await?)
    }

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

    /// Creates a client with a pre-configured [`reqwest::Client`].
    ///
    /// Use this to customize timeouts, headers, proxies, or any other HTTP
    /// behaviour.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let http = reqwest::Client::builder()
    ///     .timeout(std::time::Duration::from_secs(10))
    ///     .build()
    ///     .unwrap();
    /// let client = uk_police_api::Client::from_http_client(http);
    /// ```
    pub fn from_http_client(http: reqwest::Client) -> Self {
        Self {
            http,
            base_url: BASE_URL.to_string(),
        }
    }

    /// Returns a list of all police forces.
    pub async fn forces(&self) -> Result<Vec<Force>, Error> {
        let url = format!("{}/forces", self.base_url);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns details for a specific police force.
    pub async fn force(&self, id: &str) -> Result<ForceDetail, Error> {
        let url = format!("{}/forces/{}", self.base_url, id);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns a list of crime categories. Optionally filtered by date (format: `YYYY-MM`).
    pub async fn crime_categories(&self, date: Option<&str>) -> Result<Vec<CrimeCategory>, Error> {
        let mut url = format!("{}/crime-categories", self.base_url);
        if let Some(date) = date {
            url.push_str(&format!("?date={date}"));
        }
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
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
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
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
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns the date when crime data was last updated.
    pub async fn crime_last_updated(&self) -> Result<CrimeLastUpdated, Error> {
        let url = format!("{}/crime-last-updated", self.base_url);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns a list of senior officers for a given force.
    pub async fn senior_officers(&self, force_id: &str) -> Result<Vec<SeniorOfficer>, Error> {
        let url = format!("{}/forces/{}/people", self.base_url, force_id);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns crimes at a specific location.
    ///
    /// # Arguments
    ///
    /// * `location_id` - A location ID (from a street's `id` field).
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn crimes_at_location(
        &self,
        location_id: u64,
        date: Option<&str>,
    ) -> Result<Vec<Crime>, Error> {
        let mut url = format!(
            "{}/crimes-at-location?location_id={}",
            self.base_url, location_id
        );
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns crimes that could not be mapped to a location.
    ///
    /// # Arguments
    ///
    /// * `category` - Crime category slug (e.g. "all-crime"). See [`Client::crime_categories`].
    /// * `force` - Force identifier (e.g. "metropolitan").
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn crimes_no_location(
        &self,
        category: &str,
        force: &str,
        date: Option<&str>,
    ) -> Result<Vec<Crime>, Error> {
        let mut url = format!(
            "{}/crimes-no-location?category={}&force={}",
            self.base_url, category, force
        );
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns all outcomes for a specific crime.
    ///
    /// # Arguments
    ///
    /// * `persistent_id` - The 64-character crime identifier.
    pub async fn outcomes_for_crime(&self, persistent_id: &str) -> Result<CrimeOutcomes, Error> {
        let url = format!("{}/outcomes-for-crime/{}", self.base_url, persistent_id);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns a list of neighbourhoods for a force.
    pub async fn neighbourhoods(&self, force_id: &str) -> Result<Vec<Neighbourhood>, Error> {
        let url = format!("{}/{}/neighbourhoods", self.base_url, force_id);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns details for a specific neighbourhood.
    pub async fn neighbourhood(
        &self,
        force_id: &str,
        neighbourhood_id: &str,
    ) -> Result<NeighbourhoodDetail, Error> {
        let url = format!("{}/{}/{}", self.base_url, force_id, neighbourhood_id);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns the boundary of a neighbourhood as a list of lat/lng pairs.
    pub async fn neighbourhood_boundary(
        &self,
        force_id: &str,
        neighbourhood_id: &str,
    ) -> Result<Vec<LatLng>, Error> {
        let url = format!(
            "{}/{}/{}/boundary",
            self.base_url, force_id, neighbourhood_id
        );
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns the policing team for a neighbourhood.
    pub async fn neighbourhood_team(
        &self,
        force_id: &str,
        neighbourhood_id: &str,
    ) -> Result<Vec<SeniorOfficer>, Error> {
        let url = format!("{}/{}/{}/people", self.base_url, force_id, neighbourhood_id);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns events for a neighbourhood.
    pub async fn neighbourhood_events(
        &self,
        force_id: &str,
        neighbourhood_id: &str,
    ) -> Result<Vec<NeighbourhoodEvent>, Error> {
        let url = format!("{}/{}/{}/events", self.base_url, force_id, neighbourhood_id);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns policing priorities for a neighbourhood.
    pub async fn neighbourhood_priorities(
        &self,
        force_id: &str,
        neighbourhood_id: &str,
    ) -> Result<Vec<NeighbourhoodPriority>, Error> {
        let url = format!(
            "{}/{}/{}/priorities",
            self.base_url, force_id, neighbourhood_id
        );
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Locates the neighbourhood policing team responsible for a given point.
    pub async fn locate_neighbourhood(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<LocateNeighbourhoodResult, Error> {
        let url = format!("{}/locate-neighbourhood?q={},{}", self.base_url, lat, lng);
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns stop and searches within a given area.
    ///
    /// # Arguments
    ///
    /// * `area` - A point (1 mile radius) or custom polygon.
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn stops_street(
        &self,
        area: &Area,
        date: Option<&str>,
    ) -> Result<Vec<StopAndSearch>, Error> {
        let mut url = format!("{}/stops-street?{}", self.base_url, Self::area_query(area));
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns stop and searches at a specific location.
    ///
    /// # Arguments
    ///
    /// * `location_id` - A location ID (from a street's `id` field).
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn stops_at_location(
        &self,
        location_id: u64,
        date: Option<&str>,
    ) -> Result<Vec<StopAndSearch>, Error> {
        let mut url = format!(
            "{}/stops-at-location?location_id={}",
            self.base_url, location_id
        );
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns stop and searches that could not be mapped to a location.
    ///
    /// # Arguments
    ///
    /// * `force` - Force identifier (e.g. "metropolitan").
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn stops_no_location(
        &self,
        force: &str,
        date: Option<&str>,
    ) -> Result<Vec<StopAndSearch>, Error> {
        let mut url = format!("{}/stops-no-location?force={}", self.base_url, force);
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
    }

    /// Returns stop and searches reported by a force.
    ///
    /// # Arguments
    ///
    /// * `force` - Force identifier (e.g. "metropolitan").
    /// * `date` - Optional month filter (format: `YYYY-MM`). Defaults to the latest available.
    pub async fn stops_force(
        &self,
        force: &str,
        date: Option<&str>,
    ) -> Result<Vec<StopAndSearch>, Error> {
        let mut url = format!("{}/stops-force?force={}", self.base_url, force);
        if let Some(date) = date {
            url.push_str(&format!("&date={date}"));
        }
        let response = self.http.get(&url).send().await?;
        Self::handle_response(response).await
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
        assert_eq!(
            crimes[0].location.as_ref().unwrap().street.name,
            "On or near Campbell Street"
        );
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

    #[tokio::test]
    async fn test_senior_officers() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/forces/metropolitan/people"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "name": "Mark Rowley",
                    "rank": "Commissioner",
                    "bio": null,
                    "contact_details": {
                        "twitter": "https://x.com/metpoliceuk"
                    }
                }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let officers = client.senior_officers("metropolitan").await.unwrap();

        assert_eq!(officers.len(), 1);
        assert_eq!(officers[0].name, "Mark Rowley");
        assert_eq!(officers[0].rank, "Commissioner");
        assert!(officers[0].bio.is_none());
        assert_eq!(
            officers[0].contact_details.twitter,
            Some("https://x.com/metpoliceuk".to_string())
        );
    }

    #[tokio::test]
    async fn test_crimes_at_location() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/crimes-at-location"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_crime_json()))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let crimes = client
            .crimes_at_location(1738842, Some("2024-01"))
            .await
            .unwrap();

        assert_eq!(crimes.len(), 1);
        assert_eq!(crimes[0].category, "anti-social-behaviour");
    }

    #[tokio::test]
    async fn test_crimes_no_location() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/crimes-no-location"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "category": "burglary",
                    "persistent_id": "abc123",
                    "location_subtype": "",
                    "id": 999,
                    "location": null,
                    "context": "",
                    "month": "2024-01",
                    "location_type": null,
                    "outcome_status": null
                }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let crimes = client
            .crimes_no_location("burglary", "metropolitan", Some("2024-01"))
            .await
            .unwrap();

        assert_eq!(crimes.len(), 1);
        assert_eq!(crimes[0].category, "burglary");
        assert!(crimes[0].location.is_none());
        assert!(crimes[0].location_type.is_none());
    }

    #[tokio::test]
    async fn test_outcomes_for_crime() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/outcomes-for-crime/dd6e56f90d1bdd7bc7482af17852369f263203d9a688fac42ec53bf48485d8f1",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "crime": {
                    "category": "violent-crime",
                    "persistent_id": "dd6e56f90d1bdd7bc7482af17852369f263203d9a688fac42ec53bf48485d8f1",
                    "location_subtype": "",
                    "id": 116202605,
                    "location": {
                        "latitude": "52.637146",
                        "street": { "id": 1737432, "name": "On or near Vaughan Street" },
                        "longitude": "-1.149381"
                    },
                    "context": "",
                    "month": "2024-01",
                    "location_type": "Force",
                    "outcome_status": null
                },
                "outcomes": [
                    {
                        "category": {
                            "code": "no-further-action",
                            "name": "Investigation complete; no suspect identified"
                        },
                        "date": "2024-01",
                        "person_id": null
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let result = client
            .outcomes_for_crime("dd6e56f90d1bdd7bc7482af17852369f263203d9a688fac42ec53bf48485d8f1")
            .await
            .unwrap();

        assert_eq!(result.crime.category, "violent-crime");
        assert_eq!(result.outcomes.len(), 1);
        assert_eq!(
            result.outcomes[0].category.code,
            crate::models::OutcomeCategory::NoFurtherAction
        );
        assert!(result.outcomes[0].person_id.is_none());
    }

    #[tokio::test]
    async fn test_neighbourhoods() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/leicestershire/neighbourhoods"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "id": "NC04", "name": "City Centre" },
                { "id": "NC66", "name": "Cultural Quarter" }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let neighbourhoods = client.neighbourhoods("leicestershire").await.unwrap();

        assert_eq!(neighbourhoods.len(), 2);
        assert_eq!(neighbourhoods[0].id, "NC04");
        assert_eq!(neighbourhoods[1].name, "Cultural Quarter");
    }

    #[tokio::test]
    async fn test_neighbourhood() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/leicestershire/NC04"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "NC04",
                "name": "City Centre",
                "description": "The city centre neighbourhood",
                "population": "7985",
                "url_force": "https://www.leics.police.uk/local-policing/city-centre",
                "contact_details": {
                    "email": "citycentre@example.com"
                },
                "centre": {
                    "latitude": "52.6389",
                    "longitude": "-1.1350"
                },
                "links": [
                    { "url": "https://example.com", "title": "Example", "description": null }
                ],
                "locations": [
                    {
                        "name": "Mansfield House",
                        "latitude": "52.6352",
                        "longitude": "-1.1332",
                        "postcode": "LE1 3GG",
                        "address": "74 Belgrave Gate",
                        "telephone": "101",
                        "type": "station",
                        "description": null
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let detail = client
            .neighbourhood("leicestershire", "NC04")
            .await
            .unwrap();

        assert_eq!(detail.id, "NC04");
        assert_eq!(detail.population, Some("7985".to_string()));
        assert_eq!(detail.centre.latitude, "52.6389");
        assert_eq!(detail.links.len(), 1);
        assert_eq!(detail.locations.len(), 1);
        assert_eq!(detail.locations[0].kind, Some("station".to_string()));
    }

    #[tokio::test]
    async fn test_neighbourhood_boundary() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/leicestershire/NC04/boundary"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "latitude": "52.6394", "longitude": "-1.1459" },
                { "latitude": "52.6389", "longitude": "-1.1457" },
                { "latitude": "52.6381", "longitude": "-1.1447" }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let boundary = client
            .neighbourhood_boundary("leicestershire", "NC04")
            .await
            .unwrap();

        assert_eq!(boundary.len(), 3);
        assert_eq!(boundary[0].latitude, "52.6394");
        assert_eq!(boundary[2].longitude, "-1.1447");
    }

    #[tokio::test]
    async fn test_neighbourhood_team() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/leicestershire/NC04/people"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "name": "Andy Cooper",
                    "rank": "Sgt",
                    "bio": "Andy has been with the force since 2003.",
                    "contact_details": {}
                }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let team = client
            .neighbourhood_team("leicestershire", "NC04")
            .await
            .unwrap();

        assert_eq!(team.len(), 1);
        assert_eq!(team[0].name, "Andy Cooper");
        assert_eq!(team[0].rank, "Sgt");
        assert_eq!(
            team[0].bio,
            Some("Andy has been with the force since 2003.".to_string())
        );
    }

    #[tokio::test]
    async fn test_neighbourhood_events() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/leicestershire/NC04/events"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "title": "Bike Registration",
                    "description": "Free bike registration event",
                    "address": "Mansfield House",
                    "type": "meeting",
                    "start_date": "2024-09-17T17:00:00",
                    "end_date": "2024-09-17T19:00:00",
                    "contact_details": {}
                }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let events = client
            .neighbourhood_events("leicestershire", "NC04")
            .await
            .unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title, Some("Bike Registration".to_string()));
        assert_eq!(events[0].kind, Some("meeting".to_string()));
    }

    #[tokio::test]
    async fn test_neighbourhood_priorities() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/leicestershire/NC04/priorities"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "action": "Increased patrols in the area.",
                    "issue-date": "2024-07-01T00:00:00",
                    "action-date": "2024-09-01T00:00:00",
                    "issue": "Anti-social behaviour on Granby Street"
                }
            ])))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let priorities = client
            .neighbourhood_priorities("leicestershire", "NC04")
            .await
            .unwrap();

        assert_eq!(priorities.len(), 1);
        assert_eq!(
            priorities[0].issue,
            Some("Anti-social behaviour on Granby Street".to_string())
        );
        assert_eq!(
            priorities[0].action,
            Some("Increased patrols in the area.".to_string())
        );
    }

    #[tokio::test]
    async fn test_locate_neighbourhood() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/locate-neighbourhood"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "force": "metropolitan",
                "neighbourhood": "E05013806N"
            })))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let result = client
            .locate_neighbourhood(51.500617, -0.124629)
            .await
            .unwrap();

        assert_eq!(result.force, "metropolitan");
        assert_eq!(result.neighbourhood, "E05013806N");
    }

    fn mock_stop_json() -> serde_json::Value {
        serde_json::json!([{
            "type": "Person search",
            "involved_person": true,
            "datetime": "2024-01-15T12:30:00+00:00",
            "operation": false,
            "operation_name": null,
            "location": {
                "latitude": "52.634407",
                "street": { "id": 1737432, "name": "On or near Vaughan Street" },
                "longitude": "-1.149381"
            },
            "gender": "Male",
            "age_range": "18-24",
            "self_defined_ethnicity": "White - English/Welsh/Scottish/Northern Irish/British",
            "officer_defined_ethnicity": "White",
            "legislation": "Misuse of Drugs Act 1971 (section 23)",
            "object_of_search": "Controlled drugs",
            "outcome": "A no further action disposal",
            "outcome_linked_to_object_of_search": null,
            "removal_of_more_than_outer_clothing": false
        }])
    }

    #[tokio::test]
    async fn test_stops_street() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/stops-street"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_stop_json()))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let area = Area::Point(Coordinate {
            lat: 52.629729,
            lng: -1.131592,
        });
        let stops = client.stops_street(&area, Some("2024-01")).await.unwrap();

        assert_eq!(stops.len(), 1);
        assert_eq!(
            stops[0].kind,
            Some(crate::models::StopAndSearchType::Person)
        );
        assert_eq!(stops[0].involved_person, Some(true));
        assert_eq!(stops[0].gender, Some("Male".to_string()));
        assert_eq!(
            stops[0].outcome,
            Some("A no further action disposal".to_string())
        );
    }

    #[tokio::test]
    async fn test_stops_at_location() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/stops-at-location"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_stop_json()))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let stops = client
            .stops_at_location(1737432, Some("2024-01"))
            .await
            .unwrap();

        assert_eq!(stops.len(), 1);
        assert_eq!(
            stops[0].object_of_search,
            Some("Controlled drugs".to_string())
        );
    }

    #[tokio::test]
    async fn test_stops_no_location() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/stops-no-location"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                    "type": "Vehicle search",
                    "involved_person": false,
                    "datetime": "2024-01-10T08:00:00+00:00",
                    "operation": null,
                    "operation_name": null,
                    "location": null,
                    "gender": null,
                    "age_range": null,
                    "self_defined_ethnicity": null,
                    "officer_defined_ethnicity": null,
                    "legislation": "Misuse of Drugs Act 1971 (section 23)",
                    "object_of_search": "Controlled drugs",
                    "outcome": false,
                    "outcome_linked_to_object_of_search": null,
                    "removal_of_more_than_outer_clothing": null
                }])),
            )
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let stops = client
            .stops_no_location("leicestershire", Some("2024-01"))
            .await
            .unwrap();

        assert_eq!(stops.len(), 1);
        assert_eq!(
            stops[0].kind,
            Some(crate::models::StopAndSearchType::Vehicle)
        );
        assert!(stops[0].location.is_none());
        assert!(stops[0].outcome.is_none());
    }

    #[tokio::test]
    async fn test_stops_force() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/stops-force"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                    "type": "Person and Vehicle search",
                    "involved_person": true,
                    "datetime": "2024-01-20T14:00:00+00:00",
                    "operation": true,
                    "operation_name": "Operation Blitz",
                    "location": {
                        "latitude": "52.634407",
                        "street": { "id": 1737432, "name": "On or near Vaughan Street" },
                        "longitude": "-1.149381"
                    },
                    "gender": "Female",
                    "age_range": "25-34",
                    "self_defined_ethnicity": null,
                    "officer_defined_ethnicity": "Black",
                    "legislation": "Police and Criminal Evidence Act 1984 (section 1)",
                    "object_of_search": "Stolen goods",
                    "outcome": "Arrest",
                    "outcome_object": {
                        "id": "bu-arrest",
                        "name": "Arrest"
                    },
                    "outcome_linked_to_object_of_search": true,
                    "removal_of_more_than_outer_clothing": false
                }])),
            )
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let stops = client
            .stops_force("leicestershire", Some("2024-01"))
            .await
            .unwrap();

        assert_eq!(stops.len(), 1);
        assert_eq!(
            stops[0].kind,
            Some(crate::models::StopAndSearchType::PersonAndVehicle)
        );
        assert_eq!(stops[0].operation, Some(true));
        assert_eq!(stops[0].operation_name, Some("Operation Blitz".to_string()));
        assert_eq!(stops[0].outcome, Some("Arrest".to_string()));
        assert_eq!(
            stops[0].outcome_object.as_ref().unwrap().name,
            Some("Arrest".to_string())
        );
    }

    #[tokio::test]
    async fn test_not_found() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/forces/nonexistent"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let err = client.force("nonexistent").await.unwrap_err();

        match err {
            Error::Api { status, body } => {
                assert_eq!(status, 404);
                assert_eq!(body, "Not Found");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_rate_limited() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/forces"))
            .respond_with(ResponseTemplate::new(429).set_body_string("Rate limit exceeded"))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let err = client.forces().await.unwrap_err();

        match err {
            Error::Api { status, body } => {
                assert_eq!(status, 429);
                assert_eq!(body, "Rate limit exceeded");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_bad_request() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/crime-categories"))
            .respond_with(ResponseTemplate::new(400).set_body_string("Bad Request"))
            .mount(&server)
            .await;

        let client = test_client(&server.uri());
        let err = client.crime_categories(None).await.unwrap_err();

        match err {
            Error::Api { status, body } => {
                assert_eq!(status, 400);
                assert_eq!(body, "Bad Request");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }
}
