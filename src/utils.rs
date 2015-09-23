
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