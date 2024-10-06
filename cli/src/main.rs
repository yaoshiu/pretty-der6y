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

use std::{fs::File, io::Read};

use clap::Parser;
use lib::{
    chrono::{Local, NaiveDateTime},
    Account,
};
use log::{debug, info, Level, Metadata, Record};

struct SimpleLogger {
    level: Level,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    password: String,
    #[arg(short, long)]
    mileage: f64,
    #[arg(short, long)]
    route: String,
    #[arg(short, long)]
    time: Option<String>,

    /// Verbosity level
    #[arg(short, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let logger = SimpleLogger {
        level: match args.verbose {
            0 => Level::Info,
            1 => Level::Debug,
            _ => Level::Trace,
        },
    };

    let level_filter = logger.level.to_level_filter();

    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level_filter))?;

    let mut account = Account::new();

    info!("Logging in");
    debug!("Username: {}", args.username);
    debug!("Password: {}", args.password);

    account.login(&args.username, &args.password).await?;

    let time = match args.time {
        Some(time) => NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S")
            .map(|t| t.and_local_timezone(Local).earliest().unwrap())?,
        None => Local::now(),
    };

    let mut file = File::open(&args.route)?;
    let mut geojson = String::new();
    file.read_to_string(&mut geojson)?;

    info!("Uploading running data");
    debug!("Route: {}", geojson);
    debug!("Mileage: {}", args.mileage);
    debug!("Time: {}", time);

    account
        .upload_running(&geojson, args.mileage, &time)
        .await?;

    Ok(())
}
