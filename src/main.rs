use glam::{DVec3, UVec2};
use image::ImageBuffer;
use pt::geometries::{Intersection, Object, Ray, Sphere};

fn intersects_world(objects: &Vec<Box<dyn Object>>, ray: &Ray) -> bool {
  let mut closest_object: Option<Intersection> = None;
  let mut closest_distance: f64 = f64::MAX;

  for obj in objects.iter() {
    if let Some(intersection) = obj.intersects(ray, -1.0, closest_distance) {
      closest_distance = intersection.t;
      closest_object = Some(intersection);
    }
  }

  closest_object.is_some()
}

/* Set-up Scene and render Image */
fn main() {
  // Create Image
  let dimensions: UVec2 = UVec2::new(800, 600);
  let mut buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(dimensions.x, dimensions.y);

  // Scene
  let mut objects: Vec<Box<dyn Object>> = Vec::new();
  objects.push(Box::new(Sphere{ center: DVec3::new(0.0, 0.0, -1.0), radius: 0.5 }));
  
  // Camera
  let focal_length: f64 = 1.0;
  let center: DVec3 = DVec3::new(0.0, 0.0, 0.0);
  let u: DVec3 = DVec3::new(2.0 * (dimensions.x as f64 / dimensions.y as f64), 0.0, 0.0);
  let v: DVec3 = DVec3::new(0.0, -2.0, 0.0);
  let delta_u: DVec3 = u / dimensions.x as f64;
  let delta_v: DVec3 = v / dimensions.y as f64;
  let upper_left: DVec3 = 
    (center - DVec3::new(0.0, 0.0, focal_length) - u/2.0 - v/2.0) 
    + 0.5 * (delta_u + delta_v);

  // Write an image
  for y in 0..dimensions.y {
    for x in 0..dimensions.x {
      let ray: Ray = Ray::new(
        center, 
        center - (upper_left + (x as f64 * delta_u) + (y as f64 * delta_v))
      );
      
      let color: DVec3;
      if intersects_world(&objects, &ray) {
        color = DVec3::new(255.0, 0.0, 0.0);
      } else {
        let a: f64 = 0.5 * (ray.direction.normalize().y + 1.0);
        color = (a)*DVec3::new(1.0, 1.0, 1.0) + (1.0-a)*DVec3::new(0.5, 0.7, 1.0);
      }

      // Write pixel to image
      let r: u8 = (255.99 * color.x).clamp(0.0, 255.0) as u8;
      let g: u8 = (255.99 * color.y).clamp(0.0, 255.0) as u8;
      let b: u8 = (255.99 * color.z).clamp(0.0, 255.0) as u8;

      let pixel = buffer.get_pixel_mut(x, y);
      *pixel = image::Rgb([r, g, b]);
    }
  }

  buffer.save("output.png").unwrap();
}
