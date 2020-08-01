use crate::data::onecall::OneCall;
use anyhow::{anyhow, Error};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};


#[derive(Default)]
pub struct WeatherService {
    lat: f64,
    lon: f64,
    unit_system: String,
    api_key: String,
}

impl WeatherService {
    pub fn new(lat: f64, lon: f64, unit_system: String, api_key: String,) -> Self {
        Self {
            lat,
            lon,
            unit_system,
            api_key,
        }
    }
    
    pub fn get_weather(&mut self, callback: Callback<Result<OneCall, Error>>) -> FetchTask {
        let url = format!("https://api.openweathermap.org/data/2.5/onecall?lat={lat}&lon={lon}&units={unit_system}&appid={api_key}",
            api_key=self.api_key,
            lat=self.lat,
            lon=self.lon,
            unit_system=self.unit_system
        );
        let handler = move |response: Response<Json<Result<OneCall, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!(
                    "{}: error getting weather data from OpenWeatherMap",
                    meta.status
                )))
            }
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        FetchService::fetch(request, handler.into()).unwrap()
    }
}