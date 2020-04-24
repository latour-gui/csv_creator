use super::geopos::Geopos;
use geocoding::Point;

use std::fs;
use std::hash::{Hash, Hasher};

const STATE_POSITION_FILE: &'static str = "data/rel_state_pos.csv";
const DEPARTEMENT_POSITION_FILE: &'static str = "data/rel_departement_pos.csv";

#[derive(Debug, Deserialize)]
pub struct State {
    id: u32,
    code: String,
    name: String,
    latitude: f64,
    longitude: f64,
}

impl State {
    pub fn new(id: u32, code: String, name: String, latitude: f64, longitude: f64) -> Self {
        Self {
            id,
            code,
            name,
            latitude,
            longitude,
        }
    }

    pub fn load() -> Vec<Self> {
        let mut tab: Vec<Self> = Vec::new();
        // get state pos as vec
        let state_position_content = fs::read_to_string(STATE_POSITION_FILE).expect(&format!(
            "Error : could not read file {}",
            STATE_POSITION_FILE
        ));

        let mut rdr = csv::Reader::from_reader(state_position_content.as_bytes());
        for result in rdr.deserialize() {
            let state: Self = result.unwrap();
            tab.push(state);
        }

        let departement_position_content = fs::read_to_string(DEPARTEMENT_POSITION_FILE).expect(
            &format!("Error : could not read file {}", DEPARTEMENT_POSITION_FILE),
        );

        let mut rdr = csv::Reader::from_reader(departement_position_content.as_bytes());
        for result in rdr.records() {
            let record = result.unwrap();

            let id: u32 = record[4].parse().unwrap();
            let code: String = record[0].parse().unwrap();
            let name: String = record[1].parse().unwrap();
            let latitude_deg_min_sec: String = record[3].parse().unwrap();
            let longitude_deg_min_sec: String = record[2].parse().unwrap();

            let couple = super::geopos::convert_deg_min_sec_to_float(
                &latitude_deg_min_sec,
                &longitude_deg_min_sec,
            )
            .unwrap();

            let latitude = couple.0;
            let longitude = couple.1;

            let departement = State::new(id, code, name, latitude, longitude);

            tab.push(departement);
        }

        tab
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Geopos for State {
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

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.name == other.name
    }
}

impl Eq for State {}
