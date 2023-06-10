use csv::Reader;
use geo::{prelude::*, Point};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};
use wkt::TryFromWkt;

const DEFAULT_ROUTINE: &'static str = include_str!("../../data/default_routine.csv");

#[derive(Serialize)]
pub struct LGPoint {
    longitude: f64,
    latitude: f64,
}

pub fn get_routine(
    mut mileage: f64,
    filepath: Option<String>,
) -> Result<Vec<LGPoint>, Box<dyn Error>> {
    let rdr = match filepath {
        Some(fp) => fs::read_to_string(Path::new(&fp))?,
        None => DEFAULT_ROUTINE.into(),
    };

    let mut rdr = Reader::from_reader(rdr.as_bytes());
    let mut rng = thread_rng();

    let mut res = Vec::new();

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct Record {
        WKT: String,
    }

    let mut points: Vec<Point<f64>> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        let point: Point<f64> = Point::try_from_wkt_str(record.WKT.as_str())?;
        points.push(point);
    }
    let mut last = None;
    loop {
        for point in &points {
            if last.is_none() {
                last = Some(point);
            }

            let new = LGPoint {
                longitude: point.x() + rng.gen_range(-2e-5..2e-5),
                latitude: point.y() + rng.gen_range(-1e-5..1e-5),
            };
            mileage -= last.unwrap().geodesic_distance(&point) / 1000.;
            last = Some(point);

            res.push(new);
            if mileage <= 0. {
                return Ok(res);
            }
        }
    }
}
