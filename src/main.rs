extern crate chrono;
extern crate geocoding;
extern crate regex;
extern crate reqwest;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

pub mod models;

use models::geopos::Geopos;
use models::{AWForecast, Forecast, Province, Station};

use chrono::NaiveDate;
use regex::Regex;

use std::collections::HashMap;
use std::f64;
use std::fs::File;
use std::io::Write;

const OUTPUT_FILE: &'static str = "data/forecast.csv";

fn main() {
    // print_time::time_csv("data/time.csv");
    get_all_station_reports()
}

fn get_all_station_reports() {
    // 1) get provinces
    let provinces = Province::load();
    assert_eq!(provinces.len(), 177);

    // 2) get stations
    let stations = Station::load();

    // 3) associate a station for each province
    let tab: HashMap<&Province, &Station> = associate_province_and_station(&provinces, &stations);
    assert_eq!(tab.len(), 177);

    // 4) collect the stations ids
    let stations_ids: Vec<String> = tab.values().map(|station| station.get_code()).collect();
    assert_eq!(stations_ids.len(), 177);

    let re = Regex::new(r#"href="(?P<value>(\w\d+)\.csv)""#).unwrap();

    // 5) loop through years
    let mut file_downloaded = 0;
    let mut file_skipped = 0;
    let mut forecast_tab: Vec<AWForecast> = Vec::new();
    for year in 2011..=2015 {
        // 5.1) get file
        let year_page = reqwest::blocking::get(&format!(
            "https://www.ncei.noaa.gov/data/global-summary-of-the-day/access/{}/",
            year
        ))
        .unwrap()
        .text()
        .unwrap();

        // 5.3) loop through stations
        for captured in re.captures_iter(&year_page) {
            let file_name = &captured["value"];

            // 5.2) isolate the stations from our list
            if !stations_ids.contains(&file_name.replace(".csv", "")) {
                // println!("skipping {}", &file_name);
                file_skipped += 1;
                continue;
            }

            print!("downloading file : {} ... ", &file_name);

            // 5.3.1) fetch content
            let file_content = reqwest::blocking::get(&format!(
                "https://www.ncei.noaa.gov/data/global-summary-of-the-day/access/{}/{}",
                year, &file_name
            ))
            .unwrap()
            .text()
            .unwrap();

            // 5.3.2) deserialize it
            for forecast in read(file_content) {
                if let Some(st) = find_key_for_value(&tab, &file_name.replace(".csv", "")) {
                    forecast_tab.push(AWForecast::new(
                        forecast.timestamp(),
                        st.id(),
                        forecast.tmin,
                        forecast.tavg,
                        forecast.tmax,
                        forecast.precipitation,
                        forecast.wind_gust,
                        forecast.wind_speed,
                    ));
                } else {
                    println!(
                        "Error : state not found for given station code {}",
                        file_name.replace(".csv", "")
                    );
                }
            }
            file_downloaded += 1;
            println!("done !");
        }
        // 5.3.3) generate SQL inserts with all information above
        // 5.4) write it into a file
    }
    println!(
        "\n\nFiles skipped :\t\t\t{}\nFiles downloaded & parsed :\t{}",
        file_skipped, file_downloaded
    );
    write_to_file(OUTPUT_FILE, forecast_tab);
}

fn find_key_for_value<'a, 'b>(
    map: &HashMap<&'a Province, &'b Station>,
    station_code: &str,
) -> Option<&'a Province> {
    map.iter().find_map(|(&key, &val)| {
        if &val.get_code() == station_code {
            Some(key)
        } else {
            None
        }
    })
}

fn associate_province_and_station<'a, 'b>(
    provinces: &'a [Province],
    stations: &'b [Station],
) -> HashMap<&'a Province, &'b Station> {
    let mut tab: HashMap<&Province, &Station> = HashMap::new();

    for province in provinces {
        let mut min_dist = f64::MAX;
        let mut closest_station: Option<&Station> = None;
        for station in stations.iter() {
            let dist = province.distance(station);
            if dist < min_dist {
                min_dist = dist;
                closest_station = Some(station);
            }
        }
        tab.insert(province, closest_station.unwrap());
    }

    tab
}

fn write_to_file(file_name: &str, content: Vec<AWForecast>) {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for forecast in content {
        wtr.serialize(forecast).unwrap()
    }

    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

    let mut file = File::create(file_name).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn read(input: String) -> Vec<Forecast> {
    let mut tab: Vec<Forecast> = Vec::new();

    let mut rdr = csv::Reader::from_reader(input.as_bytes());
    for result in rdr.records() {
        let record = result.unwrap();

        let station: String = record[0].parse().unwrap();
        let name: String = record[5].parse().unwrap();
        let date: NaiveDate = record[1].parse().unwrap();
        let tmin: f32 = record[22]
            .parse::<String>()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap();
        let tavg: f32 = record[6]
            .parse::<String>()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap();
        let tmax: f32 = record[20]
            .parse::<String>()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap();
        let precipitation: f32 = record[24]
            .parse::<String>()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap();
        let wind_gust: f32 = record[19]
            .parse::<String>()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap();
        let wind_speed: f32 = record[16]
            .parse::<String>()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap();

        let f = Forecast::new(
            station,
            name,
            date,
            tmin,
            tavg,
            tmax,
            precipitation,
            wind_gust,
            wind_speed,
        );

        tab.push(f);
    }

    tab
}
