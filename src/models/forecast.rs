use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug)]
pub struct Forecast {
    station: String,
    name: String,
    date: NaiveDate,
    pub tmin: f32,
    pub tavg: f32,
    pub tmax: f32,
    pub precipitation: f32,
    pub wind_gust: f32,
    pub wind_speed: f32,
}

impl Forecast {
    pub fn new(
        station: String,
        name: String,
        date: NaiveDate,
        tmin: f32,
        tavg: f32,
        tmax: f32,
        precipitation: f32,
        wind_gust: f32,
        wind_speed: f32,
    ) -> Self {
        Self {
            station,
            name,
            date,
            tmin,
            tavg,
            tmax,
            precipitation,
            wind_gust,
            wind_speed,
        }
    }

    pub fn timestamp(&self) -> String {
        let naive_time = NaiveTime::from_hms(0, 0, 0);

        // 2008-04-30 00:00:00.000
        format!(
            "{}",
            NaiveDateTime::new(self.date, naive_time).format("%Y-%m-%d %H:%M:%S%.3f")
        )
    }
}

