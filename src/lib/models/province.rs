use super::geopos::Geopos;
use geocoding::Point;
use std::hash::{Hash, Hasher};

use std::fs;

const PROVINCE_POSITION_FILE: &'static str = "data/provinces.csv";

#[derive(Debug, Deserialize)]
pub struct Province {
    id: u32,
    latitude: f64,
    longitude: f64,
}

impl Province {
    pub fn new(id: u32, latitude: f64, longitude: f64) -> Self {
        Self {
            id,
            latitude,
            longitude,
        }
    }

    pub fn load() -> Vec<Self> {
        let mut tab: Vec<Self> = Vec::new();
        // get province pos as vec
        let province_position_content = fs::read_to_string(PROVINCE_POSITION_FILE).expect(
            &format!("Error : could not read file {}", PROVINCE_POSITION_FILE),
        );

        let mut rdr = csv::Reader::from_reader(province_position_content.as_bytes());
        for result in rdr.deserialize() {
            let province: Self = result.unwrap();
            tab.push(province);
        }

        tab
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Geopos for Province {
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

impl Hash for Province {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        (self.longitude as u64).hash(state);
        (self.latitude as u64).hash(state);
    }
}

impl PartialEq for Province {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Province {}
