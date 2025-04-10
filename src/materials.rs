use rand::prelude::*;
use glam::DVec3;

pub fn random_unit_vector() -> DVec3 {
  let mut rng = rand::rng();
  let x: f64 = rng.random_range(-1.0..1.0);
  let y: f64 = rng.random_range(-1.0..1.0);
  let z: f64 = rng.random_range(-1.0..1.0);
  
  DVec3::new(x, y, z)
}

// Generating a random vector on a hemisphere using the rejection method
pub fn random_hemisphere_vector(normal: DVec3) -> DVec3 {
  let on_unit_sphere: DVec3;
  loop {
    let vector: DVec3 = random_unit_vector();

    // If the point is inside the unit sphere, accept it
    if vector.length_squared() <= 1.0 {
      on_unit_sphere = vector.normalize(); // Return the normalized unit vector
      break;
    }
  }
  if on_unit_sphere.dot(normal) > 0.0 {
    return on_unit_sphere;
  }
  -on_unit_sphere
}