use crate::components::image::Image;
use crate::data::geojson::*;
use crate::data::onecall::{OneCall, WeatherDaily};
use crate::data::geodata;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::{StorageService, console::ConsoleService};
use yew::services::fetch::FetchTask;
use serde_json::from_str;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use load_dotenv::load_dotenv;
use anyhow::Error;
use crate::fetchweather::WeatherService;

const GEOJSON_KEY: &'static str = "geojsonData";
const BASE_FEATURES_KEY: &'static str = "basefeatures";
load_dotenv!();

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    fn update_map();
}

pub enum Msg {
    WeatherReady(Result<OneCall, Error>),
}

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    geo_data: Vec<Feature>,
    position: Vec<f64>,
    weather_service: WeatherService,
    callback: Callback<Result<OneCall, Error>>,
    task: Option<FetchTask>,
    weather: Option<OneCall>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut storage = StorageService::new(Area::Session).expect("storage was disabled by the user");
        
        let Json(geo_data) = storage.restore(GEOJSON_KEY);
        let mut geo_data = geo_data.unwrap_or_else(|_| Vec::new());

        let Json(baselayer) = storage.restore(BASE_FEATURES_KEY);
        let baselayer = baselayer.unwrap_or_else(|_| FeatureCollection::new());

        let basic_layer: Result<FeatureCollection, _> = from_str(geodata::BASE_FEATURES);
        match basic_layer {
            Ok(layer) => {
                storage.store(BASE_FEATURES_KEY, Json(&layer));
                update_map();
        },
            _ => { ConsoleService::error("Error loading the base layer"); },
        };
       
        let lat = env!("LATITUDE","Could not find LATITUDE in .env");
        let lng = env!("LONGITUDE", "Could not find LONGITUDE in .env");
        let lat: f64 = str2f64(lat);
        let lng: f64 = str2f64(lng);
        let position = vec!(lng, lat);
        let weather_key=env!("WEATHER_KEY","Could not find WEATHER_KEY in .env").to_string();

        
        App {
            link: link.clone(),
            storage,
            geo_data,
            position,
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
            Msg::WeatherReady(Ok(weather)) => {
                self.weather = Some(weather.clone());
                //ConsoleService::log(format!("Weather info: {:?}", self.weather).as_str());

                //Create a point near the beach
                let pos = vec!(14.08937, 42.585314);
                let point = Geometry::new_point(pos.into());
                let mut feat = Feature::new();
                feat.add_geomerty(Some(point));
                // Extract weather info
                let current_weather = weather.current.unwrap();
                let weather_condition = current_weather.weather[0].as_ref();
                let weather_description = weather_condition.unwrap().description.as_ref();
                // Add on map with an info icon
                feat.add_property("popupContent".into(), weather_description.unwrap().as_str().into());
                feat.add_property("markerIcon".into(), "information".into());
                // Pass it over the fence
                self.geo_data.insert(0, feat);
                self.storage.store(GEOJSON_KEY, Json(&self.geo_data));
                // Update the map
                update_map();
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
        let render_icon = |daily: &WeatherDaily| {
            let daily_condition = daily.weather[0].as_ref();
            match daily_condition {
                Some(daily_condition)=> {
                    let weather_description = match daily_condition.description.as_ref() {
                        Some(description) => description.to_owned(),
                        None => {
                            let ret = "".to_string();
                            ret
                        }
                    };
                    let weather_icon = match daily_condition.icon.as_ref() {
                        Some(icon) => format!("http://openweathermap.org/img/wn/{}.png", icon),
                        None => {
                            let ret = "".to_string();
                            ret
                        }
                    };
                    //ConsoleService::log(format!("Weather description: {:?}", &weather_description).as_str());
                    html! {
                        <div class="column">
                            <Image img=&weather_icon caption=&weather_description />
                        </div>
                    }
                },
                None => html! { <div> </div> }
            }
        };
        let weather_data=self.weather.as_ref();
        match weather_data {
            Some(weather) => {
                let daily_weather = weather.daily.as_ref().unwrap();
                html! {
                    <div>
                        <div class="container">
                            <div class="row">
                                {for daily_weather.iter().take(3).map(render_icon)}
                            </div>
                        </div>
                    </div>
                }
            }
            None => html! {
                <div>
                    { "Loading Weather data..."}
                </div>
            }
        }
    }
}

fn str2f64(s: &str) -> f64 {
    s.trim()
     .parse()
     .expect("Failed parsing a String to f64")
}