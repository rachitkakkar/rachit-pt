use glam::{DVec3, UVec2};
use std::{thread, time, fmt::Write};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rand::prelude::*;
use image::ImageBuffer;
use pt::geometries::{Intersection, Object, Ray, Sphere};
use pt::materials;

// Get the color illuminated by a particular ray given a scene
fn intersects_world(objects: &Vec<Box<dyn Object>>, ray: &Ray, depth: i32) -> DVec3 {
  if depth <= 0 {
    return DVec3::new(0.0, 0.0, 0.0);
  }

  let mut closest_intersection: Option<Intersection> = None;
  let mut closest_distance: f64 = f64::MAX;

  for obj in objects.iter() {
    if let Some(intersection) = obj.intersects(ray, 0.001, closest_distance) {
      closest_distance = intersection.t;
      closest_intersection = Some(intersection);
    }
  }

  let mut color: DVec3 = DVec3::new(0.0, 0.0, 0.0);
  if closest_intersection.is_some() {
    let intersection: Intersection = closest_intersection.unwrap();
    // let direction: DVec3 = materials::random_hemisphere_vector(intersection.normal);
    let direction: DVec3 = intersection.normal + materials::random_unit_vector();
    color += 0.5 * (intersects_world(objects, &Ray::new(intersection.location, direction), depth-1));
  } else {
    let a: f64 = 0.5 * (ray.direction.normalize().y + 1.0);
    color += (a) * DVec3::new(1.0, 1.0, 1.0) + (1.0-a)*DVec3::new(0.5, 0.7, 1.0);
  }

  color
}

/* Set-up Scene and render Image */
fn main() {
  println!("rachit-pt: Rendering image to output.png\n");

  // Create Image
  println!("[1/4] 📸 Creating image...");
  thread::sleep(time::Duration::from_millis(rand::rng().random::<u64>() % 1000));
  let dimensions: UVec2 = UVec2::new(800, 600);
  let mut buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(dimensions.x, dimensions.y);

  // Scene
  println!("[2/4] 🔧 Constructing scene...");
  thread::sleep(time::Duration::from_millis(rand::rng().random::<u64>() % 1000));
  let mut objects: Vec<Box<dyn Object>> = Vec::new();
  objects.push(Box::new(Sphere{ center: DVec3::new(0.0, 0.0, -1.0), radius: 0.5 }));
  objects.push(Box::new(Sphere{ center: DVec3::new(0.0, -100.5, -1.0), radius: 100.0 }));

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

  // Set-up rendering settings
  let samples: i32 = 100;
  let max_bounces: i32 = 50;

  // Write an image
  let bar = ProgressBar::new(dimensions.y as u64);
  bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})")
      .unwrap()
      .with_key("eta", |state: &ProgressState, w: &mut dyn Write | write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
      .progress_chars("#>-"));
  println!("[3/4] 🖼️  Rendering...");
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
        
        color += intersects_world(&objects, &ray, max_bounces);
      }
      color *= 1.0 / samples as f64;

      // Write pixel to image (scale 0-1 to 0-255 and convert to linear to gamma 2)
      let r: u8 = (255.99 * (color.x).sqrt()).clamp(0.0, 255.0) as u8;
      let g: u8 = (255.99 * (color.y).sqrt()).clamp(0.0, 255.0) as u8;
      let b: u8 = (255.99 * (color.z).sqrt()).clamp(0.0, 255.0) as u8;

      let pixel = buffer.get_pixel_mut(x, y);
      *pixel = image::Rgb([r, g, b]);
    }

    bar.inc(1);
  }
  bar.finish();

  buffer.save("output.png").unwrap();
}
