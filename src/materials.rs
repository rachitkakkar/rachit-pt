use rand::prelude::*;
use glam::DVec3;
use crate::geometries::{Intersection, Ray};

pub fn random_unit_vector() -> DVec3 {
  let mut rng = rand::rng();
  let x: f64 = rng.random_range(-1.0..1.0);
  let y: f64 = rng.random_range(-1.0..1.0);
  let z: f64 = rng.random_range(-1.0..1.0);
  
  DVec3::new(x, y, z)
}

// Generate a random direction on the hemisphere with the given normal using the rejection method
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

// Ensure the scattered direction is not degenerate
fn is_degenerate_direction(direction: &DVec3) -> bool {
  direction.length_squared() < 1e-8 // Threshold to catch nearly zero vectors
}

// Reflect a vector around a normal (used for reflection)
fn reflect(v: DVec3, n: DVec3) -> DVec3 {
  v - 2.0 * v.dot(n) * n
}

pub trait Material {
  fn scatter(&self, ray: &Ray, intersection: Intersection) -> Option<(Ray, DVec3)>;
}

pub struct Lambertian {
  pub albedo: DVec3,
}

impl Lambertian {
  pub fn new(albedo: DVec3) -> Lambertian {
    Lambertian { albedo }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _ray: &Ray, intersection: Intersection) -> Option<(Ray, DVec3)> {
    let mut scatter_direction: DVec3 = random_hemisphere_vector(intersection.normal);

    // Check if the generated direction is degenerate
    if is_degenerate_direction(&scatter_direction) {
      // If the direction is degenerate, reattempt the generation
      scatter_direction = intersection.normal;
    }

    // Create the scattered ray
    let scattered_ray: Ray = Ray {
      origin: intersection.location,
      direction: scatter_direction,
    };

    Some((scattered_ray, self.albedo))
  }
}

pub struct Metal {
  pub albedo: DVec3,  // The color of the metal
  pub fuzz: f64,      // The fuzziness factor for roughness
}

impl Metal {
  // Constructor for Metal
  pub fn new(albedo: DVec3, fuzz: f64) -> Metal {
    // Ensure fuzz is between 0.0 and 1.0 for realistic behavior
    let fuzz: f64 = if fuzz < 1.0 { fuzz } else { 1.0 };
    Metal { albedo, fuzz }
  }
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, intersection: Intersection) -> Option<(Ray, DVec3)> {
    // Reflect the ray direction around the normal
    let mut reflected = reflect(ray.direction.normalize(), intersection.normal);
    reflected = reflected.normalize() + (self.fuzz * random_unit_vector());

    // Apply fuzz to the reflection direction
    // let random_offset = random_hemisphere_vector() * self.fuzz;
    // let scattered_direction = reflected + random_offset;

    // Ensure the reflected ray is in the correct direction (not backwards)
    if reflected.dot(intersection.normal) > 0.0 {
      let scattered_ray: Ray = Ray {
        origin: intersection.location,
        direction: reflected,
      };
      return Some((scattered_ray, self.albedo))
    }
    None
  }
}
