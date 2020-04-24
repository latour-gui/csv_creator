use geocoding::Point;
use regex::Regex;
use std::f64::consts::PI;

pub trait Geopos {
    fn latitude(&self) -> f64;
    fn longitude(&self) -> f64;
    fn geopos(&self) -> Point<f64>;

    fn distance<G: Geopos>(&self, b: &G) -> f64 {
        let r = 6_371.0; // kilometres

        let lat_1 = self.latitude() * PI / 180.0;
        let lat_2 = b.latitude() * PI / 180.0;

        let delta_latitude = lat_2 - lat_1;
        let delta_longitude = (b.longitude() - self.longitude()) * PI / 180.0;

        let haversine = f64::powi(f64::sin(delta_latitude / 2f64), 2)
            + f64::cos(lat_1) * f64::cos(lat_2) * f64::powi(f64::sin(delta_longitude / 2f64), 2);

        let c = 2f64 * f64::atan2(f64::sqrt(haversine), f64::sqrt(1f64 - haversine));

        r * c
    }
}

/// Returns (lat,long)
pub fn convert_deg_min_sec_to_float(
    latitude: &str,
    longitude: &str,
) -> Result<(f64, f64), &'static str> {
    let re = Regex::new(
        r#"^(?P<degree>\d{1,2})°(?P<minute>\d{2})'(?P<second>\d{2})"(?P<orientation>E|O|N|S)?$"#,
    )
    .unwrap();

    if !re.is_match(&latitude) || !re.is_match(&longitude) {
        return Err("Error : at least one of the input is malformated");
    }

    let extract_info = |input: &str| -> f64 {
        let c = re.captures(&input).unwrap();

        let is_positive = match c.name("orientation") {
            Some(_) => &c["orientation"] != "O" && &c["orientation"] != "S",
            None => true,
        };
        let degree = c["degree"].parse::<f64>().unwrap();
        let minute = c["minute"].parse::<f64>().unwrap();
        let second = c["second"].parse::<f64>().unwrap();

        let value = degree + minute / 60.0 + second / 3600.0;
        if is_positive {
            value
        } else {
            -value
        }
    };

    Ok((extract_info(latitude), extract_info(longitude)))
}
