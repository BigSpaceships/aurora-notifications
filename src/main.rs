use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RecordJson {
    issue_datetime: String,
    message: String,
    product_id: String,
}


fn get_json() -> Result<String, reqwest::Error> {
    let res =
        reqwest::blocking::get("https://services.swpc.noaa.gov/products/alerts.json")?.text()?;

    return Ok(res);
}

fn parse_json(text: &str) -> Result<Vec<RecordJson>, serde_json::Error> {
    let v: Vec<RecordJson> = serde_json::from_str(text)?;

    return Ok(v);
}

fn main() {
    match get_json() {
        Ok(res) => {
            let json = parse_json(&res);

            match json {
                Ok(v) => {
                    println!("{:#?}", v)
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
}
