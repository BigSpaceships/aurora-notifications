use std::env;

use chrono_tz::US::Eastern;
use sqlx::postgres::PgPoolOptions;

mod records;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let records = records::get_records(2).await;

    let db_pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Could not create pool");

    let records = records.unwrap();
    for record in records {
        let local_time = record.issue_datetime.with_timezone(&Eastern);

        println!("{:?}, {:#?}", local_time,  record);
    }
}
