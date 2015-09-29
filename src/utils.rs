
use ::FeaturePolygon;
use lodestone_bearing::bearing;
use lodestone_point::FeaturePoint;

/// Utility method to check if the coordinates in the two rings are the same
pub fn compare_rings(
    ring1: &Vec<Vec<f64>>, 
    ring2: &Vec<Vec<f64>>) -> bool {
  
  let mut is_equal = true;

  for (i, coord1) in ring1.iter().enumerate() {
    let coord2 = ring2[i].clone();
    
    // 1.0e-7 is ~11mm precision
    is_equal = (coord1[0] - coord2[0]).abs() < 1e-7 &&
      (coord1[1] - coord2[1]).abs() < 1e-7;

    if !is_equal {
      break;
    }
  }

  is_equal
}

pub fn is_convex(poly: &FeaturePolygon) -> bool {
  for d in inner_angles(&poly) {
    if d > 180.0 { return false; }
  }
  true
}

fn inner_angles(poly: &FeaturePolygon) -> Vec<f64> {  
  
  let mut abs_bearings = vec![];
  let mut angles = vec![];
  let shell = poly.coordinates()[0].to_vec();

  // https://doc.rust-lang.org/book/closures.html#returning-closures
  let identity: Box<Fn(f64) -> f64> = match is_clockwise(&shell) {
    true => Box::new(|x: f64| x),
    false => Box::new(|x: f64| -x)
  };

  // gather the bearings
  for edge in shell.windows(2) {
    let pt1 = FeaturePoint::new(edge[0].to_vec());
    let pt2 = FeaturePoint::new(edge[1].to_vec());
    let bearing = abs_angle(bearing(&pt1, &pt2));

    abs_bearings.push(bearing);
  }

  // copy the last bearing and prepend to the list so we have
  // all the pairs we need to calculate inner_angles.
  let last = abs_bearings.last().unwrap().clone();
  abs_bearings.insert(0, last);

  for pair in abs_bearings.windows(2) {
    let b1 = &pair[0];
    let b2 = &pair[1];
    let inner_angle = abs_angle(180.0 - identity(b2 - b1));
    angles.push(inner_angle);
  }

  angles
}

/// Convert from -180° ... 180° to 0° ... 360°
fn abs_angle(x: f64) -> f64 {
  let mut x = x % 360.0;
  if x < 0.0 { x = x + 360.0 };
  x
}

fn is_clockwise(ring: &Vec<Vec<f64>>) -> bool {
  let mut sum = 0.0;  
  for edge in ring.windows(2) {
    let pt1 = &edge[0];
    let pt2 = &edge[1];
    sum += (pt2[0] - pt1[0]) * (pt2[1] + pt1[1]);
  }

  sum > 0.0
}

#[cfg(test)]
mod tests {
  use ::FeaturePolygon;
  use super::{abs_angle, inner_angles, is_clockwise, is_convex};

  #[test]
  fn test_abs_angle() {
    let x1 = 30.0;
    let x2 = -30.0;
    let x3 = 710.0;
    let x4 = -180.0;

    assert_eq!(abs_angle(x1), 30.0);
    assert_eq!(abs_angle(x2), 330.0);
    assert_eq!(abs_angle(x3), 350.0);
    assert_eq!(abs_angle(x4), 180.0);
  }

  #[test]
  fn test_inner_angles() {
    let ring_cw = vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0], vec![1.0, 0.0], vec![0.0, 0.0]];
    let poly = FeaturePolygon::new(vec![ring_cw]);

    for angle in inner_angles(&poly) {
      // angles will be close but not exactly 90 because
      // we are on a sphere
      assert_eq!(angle - 90.0 < 1e-2, true);
    }
  }

  #[test]
  fn test_is_clockwise() {
    let ring_cw = vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0], vec![1.0, 0.0], vec![0.0, 0.0]];
    let ring_ccw = vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![1.0, 1.0], vec![0.0, 1.0], vec![0.0, 0.0]];

    assert_eq!(is_clockwise(&ring_cw), true);
    assert_eq!(is_clockwise(&ring_ccw), false);
  }

  #[test]
  fn test_is_convex() {
    let convex = vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0], vec![1.0, 0.0], vec![0.0, 0.0]];
    let concave = vec![vec![-1.0, -1.0], vec![3.0, 3.0], vec![2.0, 0.0], vec![5.0, -1.0], vec![-1.0, -1.0]];
    let concave_poly = FeaturePolygon::new(vec![concave]);
    let convex_poly = FeaturePolygon::new(vec![convex]);

    assert_eq!(is_convex(&concave_poly), false);
    assert_eq!(is_convex(&convex_poly), true);
  }
}