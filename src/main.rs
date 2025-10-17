use chrono_tz::US::Eastern;
use anyhow::Result;

use crate::db::RecordDB;

mod db;
mod records;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv::dotenv().ok();

    let records = records::get_records_no_limit().await;

    let db_pool = db::connect_db().await;

    let records = records.unwrap();

    // let hi = RecordDB::add_record(&records[0], &db_pool).await;
    let num_records_added = RecordDB::add_all_records(&records, &db_pool).await?;

    println!("added {} records", num_records_added);

    for record in records {
        let local_time = record.issue_datetime.with_timezone(&Eastern);

        // RecordDB::add_record(&record, &db_pool).await;
        // println!("{:?}, {:#?}", local_time,  record);
    }

    return Ok(())
}
