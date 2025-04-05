use glam::DVec3;

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
pub struct Intersection {
  pub occured: bool,
  pub location: Option<DVec3>,
  pub normal: Option<DVec3>,
  pub t: Option<f64>
}

/* Structure(s) to represent an Objects */
pub struct Object {

}