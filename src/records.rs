use chrono::{DateTime, Days, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
struct RecordJson {
    issue_datetime: String,
    message: String,
    product_id: String,
}

#[derive(Debug)]
pub struct Record {
    issue_datetime: DateTime<FixedOffset>,
    message: String,
    product_id: String,
}

impl Record {
    fn from_json(json: RecordJson) -> Result<Record> {
        let new_time_string = json.issue_datetime + "+0000";
        let time = DateTime::parse_from_str(&new_time_string, "%Y-%m-%d %H:%M:%S%.3f %z")?;

        return Ok(Record { issue_datetime: time, message: json.message, product_id: json.product_id })
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

fn filter_records(records: Vec<Record>) -> Vec<Record> {
    let now: DateTime<Utc> = Utc::now();
    let yesterday = now - Days::new(1);

    return records.into_iter().filter(|record| record.issue_datetime > yesterday).collect();
}

pub fn get_records(days_in_past: u8) -> Result<Vec<Record>> {
    let json_string = fetch_json()?;

    let all_records = parse_json(&json_string)?;

    let filtered_records = filter_records(all_records);

    return Ok(filtered_records);
}
