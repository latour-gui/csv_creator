#[derive(Debug, Serialize, Deserialize)]
pub struct AWForecast {
    timestamp: String,
    state_province_id: u32,
    tmin: f32,
    tavg: f32,
    tmax: f32,
    precipitation: f32,
    wind_gust: f32,
    wind_speed: f32,
}

impl AWForecast {
    pub fn new(
        timestamp: String,
        state_province_id: u32,
        tmin: f32,
        tavg: f32,
        tmax: f32,
        precipitation: f32,
        wind_gust: f32,
        wind_speed: f32,
    ) -> Self {
        Self {
            timestamp,
            state_province_id,
            tmin,
            tavg,
            tmax,
            precipitation,
            wind_gust,
            wind_speed,
        }
    }
}
