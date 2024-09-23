/*
    Pretty Der6y - A third-party running data upload client.
    Copyright (C) 2024  Fay Ash

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use geo::{prelude::*, Point};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{error::Error, f64::consts::PI};

// WGS-84 to GCJ-02 (Mars Coordinate System) conversion
// Only valid for coordinates within China.
fn wgs84_to_gcj02(lat: f64, lon: f64) -> (f64, f64) {
    const A: f64 = 6378245.;
    const EE: f64 = 0.006_693_421_622_965_943;

    fn transform_lat(x: f64, y: f64) -> f64 {
        let mut ret = -100.0 + 2.0 * x + 3.0 * y + 0.2 * y * y + 0.1 * x * y + 0.2 * x.abs().sqrt();
        ret += (20.0 * (6.0 * x * PI).sin() + 20.0 * (2.0 * x * PI).sin()) * 2.0 / 3.0;
        ret += (20.0 * (y * PI).sin() + 40.0 * (y / 3.0 * PI).sin()) * 2.0 / 3.0;
        ret += (160.0 * (y / 12.0 * PI).sin() + 320.0 * (y * PI / 30.0).sin()) * 2.0 / 3.0;
        ret
    }

    fn transform_lon(x: f64, y: f64) -> f64 {
        let mut ret = 300.0 + x + 2.0 * y + 0.1 * x * x + 0.1 * x * y + 0.1 * x.abs().sqrt();
        ret += (20.0 * (6.0 * x * PI).sin() + 20.0 * (2.0 * x * PI).sin()) * 2.0 / 3.0;
        ret += (20.0 * (x * PI).sin() + 40.0 * (x / 3.0 * PI).sin()) * 2.0 / 3.0;
        ret += (150.0 * (x / 12.0 * PI).sin() + 300.0 * (x / 30.0 * PI).sin()) * 2.0 / 3.0;
        ret
    }

    fn out_of_china(lat: f64, lon: f64) -> bool {
        if !(72.004..=137.8347).contains(&lon) {
            return true;
        }
        if !(0.8293..=55.8271).contains(&lat) {
            return true;
        }
        false
    }

    if out_of_china(lat, lon) {
        return (lat, lon);
    }

    let mut d_lat = transform_lat(lon - 105.0, lat - 35.0);
    let mut d_lon = transform_lon(lon - 105.0, lat - 35.0);
    let rad_lat = lat / 180.0 * PI;
    let magic = (1.0 - EE * rad_lat.sin() * rad_lat.sin()).sqrt();
    d_lat = (d_lat * 180.0) / ((A * (1.0 - EE)) / (magic * magic) * PI);
    d_lon = (d_lon * 180.0) / (A / magic * rad_lat.cos() * PI);
    let mg_lat = lat + d_lat;
    let mg_lon = lon + d_lon;
    (mg_lat, mg_lon)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LGPoint {
    longitude: f64,
    latitude: f64,
}

pub fn get_routine(mut mileage: f64, geojson_str: &str) -> Result<Vec<LGPoint>, Box<dyn Error>> {
    let mut points = Vec::new();
    let mut last = None;
    let mut rng = thread_rng();
    let geo_json: geojson::GeoJson = geojson_str.parse()?;
    let features = match geo_json {
        geojson::GeoJson::FeatureCollection(fc) => fc.features,
        _ => return Err("Invalid GeoJSON".into()),
    };

    let feature = features.first().ok_or("No feature found")?;
    let geometry = feature.geometry.as_ref().ok_or("No geometry found")?;
    let coordinates = match geometry.value {
        geojson::Value::LineString(ref ls) => ls,
        _ => return Err("Invalid geometry".into()),
    };

    if coordinates.is_empty() {
        return Err("No coordinates found".into());
    }

    loop {
        for coord in coordinates {
            let (y, x) = wgs84_to_gcj02(coord[1], coord[0]);
            let point = Point::new(x, y);
            if last.is_none() {
                last = Some(point);
            }

            let new = LGPoint {
                longitude: point.x() + rng.gen_range(-5e-6..5e-6),
                latitude: point.y() + rng.gen_range(-5e-6..5e-6),
            };
            mileage -= last.unwrap().geodesic_distance(&point) / 1000.;
            last = Some(point);

            points.push(new);

            if mileage <= 0. {
                return Ok(points);
            }
        }
    }
}
