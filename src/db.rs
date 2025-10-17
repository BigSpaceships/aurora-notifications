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
    id_ext: Option<i32>,
}

impl RecordDB {
    pub async fn add_record<'c, C>(data: &Record, conn: C) -> Result<Self>
    where
        C: Executor<'c, Database = Postgres> + Copy,
    {
        let id = query!(r#"
        WITH new_record AS (
            INSERT INTO event (issue_time, serial_num, serial_num_ext, id_ext) VALUES ($1, $2, $3, NULL)
            RETURNING *
        )
        SELECT new_record.id
        FROM new_record
        "#, data.issue_datetime.naive_local(), data.message.sn, data.message.sn_ext)
        .fetch_one(conn)
        .await.map_err(|err| anyhow!("Failed to Create Event: {}", err))?;

        update_id_references(conn).await?;

        query_as!(
            RecordDB,
            r#"
            SELECT issue_time, serial_num, serial_num_ext, id, id_ext FROM event WHERE id = $1
        "#,
            id.id
        )
        .fetch_one(conn)
        .await
        .map_err(|err| anyhow!("Failed to fetch updated event: {}", err))
    }

    pub async fn add_all_records<'c, C>(data: &Vec<Record>, conn: C) -> Result<usize>
    where
        C: Executor<'c, Database = Postgres> + Copy,
    {
        let sns: Vec<i32> = data.iter().map(|record| record.message.sn).collect();
        let sn_exts: Vec<i32> = data
            .iter()
            .map(|record| record.message.sn_ext.unwrap_or(-1))
            .collect();
        let issue_times: Vec<NaiveDateTime> = data
            .iter()
            .map(|record| record.issue_datetime.naive_local())
            .collect();
        let added_records = query_as!(RecordDB,
            r#"
            INSERT INTO event (issue_time, serial_num, serial_num_ext, id_ext)
            SELECT issue_time, serial_num, NULLIF(serial_num_ext, -1) as serial_num_ext, NULL
            FROM UNNEST($1::int4[], $2::int4[], $3::timestamp[]) as a(serial_num, serial_num_ext, issue_time)
            WHERE serial_num NOT IN (SELECT serial_num FROM event)
                AND issue_time NOT IN (SELECT issue_time FROM event)
            RETURNING *
            "#,
            &sns, &sn_exts, &issue_times
        )
        .fetch_all(conn)
        .await
        .map_err(|err| anyhow!("Failed to upload events: {}", err))?;

        RecordDB::update_id_references(conn).await?;

        return Ok(added_records.len());
    }

    pub async fn update_id_references<'c, C>(conn: C) -> Result<()>
    where
        C: Executor<'c, Database = Postgres> + Copy,
    {
        query!(
            r#"
            UPDATE event as e 
            SET id_ext = other.id
            FROM event AS other
            WHERE e.serial_num_ext = other.serial_num
            "#
        )
        .execute(conn)
        .await?;

        Ok(())
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
