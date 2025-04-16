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

fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
  let cos_theta: f64 = (-uv).dot(n).min(1.0);
  let r_out_perp: DVec3 = etai_over_etat * (uv + cos_theta * n);
  let r_out_parallel_sq: f64 = (1.0 - r_out_perp.length_squared()).abs();
  let r_out_parallel: DVec3 = -(r_out_parallel_sq.sqrt()) * n;
  r_out_perp + r_out_parallel 
}

fn schlick(cosine: f64, refr_index: f64) -> f64 {
  let r0: f64 = ((1.0 - refr_index) / (1.0 + refr_index)).powi(2);
  r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
    // To-do: Refactor to use new
    let scattered_ray: Ray = Ray::new(
      intersection.location,
      scatter_direction,
    );

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

    // Ensure the reflected ray is in the correct direction (not backwards)
    if reflected.dot(intersection.normal) > 0.0 {
      let scattered_ray: Ray = Ray::new(
        intersection.location,
        reflected,
      );
      return Some((scattered_ray, self.albedo))
    }
    None
  }
}

pub struct Dielectric {
  pub refractive_index: f64, // Index of refraction (e.g., 1.5 for glass)
}

impl Dielectric {
  pub fn new(refractive_index: f64) -> Dielectric {
    Dielectric { refractive_index }
  }
}

impl Material for Dielectric {
  fn scatter(&self, ray: &Ray, intersection: Intersection) -> Option<(Ray, DVec3)> {
    let attenuation: DVec3 = DVec3::splat(1.0); // No attenuation for dielectric (purely transparent)
    let unit_direction: DVec3 = ray.direction.normalize();
    let cos_theta: f64 = (-unit_direction).dot(intersection.normal).min(1.0);
    let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
    let (ni_over_nt, cosine) = if intersection.front_face {
        // Ray is outside the material
        (
          1.0 / self.refractive_index,
          cos_theta,
        )
      } else {
        // Ray is inside the material
        (
          self.refractive_index,
          self.refractive_index * cos_theta,
        )
      };

    let cannot_refract: bool = ni_over_nt * sin_theta > 1.0;
    let reflect_prob: f64 = if cannot_refract {
      1.0
    } else {
      schlick(cosine, self.refractive_index)
    };
    let direction: DVec3 = if rand::random::<f64>() < reflect_prob {
      reflect(unit_direction, intersection.normal)
    } else {
      refract(unit_direction, intersection.normal, ni_over_nt)
    };

    let scattered_ray: Ray = Ray::new(
      intersection.location,
      direction,
    );
    return Some((scattered_ray, attenuation));
  }
}