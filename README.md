# 👮‍♂️ uk-police-api

An async Rust client for the [UK Police API](https://data.police.uk/docs/).

Provides access to police force information, crime categories, and crime data availability across England, Wales, and Northern Ireland.

## Installation

```toml
[dependencies]
uk-police-api = "0.1"
```

## Usage

```rust
use uk_police_api::Client;

#[tokio::main]
async fn main() -> Result<(), uk_police_api::Error> {
    let client = Client::new();

    // List all police forces
    let forces = client.forces().await?;
    for force in &forces {
        println!("{}: {}", force.id, force.name);
    }

    // Get details for a specific force
    let met = client.force("metropolitan").await?;
    println!("{} - {}", met.name, met.telephone.unwrap_or_default());

    // List crime categories
    let categories = client.crime_categories(Some("2024-01")).await?;
    for category in &categories {
        println!("{}: {}", category.url, category.name);
    }

    // Check when crime data was last updated
    let updated = client.crime_last_updated().await?;
    println!("Last updated: {}", updated.date);

    Ok(())
}
```

## Supported endpoints

| Method | Description |
|--------|-------------|
| `forces()` | List all police forces |
| `force(id)` | Get details for a specific force |
| `crime_categories(date)` | List crime categories, optionally filtered by date |
| `crime_last_updated()` | Get the date crime data was last updated |

## License

MIT
