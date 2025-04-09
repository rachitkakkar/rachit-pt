use glam::{DVec3, UVec2};
use image::ImageBuffer;
use std::fmt::Write;
use std::{thread, time};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rand::prelude::*;
use pt::geometries::{Intersection, Object, Ray, Sphere};

fn intersects_world(objects: &Vec<Box<dyn Object>>, ray: &Ray) -> Option<Intersection> {
  let mut closest_object: Option<Intersection> = None;
  let mut closest_distance: f64 = f64::MAX;

  for obj in objects.iter() {
    if let Some(intersection) = obj.intersects(ray, 0.0, closest_distance) {
      closest_distance = intersection.t;
      closest_object = Some(intersection);
    }
  }

  closest_object
}

/* Set-up Scene and render Image */
fn main() {
  println!("rachit-pt: Rendering image to output.png\n");

  // Create Image
  println!("[1/4] 📸 Creating image...");
  thread::sleep(time::Duration::from_millis(800));
  let dimensions: UVec2 = UVec2::new(800, 600);
  let mut buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(dimensions.x, dimensions.y);

  // Scene
  println!("[2/4] 🔧 Constructing scene...");
  thread::sleep(time::Duration::from_millis(800));
  let mut objects: Vec<Box<dyn Object>> = Vec::new();
  objects.push(Box::new(Sphere{ center: DVec3::new(0.0, 0.0, -1.0), radius: 0.5 }));
  objects.push(Box::new(Sphere{ center: DVec3::new(0.0, -100.5, -1.0), radius: 100.0 }));

  // Camera
  let samples: i32 = 100;
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
  let bar = ProgressBar::new(dimensions.y as u64);
  bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})")
      .unwrap()
      .with_key("eta", |state: &ProgressState, w: &mut dyn Write | write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
      .progress_chars("#>-"));
  println!("[3/4] 🖼️ Rendering...");
  for y in 0..dimensions.y {
    for x in 0..dimensions.x {
      // Anti-aliasing
      let mut color: DVec3 = DVec3::new(0.0, 0.0, 0.0);
      for _ in 0..samples {
        let offset: DVec3 = DVec3::new(
          rand::rng().random::<f64>() - 0.5, 
          rand::rng().random::<f64>() - 0.5, 0.0
        );
        let pixel_sample: DVec3 = upper_left 
          + ((x as f64 + offset.x) * delta_u) 
          + ((y as f64 + offset.y) * delta_v);
        let ray: Ray = Ray::new(
          center, 
          pixel_sample  - center
        );

        if let Some(intersection) = intersects_world(&objects, &ray) {
          color += 0.5 * (intersection.normal + DVec3::new(1.0, 1.0, 1.0));
        } else {
          let a: f64 = 0.5 * (ray.direction.normalize().y + 1.0);
          color += (a) * DVec3::new(1.0, 1.0, 1.0) + (1.0-a)*DVec3::new(0.5, 0.7, 1.0);
        }
      }
      color *= 1.0 / samples as f64;

      // Write pixel to image (scale 0-1 to 0-255)
      let r: u8 = (255.99 * color.x).clamp(0.0, 255.0) as u8;
      let g: u8 = (255.99 * color.y).clamp(0.0, 255.0) as u8;
      let b: u8 = (255.99 * color.z).clamp(0.0, 255.0) as u8;

      let pixel = buffer.get_pixel_mut(x, y);
      *pixel = image::Rgb([r, g, b]);
    }

    bar.inc(1);
  }
  bar.finish();

  buffer.save("output.png").unwrap();
}
