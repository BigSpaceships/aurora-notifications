use anyhow::Result;
use chrono_tz::US::Eastern;

use crate::db::RecordDB;

mod db;
mod records;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let records = records::get_records_no_limit().await;
    // let records = records::get_records(2).await;

    let db_pool = db::connect_db().await;

    let records = records.unwrap();

    let num_records_added = RecordDB::add_all_records(&records, &db_pool).await?;

    println!("added {} records", num_records_added);

    for record in records {
        let local_time = record.issue_datetime.with_timezone(&Eastern);

        // let record = RecordDB::add_record(&record, &db_pool).await?;
        // println!("{:#?}", record);
    }

    return Ok(());
}
