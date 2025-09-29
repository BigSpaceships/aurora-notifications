use chrono_tz::US::Eastern;

mod records;

fn main() {
    let records = records::get_records(2);

    let records = records.unwrap();
    for record in records {
        let local_time = record.issue_datetime.with_timezone(&Eastern);

        println!("{:?}, {:#?}", local_time,  record);
    }
}
