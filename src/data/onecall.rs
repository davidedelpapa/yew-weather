#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OneCall {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: usize,
    pub current: Option<WeatherUnit>,
    pub hourly: Option<Vec<WeatherUnit>>,
    pub daily: Option<Vec<WeatherDaily>>,
    pub minutely: Option<Vec<Minutely>>,
}
impl OneCall {
    pub fn new() -> Self {
        OneCall {
            lat: 0.0,
            lon: 0.0,
            timezone: String::new(),
            timezone_offset: 0,
            current: None,
            hourly: None,
            daily: None,
            minutely: None,
        }
    }
}


#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeatherUnit{
    pub dt: Option<usize>,
    pub sunrise: Option<usize>,
    pub sunset: Option<usize>,
    pub temp: Option<f32>,
    pub feels_like: Option<f32>,
    pub pressure: Option<u32>,
    pub humidity: Option<u32>,
    pub dew_point: Option<f32>,
    pub uvi: Option<f32>,
    pub clouds: Option<f32>,
    pub visibility: Option<usize>,
    pub wind_speed: Option<f32>,
    pub wind_deg: Option<u32>,
    pub wind_gust: Option<f32>,
    pub rain: Option<VolumesInfo>,
    pub snow: Option<VolumesInfo>,
    pub weather: Vec<Option<WeatherCondition>>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyUnit {
    pub morn: f32,
    pub eve: f32,
    pub day: f32,
    pub night: f32,
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeatherDaily{
    pub dt: Option<usize>,
    pub sunrise: Option<usize>,
    pub sunset: Option<usize>,
    pub temp: Option<DailyUnit>,
    pub feels_like: Option<DailyUnit>,
    pub pressure: Option<u32>,
    pub humidity: Option<u32>,
    pub dew_point: Option<f32>,
    pub uvi: Option<f32>,
    pub clouds: Option<f32>,
    pub visibility: Option<usize>,
    pub wind_speed: Option<f32>,
    pub wind_deg: u32,
    pub wind_gust: Option<f32>,
    pub rain: Option<f32>,
    pub snow: Option<f32>,
    pub weather: Vec<Option<WeatherCondition>>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeatherCondition{
    pub id: u16,
    pub main: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VolumesInfo{
    #[serde(rename = "1h")]
    pub r1h: f32
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Minutely{
    pub dt: Option<usize>,
    pub precipitation: Option<f32>,
}

