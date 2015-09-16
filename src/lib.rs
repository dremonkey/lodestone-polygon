/// The main crate for lodestone-polygon
///
/// ## Overview
/// 
/// Takes coordinates and returns a new Polygon GeoJson feature.
/// Inspired by [turf-polygon](https://github.com/Turfjs/turf-polygon).

// Standard lib crates
use std::str::FromStr;

// Third party crates
extern crate geojson;
extern crate rustc_serialize;

use rustc_serialize::json::{self, ToJson};
use geojson::{Error, Feature, Geometry, Position, Value, FromObject};

pub struct FeaturePolygon {
  feature: Feature
}

impl FeaturePolygon {
  pub fn new(coordinates: Vec<Vec<Position>>) -> Self {

    assert!(coordinates.len() >= 1);

    for ring in &coordinates {
      assert!(ring.len() >= 4, "Each LinearRing of a Polygon must have 4 or more latlng");
      assert_eq!(ring.first(), ring.last());
    }

    let geometry = Geometry::new(Value::Polygon(coordinates));
    let properties = json::Object::new();

    FeaturePolygon {
      feature: Feature {
        bbox: None,
        crs: None,
        geometry: geometry,
        id: None,
        properties: Some(properties),
      }
    }
  }

  pub fn coordinates(&self) -> &Vec<Vec<Position>> {
    type Err = Error;
    
    match self.feature.geometry.value {
      Value::Polygon(ref coords) => coords,
      _ => unreachable!("Type other than Value::Polygon should not be possible"),
    }
  }
}

impl FromStr for FeaturePolygon {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {

    let decoded_json = match json::Json::from_str(s) {
      Ok(j) => j,
      Err(..) => return Err(Error::new("Encountered malformed JSON")),
    };
    
    let object = match decoded_json {
      json::Json::Object(object) => object,
      _ => return Err(Error::new("Attempted to create GeoJSON from JSON that is not an object")),
    };

    Self::from_object(&object)
  }
}

impl FromObject for FeaturePolygon {
  fn from_object(object: &json::Object) -> Result<Self, Error> {
    let feature = Feature::from_object(object).unwrap();
    Ok(FeaturePolygon {
      feature: feature
    })
  }
}

impl ToJson for FeaturePolygon {
  fn to_json(&self) -> json::Json {
    self.feature.to_json()
  }
}

impl ToString for FeaturePolygon {
  fn to_string(&self) -> String {
    self.to_json().to_string()
  }
}

#[cfg(test)]
mod tests {
  use rustc_serialize::json::{self, ToJson};
  use super::FeaturePolygon;

  #[test]
  fn test_valid_coordinates() {
    let expected_json = "{\"geometry\":{\"coordinates\":[[[1.0,1.0],[2.0,2.0],[2.0,1.0],[1.0,1.0]]],\"type\":\"Polygon\"},\"properties\":{},\"type\":\"Feature\"}";

    let coords = vec![vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![2.0, 1.0], vec![1.0, 1.0]]];
    let polygon = FeaturePolygon::new(coords);
    let polygon_str = json::encode(&polygon.to_json()).unwrap();

    assert_eq!(polygon_str, expected_json);
  }

  #[test]
  #[should_panic(expected = "Each LinearRing of a Polygon must have 4 or more latlng")]
  fn test_invalid_ring_coords_count() {
    let coords = vec![vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![1.0, 1.0]]];
    FeaturePolygon::new(coords);
  }

  #[test]
  #[should_panic(expected = "assertion failed: `(left == right)`")]
  fn test_invalid_ring_coords() {
    let coords = vec![vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![2.0, 1.0], vec![3.0, 3.0]]];
    FeaturePolygon::new(coords);
  }
}