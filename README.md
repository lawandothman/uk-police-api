# 👮‍♂️ uk-police-api

An async Rust client for the [UK Police API](https://data.police.uk/docs/).

## Installation

```toml
[dependencies]
uk-police-api = "0.1"
```

## Usage

```rust
use uk_police_api::{Client, Area, Coordinate};

#[tokio::main]
async fn main() -> Result<(), uk_police_api::Error> {
    let client = Client::new();

    // List all police forces
    let forces = client.forces().await?;

    // Get details for a specific force
    let met = client.force("metropolitan").await?;

    // Street-level crimes near a point
    let area = Area::Point(Coordinate { lat: 52.629729, lng: -1.131592 });
    let crimes = client.street_level_crimes("all-crime", &area, Some("2024-01")).await?;

    // Outcomes at a location
    let outcomes = client.street_level_outcomes(&area, Some("2024-01")).await?;

    Ok(())
}
```

## Supported endpoints

### Forces

| Method | Description |
|--------|-------------|
| `forces()` | List all police forces |
| `force(id)` | Details for a specific force |
| `senior_officers(force_id)` | Senior officers for a force |

### Crimes

| Method | Description |
|--------|-------------|
| `street_level_crimes(category, area, date)` | Street-level crimes by point, polygon, or location ID |
| `crimes_at_location(location_id, date)` | Crimes at a specific location |
| `crimes_no_location(category, force, date)` | Crimes that could not be mapped to a location |
| `crime_categories(date)` | List crime categories |
| `crime_last_updated()` | Date crime data was last updated |
| `street_level_outcomes(area, date)` | Street-level outcomes by point, polygon, or location ID |
| `outcomes_for_crime(persistent_id)` | All outcomes for a specific crime |

### Neighbourhoods

| Method | Description |
|--------|-------------|
| `neighbourhoods(force_id)` | List neighbourhoods for a force |
| `neighbourhood(force_id, neighbourhood_id)` | Details for a specific neighbourhood |
| `neighbourhood_boundary(force_id, neighbourhood_id)` | Boundary coordinates for a neighbourhood |
| `neighbourhood_team(force_id, neighbourhood_id)` | Team members for a neighbourhood |
| `neighbourhood_events(force_id, neighbourhood_id)` | Events for a neighbourhood |
| `neighbourhood_priorities(force_id, neighbourhood_id)` | Policing priorities for a neighbourhood |
| `locate_neighbourhood(lat, lng)` | Find the neighbourhood responsible for a point |

### Stop and search

| Method | Description |
|--------|-------------|
| `stops_street(area, date)` | Stop and searches by area (point or polygon) |
| `stops_at_location(location_id, date)` | Stop and searches at a specific location |
| `stops_no_location(force, date)` | Stop and searches that could not be mapped to a location |
| `stops_force(force, date)` | Stop and searches reported by a force |

## License

MIT
