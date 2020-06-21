use crate::components::button::Button;
use crate::data::geojson::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use rand::rngs::ThreadRng;

const GEOJSON_KEY: &'static str = "geojsonData";
const POSITION_KEY: &'static str = "geoPosition";

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    fn update_geojson();
}

pub enum Msg {
    AddOne,
    RemoveOne,
}

pub struct App {
    link: ComponentLink<Self>,
    counter: i32,
    storage: StorageService,
    geo_data: Vec<Feature>,
    rng: ThreadRng,
    position: Vec<f64>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Watchout! New! Now it returns a Result
        let storage = StorageService::new(Area::Session).expect("storage was disabled by the user");
        let Json(geo_data) = storage.restore(GEOJSON_KEY);
        let geo_data = geo_data.unwrap_or_else(|_| Vec::new());
        let rng = thread_rng();
        let position: yew::format::Text = storage.restore(POSITION_KEY);
        let position = string2vec_f64(position
                        .unwrap_or_else(|_| "0.0".to_string()));
        App {
            link: link,
            counter: 0,
            storage,
            geo_data,
            rng,
            position,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.counter += 1;
                
                let position: Vec<f64> = self.position.into_iter().map(|x: f64| {
                    let d: f64 = self.rng.gen_range(0.00001, 0.0003);
                    if random() {
                        return x-d;
                    }
                    x+d
                }).collect();
                let mut feat = Feature::new();
                let position: Value = position.into();
                let point = Geometry::new_point(position);
                feat.add_property("popupContent".into(), self.counter.to_string().into());
                feat.add_geomerty(Some(point));
                self.geo_data.push(feat);
                self.storage.store(GEOJSON_KEY, Json(&self.geo_data));
                update_geojson();
            }
            Msg::RemoveOne => {
                let _ = self.geo_data.pop();
                self.counter -= if self.counter == 0 { 0 } else { 1 };
                self.storage.store(GEOJSON_KEY, Json(&self.geo_data));
                update_geojson();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Welcome to Yew" }</h1>
                <p>{ self.counter } </p>
                <Button onsignal=self.link.callback(|_| Msg::RemoveOne) title="-1" />
                <Button onsignal=self.link.callback(|_| Msg::AddOne) title="+1" />
            </div>
        }
    }
}

fn string2vec_f64(s: String) -> Vec<f64> {
    s.split(',').map(|s| s.trim())
     .filter(|s| !s.is_empty())
     .map(|s| s.parse().unwrap())
     .collect()
}
