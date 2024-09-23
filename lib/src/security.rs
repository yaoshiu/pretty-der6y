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

use std::error::Error;

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, BlockSizeUser, KeyInit};
use base64::prelude::*;
use chrono::{Local, NaiveDateTime, TimeZone};
use derive_builder::Builder;
use ecb::{Decryptor, Encryptor};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::LGPoint;

macro_rules! uncaesar {
    ($text: expr) => {{
        const TEXT: &str = $text;
        const LEN: usize = TEXT.len();
        const SHIFT: u8 = 3;

        const fn uncaesar(input: &[u8]) -> [u8; LEN] {
            let mut output = [0; LEN];
            let mut i = 0;

            while i < input.len() {
                let c = input[i];
                output[i] = if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = c.wrapping_sub(base);
                    base + (offset + 26 - SHIFT) % 26
                } else {
                    c
                };

                i += 1;
            }

            output
        }

        // Safety: EXPR is for sure a valid UTF-8 string
        unsafe { std::str::from_utf8_unchecked(&uncaesar(TEXT.as_bytes())) }
    }};
}

const SALT: &str = uncaesar!("lwdxYiqhaKlUljC6");

#[derive(Serialize, Deserialize, Default, Builder, Debug)]
#[builder(default)]
#[serde(rename_all = "camelCase")]
pub struct UploadRunningInfo {
    gps_mileage: f64,
    effective_part: u8,
    sign_time: String,
    keep_time: i64,
    device_type: String,
    ave_pace: i64,
    app_version: String,
    oct: String,
    sign_point: Vec<LGPoint>,
    end_time: String,
    limitations_goals_sex_info_id: String,
    semester_id: String,
    uneffective_reason: String,
    #[serde(rename = "type")]
    run_type: String,
    pace_number: i64,
    routine_line: Vec<LGPoint>,
    sign_digital: String,
    total_mileage: f64,
    total_part: u8,
    calorie: i64,
    effective_mileage: f64,
    system_version: String,
    pace_range: f64,
    scoring_type: u8,
    start_time: String,
}

pub fn hs(text: &str) -> String {
    let mut hasher = <Sha1 as Digest>::new();
    hasher.update(format!("{}{}", text, SALT));

    hex::encode(hasher.finalize())
}

fn encrypt(plain_data: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let secret_key = get_secret_key(key);
    let cipher = Encryptor::<aes::Aes128>::new(secret_key.as_slice().into());

    let mut buffer = vec![0u8; plain_data.len() + aes::Aes128::block_size()];
    buffer[..plain_data.len()].copy_from_slice(plain_data.as_bytes());

    let ciphertext = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, plain_data.len())
        .map_err(|e| e.to_string())?;

    Ok(BASE64_STANDARD.encode(ciphertext))
}

fn get_secret_key(key: &str) -> Vec<u8> {
    let mut secret_key = key.as_bytes().to_vec();
    secret_key.resize(16, 0);
    secret_key
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Oct<'a> {
    tp: u8,
    ep: u8,
    kt: i64,
    em: f64,
    rt: &'a str,
    uer: &'a str,
    xq: &'a str,
    dt: &'a str,
    bf: f64,
    bs: i64,
    zlc: f64,
    jf: u8,
    et: &'a str,
    lid: &'a str,
    kll: i64,
    app: &'a str,
    ap: i64,
    lcs: f64,
    st: &'a str,
    sv: &'a str,
}

pub fn sign_run_data(
    data: &mut UploadRunningInfo,
    a1: &str,
    a2: &str,
) -> Result<(), Box<dyn Error>> {
    let oct = Oct {
        tp: data.total_part,
        ep: data.effective_part,
        kt: data.keep_time,
        em: data.effective_mileage,
        rt: &data.run_type,
        uer: &data.uneffective_reason,
        xq: &data.semester_id,
        dt: &data.device_type,
        bf: data.pace_range,
        bs: data.pace_number,
        zlc: data.total_mileage,
        jf: data.scoring_type,
        et: &data.end_time,
        lid: &data.limitations_goals_sex_info_id,
        kll: data.calorie,
        app: &data.app_version,
        ap: data.ave_pace,
        lcs: data.gps_mileage,
        st: &data.start_time,
        sv: &data.system_version,
    };

    let json_data = serde_json::to_string_pretty(&oct)?;

    let re = Regex::new(": ")?;
    let formatted_json = re.replace_all(&json_data, " : ").to_string();

    let dy_key = get_rn_key(a1, a2);

    let end_time = NaiveDateTime::parse_from_str(&data.end_time, "%Y-%m-%d %H:%M:%S")?;
    let end_time = Local
        .from_local_datetime(&end_time)
        .single()
        .ok_or("Error getting end_time")?;

    let sign_time = end_time.timestamp() + data.keep_time % 11;
    let sign_time = Local
        .timestamp_opt(sign_time, 0)
        .single()
        .ok_or("Error getting sign_time")?;

    data.sign_time = sign_time.format("%Y-%m-%d %H:%M:%S").to_string();
    data.oct = encrypt(&formatted_json, &dy_key)?;

    Ok(())
}

const RN_FIXED: &str = uncaesar!("3h0783g6891d4d3h9521gfe6ee341560");

fn get_rn_key(a1: &str, a2: &str) -> String {
    let dest = &a1[3..6];
    let v14 = &a2[4..7];
    let v13 = &a1[9..12];

    format!("{}{}{}{}", dest, v14, v13, RN_FIXED)
}

const DYNAMIC_FIXED: &str = uncaesar!("402881hd7f39f5g5017f39g143d8062e");

fn get_dynamic_key(a1: &str) -> Result<String, Box<dyn Error>> {
    let dest = &a1[2..5];
    let nptr = &a1[4..8];
    let v2 = a1.chars().last().ok_or("Invalid string")?;

    let v1 = dest.parse::<i32>()?;
    let v3 = v1 - nptr.parse::<i32>()?;
    let v4 = v3.abs();
    let v5 = v4 << v2.to_digit(10).ok_or("Invalid digit")?;

    let fixed_string = DYNAMIC_FIXED;

    Ok(format!("{}{}", v5, fixed_string))
}

pub fn encode_ns(text: &str, t: i64) -> Result<String, Box<dyn Error>> {
    let key = get_dynamic_key(&t.to_string())?;
    encrypt(text, &key)
}

fn decrypt(text: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let secret_key = get_secret_key(key);
    let cipher = Decryptor::<aes::Aes128>::new(secret_key.as_slice().into());

    let ciphertext = BASE64_STANDARD.decode(text.as_bytes())?;
    let mut buffer = vec![0u8; ciphertext.len()];

    let plaintext_len = cipher
        .decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut buffer)
        .map_err(|e| e.to_string())?
        .len();

    Ok(String::from_utf8_lossy(&buffer[..plaintext_len]).to_string())
}

pub fn decode_ns(text: &str, t: i64) -> Result<String, Box<dyn Error>> {
    let key = get_dynamic_key(&t.to_string())?;
    decrypt(text, &key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hs1() {
        let text = "usernamepassword1";
        let expected = "0d9769667cccfa5ffcf6ecf0c389a177b34cef97";
        assert_eq!(hs(text), expected);
    }

    #[test]
    fn test_hs2() {
        let text = "1.337353775520865312024-09-20 20:49:5485219000294421.33735377552086531";
        let expected = "55237d619d2a7a374e4d874e3a0c5f5aafce346b";
        assert_eq!(hs(text), expected);
    }

    #[test]
    fn test_encrypt1() {
        let plain_data = r#"{
  "entrance" : "1",
  "userName" : "username",
  "password" : "password",
  "signDigital" : "0d9769667cccfa5ffcf6ecf0c389a177b34cef97"
}"#;
        let time = 1000000000000;
        let encoded = encode_ns(plain_data, time).unwrap();

        let expected = "ns7Q243GuyndUvGnNrdoF048oXxrHUJ4MnWXUJD7xlnl6wUXjLJFKrOrVTitJZ2AQq5DzJJIF3eIYiw6KZT4ty7Y5uvNDvB6OioDVZ06xYVEQhBH4G7yjMgpdxx1tHdIjU1fsOiEqlz8uY4QJWo0Tby+9guDCHkdh7cLZcvoyXde/GCWjWaJEuFudgd2eHHH";

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_decrypt1() {
        let encoded = "ns7Q243GuyndUvGnNrdoF048oXxrHUJ4MnWXUJD7xlnl6wUXjLJFKrOrVTitJZ2AQq5DzJJIF3eIYiw6KZT4ty7Y5uvNDvB6OioDVZ06xYVEQhBH4G7yjMgpdxx1tHdIjU1fsOiEqlz8uY4QJWo0Tby+9guDCHkdh7cLZcvoyXde/GCWjWaJEuFudgd2eHHH";
        let time = 1000000000000;
        let decoded = decode_ns(encoded, time).unwrap();

        let expected = r#"{
  "entrance" : "1",
  "userName" : "username",
  "password" : "password",
  "signDigital" : "0d9769667cccfa5ffcf6ecf0c389a177b34cef97"
}"#;

        assert_eq!(decoded, expected);
    }
}
