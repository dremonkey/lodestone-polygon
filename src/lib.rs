/// The main crate for lodestone-polygon
///
/// ## Overview
/// 
/// Takes coordinates and returns a new Polygon GeoJson feature.
/// Inspired by [turf-polygon](https://github.com/Turfjs/turf-polygon).

// Third party packages
extern crate geojson;
extern crate rustc_serialize;

use rustc_serialize::json;
use geojson::{Feature, GeoJson, Geometry, Value};

pub extern fn polygon(
  coordinates: Vec<Vec<Vec<f64>>>) -> GeoJson {

  assert!(coordinates.len() >= 1);

  for ring in &coordinates {
    assert!(ring.len() >= 4, "Each LinearRing of a Polygon must have 4 or more latlng");
    assert_eq!(ring.first(), ring.last());
  }

  let geometry = Geometry::new(Value::Polygon(coordinates));
  let properties = json::Object::new();

  GeoJson::Feature(Feature {
    bbox: None,
    crs: None,
    geometry: geometry,
    id: None,
    properties: Some(properties),
  })
}

#[cfg(test)]
mod tests {
  use rustc_serialize::json::{self, ToJson};
  use super::polygon;

  #[test]
  fn test_valid_coordinates() {
    let expected_json = "{\"geometry\":{\"coordinates\":[[[1.0,1.0],[2.0,2.0],[2.0,1.0],[1.0,1.0]]],\"type\":\"Polygon\"},\"properties\":{},\"type\":\"Feature\"}";

    let coords = vec![vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![2.0, 1.0], vec![1.0, 1.0]]];
    let geojson = polygon(coords);
    let polygon_str = json::encode(&geojson.to_json()).unwrap();

    assert_eq!(polygon_str, expected_json);
  }

  #[test]
  #[should_panic(expected = "Each LinearRing of a Polygon must have 4 or more latlng")]
  fn test_invalid_ring_coords_count() {
    let coords = vec![vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![1.0, 1.0]]];
    polygon(coords);
  }

  #[test]
  #[should_panic(expected = "assertion failed: `(left == right)`")]
  fn test_invalid_ring_coords() {
    let coords = vec![vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![2.0, 1.0], vec![3.0, 3.0]]];
    polygon(coords);
  }
}