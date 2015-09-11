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
  coordinates: Vec<Vec<Vec<f64>>>) => GeoJson {

  assert!(coordinates.len() >= 1);

  for ring in &coordinates {
    assert!(ring.length >= 4, "Each LinearRing of a Polygon must have 4 or more latlng");
    assert_eq!(ring.first(), ring.last(), "First and last latlng are not equivalent")
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