use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewWeather {
    pub data: NewWeatherData,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewWeatherData {
    pub clear_weather_time: i32,
    pub raining: bool,
    pub rain_time: i32,
    pub thundering: bool,
    pub thunder_time: i32,
}
