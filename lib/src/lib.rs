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

mod routine;
mod security;
use const_format::formatcp;
use log::{debug, info};
use regex::Regex;
use routine::*;

use chrono::{DateTime, Duration, Local, Utc};
use rand::{thread_rng, Rng};
use reqwest::{header::*, Client, StatusCode};
use security::{decode_ns, sign_run_data, UploadRunningInfoBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, error::Error};

const URL_BASE: &str = env!("BACKEND");

const URL_CURRENT: &str = formatcp!("https://{}/education/semester/getCurrent", URL_BASE);

const URL_GET_RUNNING_LIMIT: &str = formatcp!("https://{}/running/app/getRunningLimit", URL_BASE);

const URL_GET_VERSION: &str = formatcp!(
    "https://{}/authorization/mobileApp/getLastVersion?platform=2",
    URL_BASE
);

const URL_LOGIN: &str = formatcp!("https://{}/authorization/user/v2/manage/login", URL_BASE);

const URL_UPLOAD_RUNNING: &str = formatcp!("https://{}/running//app/v3/upload", URL_BASE);

const ORGANIZATION: HeaderName = HeaderName::from_static("organization");

const HEADERS: [(HeaderName, &str); 9] = [
    (ACCEPT, "*/*"),
    (ACCEPT_ENCODING, "gzip, deflate, br"),
    (ACCEPT_LANGUAGE, "zh-CN, zh-Hans;q=0.9"),
    (AUTHORIZATION, ""),
    (CONNECTION, "keep-alive"),
    (CONTENT_TYPE, "application/json"),
    (HOST, URL_BASE),
    (ORGANIZATION, ""),
    (USER_AGENT, "Mozilla/5.0 (iPhone; CPU iPhone OS 15_4_1 like Mac OSX) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Html15Plus/1.0 (Immersed/47) uni-app"),
];

const CALORIE_PER_MILEAGE: f64 = 58.3;
const PACE: f64 = 360.;

fn format_json<T: Serialize>(json: T) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(": ")?;
    let json = serde_json::to_string_pretty(&json)?;

    Ok(re.replace_all(&json, " : ").to_string())
}

#[derive(Clone, Default)]
pub struct Account {
    client: Client,
    daily: f64,
    day: f64,
    end: f64,
    headers: HeaderMap,
    id: String,
    school_id: String,
    limitation: String,
    scoring: u8,
    semester: String,
    start: f64,
    token: String,
    version: String,
    week: f64,
    weekly: f64,
}

impl Account {
    /// Creates a new [`Account`].
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        for (key, val) in HEADERS {
            headers.insert(key, val.parse().unwrap());
        }

        let mut new = Self::default();

        new.headers = headers;

        new
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        self.set_token(username, password).await?;
        self.set_current().await?;
        self.set_version().await?;
        self.set_running_limit().await?;
        Ok(())
    }

    async fn set_token(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let sign_digital = security::hs(&format!("{}{}1", username, password));

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct LoginRequest {
            entrance: String,
            user_name: String,
            password: String,
            sign_digital: String,
        }

        let request = LoginRequest {
            entrance: "1".to_string(),
            user_name: username.to_string(),
            password: password.to_string(),
            sign_digital: sign_digital.to_string(),
        };

        let json = format_json(request)?;

        debug!("Login json: {}", json);

        let t = Utc::now().timestamp_millis();
        let encode_ns = security::encode_ns(&json, t)?;

        #[derive(Serialize, Deserialize, Debug)]
        struct SecurityBody {
            t: i64,
            pyd: String,
        }

        let request = SecurityBody { t, pyd: encode_ns };

        let res = self
            .client
            .post(URL_LOGIN)
            .headers(self.headers.clone())
            .json(&request)
            .send()
            .await?;

        if res.status() == StatusCode::BAD_REQUEST {
            return Err("Invalid account or password".into());
        }

        let res = res.error_for_status()?.text().await?;
        debug!("Login response: {}", res);

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct SecurityResponse {
            data: SecurityBody,
        }

        let data = serde_json::from_str::<SecurityResponse>(&res)?.data;

        let data = decode_ns(&data.pyd, data.t)?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct TokenData {
            id: String,
            organization_id: String,
            access_token: String,
            school_id: String,
        }

        let data: TokenData = serde_json::from_str(&data)?;

        self.id = data.id;
        self.token = data.access_token;
        self.school_id = data.school_id;
        self.headers
            .insert(ORGANIZATION, data.organization_id.parse()?);
        self.headers
            .insert(AUTHORIZATION, format!("Bearer {}", self.token).parse()?);

        info!("Get token successful!");
        Ok(())
    }

    async fn set_current(&mut self) -> Result<(), Box<dyn Error>> {
        let res = self
            .client
            .get(URL_CURRENT)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        debug!("Current response: {}", res);

        #[derive(Deserialize, Debug)]
        struct CurrentData {
            id: String,
        }

        #[derive(Deserialize)]
        struct CurrentResult {
            data: Option<CurrentData>,
        }

        let data = serde_json::from_str::<CurrentResult>(&res)?
            .data
            .ok_or("No current semester")?;

        self.semester = data.id;

        info!("Get current successful!");
        Ok(())
    }

    async fn set_version(&mut self) -> Result<(), Box<dyn Error>> {
        let res = self
            .client
            .get(URL_GET_VERSION)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        debug!("Version response: {}", res);
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct VersionData {
            version_label: String,
        }

        #[derive(Deserialize)]
        struct VersionResult {
            data: VersionData,
        }
        let data = serde_json::from_str::<VersionResult>(&res)?.data;

        self.version = data.version_label;

        info!("Get version successful!");
        Ok(())
    }

    async fn set_running_limit(&mut self) -> Result<(), Box<dyn Error>> {
        let json = json!({
            "semesterId": self.semester,
        });

        let res = self
            .client
            .post(URL_GET_RUNNING_LIMIT)
            .headers(self.headers.clone())
            .json(&json)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        debug!("Running limits response: {}", res);

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct RunningLimitsData {
            daily_mileage: Option<f64>,
            effective_mileage_end: Option<f64>,
            effective_mileage_start: Option<f64>,
            limitations_goals_sex_info_id: Option<String>,
            scoring_type: Option<u8>,
            total_day_mileage: Option<String>,
            total_week_mileage: Option<String>,
            weekly_mileage: Option<f64>,
        }

        #[derive(Deserialize)]
        struct RunningLimitsResult {
            data: RunningLimitsData,
        }

        let data = serde_json::from_str::<RunningLimitsResult>(&res)?.data;

        if let (
            Some(daily_mileage),
            Some(effective_mileage_end),
            Some(effective_mileage_start),
            Some(limitations_goals_sex_info_id),
            Some(scoring_type),
            Some(total_day_mileage),
            Some(total_week_mileage),
            Some(weekly_mileage),
        ) = (
            data.daily_mileage,
            data.effective_mileage_end,
            data.effective_mileage_start,
            data.limitations_goals_sex_info_id,
            data.scoring_type,
            data.total_day_mileage,
            data.total_week_mileage,
            data.weekly_mileage,
        ) {
            self.daily = daily_mileage;
            self.day = total_day_mileage.parse()?;
            self.end = effective_mileage_end;
            self.limitation = limitations_goals_sex_info_id;
            self.scoring = scoring_type;
            self.start = effective_mileage_start;
            self.week = total_week_mileage.parse()?;
            self.weekly = weekly_mileage;
        } else {
            return Err("Semester not started yet.".into());
        }

        info!("Get running limitation successful!");
        Ok(())
    }

    pub fn daily(&self) -> f64 {
        self.daily
    }

    pub async fn upload_running(
        &mut self,
        geojson_str: &str,
        mileage: f64,
        end_time: DateTime<Local>,
    ) -> Result<(), Box<dyn Error>> {
        let headers: HeaderMap<HeaderValue> = (&HashMap::<HeaderName, HeaderValue>::from([
            (HOST, URL_BASE.parse()?),
            (CONTENT_TYPE, "application/json".parse()?),
            (ACCEPT, "*/*".parse()?),
            (CONNECTION, "keep-alive".parse()?),
            (
                USER_AGENT,
                format!(
                    "QJGX/{} (com.ledreamer.legym; build:30000868; iOS 16.0.2) Alamofire/5.8.0",
                    self.version
                )
                .parse()?,
            ),
            (
                ACCEPT_ENCODING,
                "br;q=1.0, gzip;q=0.9, deflate;q=0.8".parse()?,
            ),
            (
                ACCEPT_LANGUAGE,
                "zh-Hans-HK;q=1.0, zh-Hant-HK;q=0.9, yue-Hant-HK;q=0.8".parse()?,
            ),
            (AUTHORIZATION, format!("Bearer {}", &self.token).parse()?),
        ]))
            .try_into()?;

        let mut mileage = mileage
            .min(self.daily - self.day)
            .min(self.weekly - self.week)
            .min(self.end);

        if mileage < self.start {
            return Err(String::from("Effective mileage too low").into());
        }

        let keep_time = {
            // WARN: Must make sure that the rng dies before the await call
            let mut rng = thread_rng();
            mileage += rng.gen_range(-0.02..-0.001);
            (mileage * PACE) as i64 + rng.gen_range(-15..15)
        };
        let pace_range = 0.59999999999999998;

        let start_time =
            end_time - Duration::try_seconds(keep_time + 8).ok_or("Invalid duration")?;

        let calorie = (CALORIE_PER_MILEAGE * mileage) as i64;
        let ave_pace = (keep_time as f64 / mileage) as i64 * 1000;
        let pace_number = (mileage * 1000. / pace_range / 2.) as i64;

        let sign_digital = security::hs(&format!(
            "{}{}{}{}{}{}{}{}{}",
            mileage,
            1,
            start_time.format("%Y-%m-%d %H:%M:%S"),
            calorie,
            ave_pace,
            keep_time,
            pace_number,
            mileage,
            1,
        ));

        let mut json = UploadRunningInfoBuilder::default()
            .app_version(self.version.clone())
            .ave_pace(ave_pace)
            .calorie(calorie)
            .device_type("iPhone 13 Pro".to_string())
            .effective_mileage(mileage)
            .effective_part(1)
            .end_time(end_time.format("%Y-%m-%d %H:%M:%S").to_string())
            .gps_mileage(mileage)
            .keep_time(keep_time)
            .limitations_goals_sex_info_id(self.limitation.clone())
            .pace_number(pace_number)
            .pace_range(pace_range)
            .routine_line(get_routine(mileage, geojson_str)?)
            .scoring_type(self.scoring)
            .semester_id(self.semester.clone())
            .sign_digital(sign_digital)
            .sign_point(vec![])
            .start_time(start_time.format("%Y-%m-%d %H:%M:%S").to_string())
            .system_version("16.0.2".to_string())
            .total_mileage(mileage)
            .total_part(1)
            .run_type("自由跑".to_string())
            .build()?;

        sign_run_data(&mut json, &self.id, &self.school_id)?;

        debug!("Upload running json: {}", format_json(&json)?);

        let res = self
            .client
            .post(URL_UPLOAD_RUNNING)
            .headers(headers)
            .json(&json)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        info!("Upload running successful!");
        debug!("Upload running response: {}", res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    use log::{Level, Metadata, Record};

    struct SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Debug
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("{} - {}", record.level(), record.args());
            }
        }

        fn flush(&self) {}
    }

    static LOGGER: SimpleLogger = SimpleLogger;

    #[tokio::test]
    async fn test_upload_running() {
        log::set_logger(&LOGGER).unwrap();

        log::set_max_level(log::LevelFilter::Debug);

        let username = env::var("USERNAME").unwrap();
        let password = env::var("PASSWORD").unwrap();

        let mut account = Account::new();
        account.login(&username, &password).await.unwrap();

        let geojson_str = include_str!("../../assets/map.geojson");
        let mileage = 5.0;
        let end_time = Local::now();

        account
            .upload_running(geojson_str, mileage, end_time)
            .await
            .unwrap();
    }
}
