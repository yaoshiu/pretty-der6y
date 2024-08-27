use std::error::Error;

use chrono::Local;
use lib::Account;

const GEOJSON_STR: &str = include_str!("../assets/map.geojson");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    todo!()
}
