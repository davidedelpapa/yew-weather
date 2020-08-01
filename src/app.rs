use crate::components::button::Button;
use crate::data::geojson::*;
use crate::data::onecall::OneCall;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::{StorageService, console::ConsoleService};
use yew::services::fetch::FetchTask;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use load_dotenv::load_dotenv;
use anyhow::Error;
use crate::fetchweather::WeatherService;

const GEOJSON_KEY: &'static str = "geojsonData";
load_dotenv!();

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    fn update_map();
}

pub enum Msg {
    AddOne,
    RemoveOne,
    WeatherReady(Result<OneCall, Error>),
}

pub struct App {
    link: ComponentLink<Self>,
    counter: i32,
    storage: StorageService,
    geo_data: Vec<Feature>,
    position: Vec<f64>,
    rng: ThreadRng,
    //console: ConsoleService,
    weather_service: WeatherService,
    callback: Callback<Result<OneCall, Error>>,
    task: Option<FetchTask>,
    weather: Option<OneCall>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Session).expect("storage was disabled by the user");
        let Json(geo_data) = storage.restore(GEOJSON_KEY);
        let geo_data = geo_data.unwrap_or_else(|_| Vec::new());

        let rng = thread_rng();
        
        let lat = env!("LATITUDE","Cound not find LATITUDE in .env");
        let lng = env!("LONGITUDE", "Cound not find LONGITUDE in .env");
        let lat: f64 = str2f64(lat);
        let lng: f64 = str2f64(lng);
        let position = vec!(lng, lat);
        let weather_key=env!("WEATHER_KEY","Cound not find WEATHER_KEY in .env").to_string();
        App {
            link: link.clone(),
            counter: 0,
            storage,
            geo_data,
            position,
            rng,
            //console: ConsoleService::new(),
            weather_service: WeatherService::new(lat, lng, "metric".to_string(), weather_key),
            callback: link.callback(Msg::WeatherReady),
            weather: None,
            task: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let task = self
                .weather_service
                .get_weather(self.callback.clone());
            self.task = Some(task);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.counter += 1;

                let position: Vec<f64> = self.position.clone().into_iter()
                    .map(|x: f64| {
                        let d: f64 = self.rng.gen_range(0.00001, 0.0003);
                        if random() {
                            return x-d;
                        }
                        x+d
                    }).collect();
                let position: Value = position.into();
                let point = Geometry::new_point(position);

                let mut feat = Feature::new();
                feat.add_geomerty(Some(point));
                feat.add_property("popupContent".into(), self.counter.to_string().into());
                self.geo_data.push(feat);
                
                self.storage.store(GEOJSON_KEY, Json(&self.geo_data));
                update_map();
            }
            Msg::RemoveOne => {
                self.counter -= if self.counter == 0 { 0 } else { 1 };

                let _ = self.geo_data.pop();

                self.storage.store(GEOJSON_KEY, Json(&self.geo_data));
                update_map();
            }
            Msg::WeatherReady(Ok(weather)) => {
                self.weather = Some(weather);
                ConsoleService::log(format!("Weather info: {:?}", self.weather).as_str());
                return false;
            }
            Msg::WeatherReady(Err(e)) => {
                ConsoleService::error(format!("Error: {}, while retrieving weather info", e).as_str());
                return false;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Button onsignal=self.link.callback(|_| Msg::RemoveOne) title="-1" />
                <Button onsignal=self.link.callback(|_| Msg::AddOne) title="+1" />
            </>
        }
    }
}

fn str2f64(s: &str) -> f64 {
    s.trim()
     .parse()
     .expect("Failed parsing a String to f64")
}