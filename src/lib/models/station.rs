use super::geopos::Geopos;
use geocoding::Point;
use regex::Regex;

use std::fs;

const STATION_POSITION_FILE: &'static str = "data/rel_station_pos.csv";

#[derive(Debug, Deserialize)]
pub struct Station {
    pub code: String,
    latitude: f64,
    longitude: f64,
}

impl Station {
    pub fn new(code: String, latitude: f64, longitude: f64) -> Self {
        Self {
            code,
            latitude,
            longitude,
        }
    }

    pub fn load() -> Vec<Self> {
        // check availables stations (in 2010 and assume its ok for other years)
        let re = Regex::new(r#"href="(?P<value>(\w\d+)\.csv)""#).unwrap();
        let mut year_available: Vec<Vec<String>> = Vec::new();
        for year in 2011..=2015 {
            let year_page = reqwest::blocking::get(&format!(
                "https://www.ncei.noaa.gov/data/global-summary-of-the-day/access/{}/",
                year,
            ))
            .unwrap()
            .text()
            .unwrap();
            // 5.3) loop through stations
            let mut available_stations_ids: Vec<String> = Vec::new();
            for captured in re.captures_iter(&year_page) {
                available_stations_ids.push(captured["value"].replace(".csv", "").to_string());
            }
            year_available.push(available_stations_ids);
        }

        let compiled_available_stations_ids =
            year_available.iter().fold(Vec::new(), |result, tab| {
                if result.len() == 0 {
                    tab.iter().map(|code| code.into()).collect::<Vec<String>>()
                } else {
                    result
                        .iter()
                        .filter(|code| tab.contains(code))
                        .map(|code| code.into())
                        .collect::<Vec<String>>()
                }
            });

        // get station pos as vec
        let station_position_content = fs::read_to_string(STATION_POSITION_FILE).expect(&format!(
            "Error : could not read file {}",
            STATION_POSITION_FILE
        ));

        let mut rdr = csv::Reader::from_reader(station_position_content.as_bytes());
        let mut tab: Vec<Self> = Vec::new();
        for result in rdr.deserialize() {
            let station: Self = result.unwrap();
            if compiled_available_stations_ids.contains(&station.code) {
                tab.push(station);
            }
        }

        tab
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }
}

impl Geopos for Station {
    fn latitude(&self) -> f64 {
        self.latitude
    }

    fn longitude(&self) -> f64 {
        self.longitude
    }

    fn geopos(&self) -> Point<f64> {
        unimplemented!()
    }
}
