use std::env;

use anyhow::{Result, anyhow};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, Postgres, postgres::PgPoolOptions, query, query_as};

use crate::records::Record;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordDB {
    issue_time: NaiveDateTime,
    serial_num: i32,
    serial_num_ext: Option<i32>,
    id: i32,
}

impl RecordDB {
    pub async fn add_record<'c, C>(data: &Record, conn: C) -> Result<Self>
    where
        C: Executor<'c, Database = Postgres>,
    {
        query_as!(RecordDB, r#"
        WITH new_record AS (
            INSERT INTO event (issue_time, serial_num, serial_num_ext) VALUES ($1, $2, $3) RETURNING *
        )
        SELECT new_record.issue_time, new_record.serial_num, new_record.serial_num_ext, new_record.id
        FROM new_record
        "#, data.issue_datetime.naive_local(), data.message.sn, data.message.sn_ext)
        .fetch_one(conn)
        .await.map_err(|err| anyhow!("Failed to Create Event: {}", err))
    }
}

pub async fn connect_db() -> Pool<Postgres> {
    let db_pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Could not create pool");

    return db_pool;
}
