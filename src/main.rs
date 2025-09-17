mod records;

fn main() {
    let records = records::get_records(2);

    println!("{:#?}", records);
}
