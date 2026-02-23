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

| Method | Description |
|--------|-------------|
| `forces()` | List all police forces |
| `force(id)` | Get details for a specific force |
| `senior_officers(force_id)` | List senior officers for a force |
| `crime_categories(date)` | List crime categories, optionally filtered by date |
| `crime_last_updated()` | Get the date crime data was last updated |
| `street_level_crimes(category, area, date)` | Street-level crimes by point, polygon, or location ID |
| `crimes_at_location(location_id, date)` | Crimes at a specific location |
| `crimes_no_location(category, force, date)` | Crimes that could not be mapped to a location |
| `street_level_outcomes(area, date)` | Street-level outcomes by point, polygon, or location ID |
| `outcomes_for_crime(persistent_id)` | All outcomes for a specific crime |

## License

MIT
