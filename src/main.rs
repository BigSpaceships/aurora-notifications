use chrono_tz::US::Eastern;

use crate::db::RecordDB;

mod db;
mod records;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let records = records::get_records(2).await;

    let db_pool = db::connect_db().await;

    let records = records.unwrap();

    // let hi = RecordDB::add_record(&records[0], &db_pool).await;
    // let hi = RecordDB::add_all_records(&records, &db_pool).await;

    for record in records {
        let local_time = record.issue_datetime.with_timezone(&Eastern);

        RecordDB::add_record(&record, &db_pool).await;
        // println!("{:?}, {:#?}", local_time,  record);
    }
}
