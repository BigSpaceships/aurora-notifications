use std::{default, fmt};

use anyhow::{Result, anyhow};
use chrono::{DateTime, Days, FixedOffset, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum IDError {
    #[error("invalid ID format: {0}")]
    Format(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct RecordJson {
    issue_datetime: String,
    message: String,
    product_id: String,
}

#[derive(Debug)]
pub struct Record {
    pub issue_datetime: DateTime<FixedOffset>,
    pub message: String,
    pub product_id: ID,
}

#[derive(Debug)]
pub enum ID {
    Mag {
        strength: u8,
        severity: WarningSeverity,
    },
    Flux {
        strength: u8,
        severity: WarningSeverity,
    },
    Message(String),
}

#[derive(Debug)]
pub enum WarningSeverity {
    Warning,
    Alert,
    Forcast,
}

impl ID {
    fn from_string(string: String) -> Result<ID> {
        // TODO: maybe I should just use the space weather message code in the message
        let re = Regex::new(r"(\A[A-Z]{1,2})(\d+)([AFWS])").unwrap();

        let caps = re.captures(&string);

        if caps.is_none() {
            return Ok(ID::Message(string.clone()));
        }

        let caps = caps.unwrap();

        let key = &caps[1];
        let level = &caps[2].parse::<u8>()?;
        let severitiy_str = &caps[3];

        let severity = match severitiy_str {
            "A" => WarningSeverity::Alert,
            "S" => WarningSeverity::Alert,
            "W" => WarningSeverity::Warning,
            "F" => WarningSeverity::Forcast,
            string => Err(anyhow!("unknown severity: {string}"))?,
        };

        return Ok(match key {
            "K" => ID::Mag {
                strength: *level,
                severity: severity,
            },
            "EF" => ID::Flux {
                strength: *level,
                severity: severity,
            },
            key => {
                println!("{key}");
                ID::Message(key.to_string())
            }
        });
    }
}

#[derive(Debug)]
pub struct RecordMessage {
    space_weather_access_code: String,
    sn: u64,
    issue_time: DateTime<FixedOffset>,
    message: String,
}

impl RecordMessage {
    fn from_message(message: String) -> Result<RecordMessage> {
        let re = Regex::new(
            r"(?ms)Space Weather Message Code:\s*(.*?)\s*Serial Number:\s*(\d*)\s*Issue Time:\s*(.{20})\s*(.*)",
        )?;

        let caps = re.captures(&message);

        if caps.is_none() {
            return Err(anyhow!("invalid format of record message: {message}"));
        }

        let caps = caps.unwrap();
        // 2025 Sep 29 1200 UTC
        let time_string = &caps[3];
        let time_string = time_string[..time_string.len()-3].to_string() + "+0000 00.000";

        let time = DateTime::parse_from_str(&time_string, "%Y %b %d %H%M %z %S%.3f")?;

        return Ok(RecordMessage {
            space_weather_access_code: caps[1].to_string(),
            sn: caps[2].parse::<u64>()?,
            issue_time: time,
            message: caps[4].to_string(),
        });
    }
}

impl Record {
    fn from_json(json: RecordJson) -> Result<Record> {
        // 2025-09-29 12:00:08.047
        let new_time_string = json.issue_datetime + "+0000";
        let time = DateTime::parse_from_str(&new_time_string, "%Y-%m-%d %H:%M:%S%.3f %z")?;

        let id = ID::from_string(json.product_id)?;

        // print!("{:#?}", RecordMessage::from_message(json.message.clone()));

        return Ok(Record {
            issue_datetime: time,
            message: json.message,
            product_id: id,
        });
    }
}

fn fetch_json() -> Result<String> {
    let res =
        reqwest::blocking::get("https://services.swpc.noaa.gov/products/alerts.json")?.text()?;

    return Ok(res);
}

fn parse_json(text: &str) -> Result<Vec<Record>> {
    let v_json: Vec<RecordJson> = serde_json::from_str(text)?;

    let v: Result<Vec<Record>> = v_json.into_iter().map(|r| Record::from_json(r)).collect();

    return v;
}

fn filter_records(records: Vec<Record>, days_in_past: u8) -> Vec<Record> {
    let now: DateTime<Utc> = Utc::now();
    let filter_date = now - Days::new(days_in_past.into());

    return records
        .into_iter()
        .filter(|record| record.issue_datetime > filter_date)
        .collect();
}

pub fn get_records(days_in_past: u8) -> Result<Vec<Record>> {
    let json_string = fetch_json()?;

    let all_records = parse_json(&json_string)?;

    let filtered_records = filter_records(all_records, days_in_past);

    return Ok(filtered_records);
}
