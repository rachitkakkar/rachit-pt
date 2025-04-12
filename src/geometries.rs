use glam::DVec3;

use crate::materials::Material;

/* Structure to represent a Ray */
pub struct Ray {
  pub origin: DVec3,
  pub direction: DVec3,
}

impl Ray {
  pub fn new(origin: DVec3, direction: DVec3) -> Ray {
    Ray { origin, direction }
  }

  pub fn at(&self, t: f64) -> DVec3 {
    self.origin + self.direction * t
  }
}

/* Structure to represent an Intersection */
pub struct Intersection<'a> {
  pub location: DVec3,
  pub normal: DVec3,
  pub t: f64,
  pub material: &'a dyn Material,
  pub front_face: bool,
}

/* Structure(s) to represent an Objects */
pub trait Object {
  fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub struct Sphere<M: Material> {
  pub material: M,
  pub center: DVec3,
  pub radius: f64
}

impl<M: Material> Sphere<M> {
  pub fn new(material: M, center: DVec3, radius: f64) -> Sphere<M> {
    Sphere { material, center, radius }
  }
}

impl<M: Material> Object for Sphere<M> {
  fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
    let oc: DVec3 = self.center - ray.origin;
    let a: f64 = ray.direction.dot(ray.direction);
    let h: f64 = ray.direction.dot(oc);
    let c: f64 = oc.dot(oc) - self.radius * self.radius;
    
    let discriminant: f64 = h * h - a * c;
    if discriminant < 0.0 {
      return None;
    }

    let sqrtd: f64 = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range.
    let mut root: f64 = (h - sqrtd) / a;
    if root <= t_min || t_max <= root {
      root = (h + sqrtd) / a;
      if root <= t_min || t_max <= root {
        return None;
      }
    }

    let t: f64 = root;
    let p: DVec3 = ray.at(t);
    let outward_normal: DVec3 = (p - self.center) / self.radius;
    let front_face: bool = ray.direction.dot(outward_normal) < 0.0;
    let normal: DVec3 = if front_face { 
      outward_normal
    } else { 
      -outward_normal 
    };

    Some( Intersection { 
      location: p, normal: normal, t: t, material: &self.material, front_face: front_face
    })
  }
}