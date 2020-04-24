use chrono::NaiveDateTime;
use std::fs::File;
use std::io::Write;

pub fn time_csv(file_name: &str) {
    let mut days: Vec<String> = Vec::new();

    // from Saturday, January 1, 2005 1:00:00 PM
    // to Wednesday, January 1, 2020 1:00:00 PM
    // by 24 hours
    for timestamp in (1104584400..=1577883600).step_by(86_400) {
        let date = NaiveDateTime::from_timestamp(timestamp, 0);

        let str_date = format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
        let day = format!("{}", date.format("%A"));
        let week = format!("{}", date.format("%V")).parse::<u32>().unwrap();

        days.push(format!("{},{},{}", str_date, day, week));
    }

    let data = format!("Date,Day,Week\n{}", days.join("\n"));
    let mut file = File::create(file_name).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}


