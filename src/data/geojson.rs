#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeatureCollection {
    pub r#type: String,
    pub features: Vec<Feature>,
}
impl FeatureCollection {
    pub fn new()-> Self {
        FeatureCollection {
            r#type: "FeatureCollection".to_string(),
            features: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    pub r#type: String,
    pub properties: Option<Value>,
    pub geometry: Option<Geometry>,
}
impl Feature {
    pub fn new() -> Self {
        Feature { 
            r#type: "Feature".to_string(),
            properties: None,
            geometry: None, 
        }
    }
    pub fn add_property(&mut self, key: String, value: Value) {
        match &mut self.properties{
            Some(v) => { v.as_object_mut().unwrap().insert(key, value); },
            None => {
                let mut v = Map::new();
                v.insert(key, value);
                let v: Value = v.into();
                self.properties = Some(v);
                }
        };        
    }
    pub fn add_geomerty(&mut self, geometry: Option<Geometry>) {
        self.geometry = geometry; 
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Value,
}
impl Geometry {
    pub fn new() -> Self {
        Geometry { 
            r#type: "".to_string(),
            coordinates: Value::Null,
        }
    }
    pub fn new_point(coordinates: Value) -> Self {
        Geometry { 
            r#type: "Point".to_string(),
            coordinates,
        }
    }
}