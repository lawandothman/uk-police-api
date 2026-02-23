use uk_police_api::{Area, Client, Coordinate, Error};

fn client() -> Client {
    Client::new()
}

// --- Forces ---

#[tokio::test]
#[ignore]
async fn forces_returns_non_empty_list() {
    let forces = client().forces().await.unwrap();
    assert!(!forces.is_empty());
}

#[tokio::test]
#[ignore]
async fn force_returns_details() {
    let force = client().force("leicestershire").await.unwrap();
    assert_eq!(force.id, "leicestershire");
    assert!(!force.name.is_empty());
}

#[tokio::test]
#[ignore]
async fn senior_officers_returns_list() {
    // May be empty for some forces, but the call itself should succeed
    let _ = client().senior_officers("leicestershire").await.unwrap();
}

// --- Crime ---

#[tokio::test]
#[ignore]
async fn crime_categories_returns_non_empty_list() {
    let categories = client().crime_categories(None).await.unwrap();
    assert!(!categories.is_empty());
}

#[tokio::test]
#[ignore]
async fn crime_last_updated_returns_date() {
    let updated = client().crime_last_updated().await.unwrap();
    assert!(!updated.date.is_empty());
}

#[tokio::test]
#[ignore]
async fn street_level_crimes_near_known_point() {
    let area = Area::Point(Coordinate {
        lat: 52.6297,
        lng: -1.1316,
    });
    let crimes = client()
        .street_level_crimes("all-crime", &area, None)
        .await
        .unwrap();
    assert!(!crimes.is_empty());
}

#[tokio::test]
#[ignore]
async fn street_level_outcomes_near_known_point() {
    let area = Area::Point(Coordinate {
        lat: 52.6297,
        lng: -1.1316,
    });
    let outcomes = client().street_level_outcomes(&area, None).await.unwrap();
    assert!(!outcomes.is_empty());
}

#[tokio::test]
#[ignore]
async fn crimes_at_location_returns_results() {
    // First get a real location ID from street-level crimes
    let area = Area::Point(Coordinate {
        lat: 52.6297,
        lng: -1.1316,
    });
    let crimes = client()
        .street_level_crimes("all-crime", &area, None)
        .await
        .unwrap();
    let location_id = crimes
        .iter()
        .find_map(|c| c.location.as_ref().map(|l| l.street.id))
        .expect("expected at least one crime with a location");

    let crimes_at = client()
        .crimes_at_location(location_id, None)
        .await
        .unwrap();
    assert!(!crimes_at.is_empty());
}

#[tokio::test]
#[ignore]
async fn crimes_no_location_returns_list() {
    // May be empty, but the call should succeed
    let _ = client()
        .crimes_no_location("all-crime", "leicestershire", None)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn outcomes_for_crime_returns_result() {
    // Get a crime with a persistent_id first
    let area = Area::Point(Coordinate {
        lat: 52.6297,
        lng: -1.1316,
    });
    let crimes = client()
        .street_level_crimes("all-crime", &area, None)
        .await
        .unwrap();
    let persistent_id = crimes
        .iter()
        .find(|c| !c.persistent_id.is_empty())
        .map(|c| c.persistent_id.clone())
        .expect("expected at least one crime with a persistent_id");

    let outcomes = client().outcomes_for_crime(&persistent_id).await.unwrap();
    assert_eq!(outcomes.crime.persistent_id, persistent_id);
}

// --- Neighbourhoods ---

#[tokio::test]
#[ignore]
async fn neighbourhoods_returns_non_empty_list() {
    let neighbourhoods = client().neighbourhoods("leicestershire").await.unwrap();
    assert!(!neighbourhoods.is_empty());
}

#[tokio::test]
#[ignore]
async fn neighbourhood_returns_details() {
    let neighbourhoods = client().neighbourhoods("leicestershire").await.unwrap();
    let first = &neighbourhoods[0];

    let detail = client()
        .neighbourhood("leicestershire", &first.id)
        .await
        .unwrap();
    assert_eq!(detail.id, first.id);
}

#[tokio::test]
#[ignore]
async fn neighbourhood_boundary_returns_points() {
    let neighbourhoods = client().neighbourhoods("leicestershire").await.unwrap();
    let first = &neighbourhoods[0];

    let boundary = client()
        .neighbourhood_boundary("leicestershire", &first.id)
        .await
        .unwrap();
    assert!(!boundary.is_empty());
}

#[tokio::test]
#[ignore]
async fn neighbourhood_team_returns_list() {
    let neighbourhoods = client().neighbourhoods("leicestershire").await.unwrap();
    let first = &neighbourhoods[0];

    // May be empty, but the call should succeed
    let _ = client()
        .neighbourhood_team("leicestershire", &first.id)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn neighbourhood_events_returns_list() {
    let neighbourhoods = client().neighbourhoods("leicestershire").await.unwrap();
    let first = &neighbourhoods[0];

    // May be empty, but the call should succeed
    let _ = client()
        .neighbourhood_events("leicestershire", &first.id)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn neighbourhood_priorities_returns_list() {
    let neighbourhoods = client().neighbourhoods("leicestershire").await.unwrap();
    let first = &neighbourhoods[0];

    // May be empty, but the call should succeed
    let _ = client()
        .neighbourhood_priorities("leicestershire", &first.id)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn locate_neighbourhood_big_ben() {
    let result = client()
        .locate_neighbourhood(51.5007, -0.1246)
        .await
        .unwrap();
    assert_eq!(result.force, "metropolitan");
}

// --- Stop and Search ---

#[tokio::test]
#[ignore]
async fn stops_street_returns_results() {
    let area = Area::Point(Coordinate {
        lat: 52.6297,
        lng: -1.1316,
    });
    // May be empty depending on data availability
    let _ = client().stops_street(&area, None).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn stops_at_location_returns_results() {
    // Get a real location ID from stop and searches
    let area = Area::Point(Coordinate {
        lat: 52.6297,
        lng: -1.1316,
    });
    let stops = client().stops_street(&area, None).await.unwrap();
    if let Some(location_id) = stops
        .iter()
        .find_map(|s| s.location.as_ref().map(|l| l.street.id))
    {
        let _ = client().stops_at_location(location_id, None).await.unwrap();
    }
}

#[tokio::test]
#[ignore]
async fn stops_no_location_returns_list() {
    let _ = client()
        .stops_no_location("leicestershire", None)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn stops_force_returns_list() {
    let _ = client().stops_force("leicestershire", None).await.unwrap();
}

// --- Error cases ---

#[tokio::test]
#[ignore]
async fn nonexistent_force_returns_api_error() {
    let err = client().force("nonexistent-force-id").await.unwrap_err();
    match err {
        Error::Api { status, .. } => assert_eq!(status, 404),
        other => panic!("expected Error::Api with 404, got: {other}"),
    }
}
