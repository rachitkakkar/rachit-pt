use glam::{DVec3, UVec2};
use std::{thread, time, fmt::Write};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use pt::{geometries::{Intersection, Object, Ray, Sphere}, materials::{Dielectric, Lambertian, Metal}};
use rand::prelude::*;
use image::ImageBuffer;

fn generate_random_scene() -> Vec<Box<dyn Object>> {
  let mut objects: Vec<Box<dyn Object>> = Vec::new();

  // Ground material (Lambertian)
  let ground_material: Lambertian = Lambertian::new(DVec3::new(0.5, 0.5, 0.5));
  objects.push(Box::new(Sphere::new(ground_material, DVec3::new(0.0, -1000.0, 0.0), 1000.0)));

  // Adding random spheres to the scene
  for a in -11..11 {
    for b in -11..11 {
      let choose_mat: f64 = rand::random::<f64>();  // Equivalent to random_double()
      let center: DVec3 = DVec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());

      if (center - DVec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          // Lambertian material (diffuse)
          let albedo: DVec3 = DVec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) * DVec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>());
          objects.push(Box::new(Sphere::new(Lambertian::new(albedo), center, 0.2)));

        } else if choose_mat < 0.95 {
          // Metal material
          let albedo: DVec3 = DVec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) * DVec3::new(0.5, 1.0, 0.5);
          let fuzz: f64 = rand::random::<f64>();  // You can control the fuzz range similarly
          objects.push(Box::new(Sphere::new(Metal::new(albedo, fuzz), center, 0.2)));

        } else {
          // Dielectric material (glass)
          objects.push(Box::new(Sphere::new(Dielectric::new(1.5), center, 0.2)));

        }

      }
    }
  }

  // Adding specific spheres to the objects list
  let material1: Dielectric = Dielectric::new(1.5);
  objects.push(Box::new(Sphere::new(material1, DVec3::new(0.0, 1.0, 0.0), 1.0)));

  let material2: Lambertian = Lambertian::new(DVec3::new(0.4, 0.2, 0.1));
  objects.push(Box::new(Sphere::new(material2, DVec3::new(-4.0, 1.0, 0.0), 1.0)));

  let material3: Metal = Metal::new(DVec3::new(0.7, 0.6, 0.5), 0.0);
  objects.push(Box::new(Sphere::new(material3, DVec3::new(4.0, 1.0, 0.0), 1.0)));

  objects
}

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
    if let Some(scatter) = intersection.material.scatter(ray, intersection) {
      let (scattered, attenuation) = scatter;
      color += attenuation * intersects_world(objects, &scattered, depth-1);
    }
  } else {
    let a: f64 = 0.5 * (ray.direction.normalize().y + 1.0);
    color += (1.0-a) * DVec3::new(1.0, 1.0, 1.0) + (a)*DVec3::new(0.5, 0.7, 1.0);
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
  // let mut objects: Vec<Box<dyn Object>> = Vec::new();
  // objects.push(Box::new(Sphere::new(Lambertian::new(DVec3::new(0.1, 0.2, 0.5)), DVec3::new(0.0, 0.0, -1.2), 0.5)));
  // objects.push(Box::new(Sphere::new(Dielectric::new(1.00 / 1.33), DVec3::new(-1.0, 0.0, -1.0), 0.4)));
  // objects.push(Box::new(Sphere::new(Dielectric::new(1.5), DVec3::new(-1.0, 0.0, -1.0), 0.5)));
  // objects.push(Box::new(Sphere::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.2), DVec3::new(1.0, 0.0, -1.0), 0.5)));
  // objects.push(Box::new(Sphere::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)), DVec3::new(0.0, -100.5, -1.0), 100.0)));
  let objects:Vec<Box<dyn Object>> = generate_random_scene();

  // Camera
  let vfov: f64 = 20.0;
  let vup: DVec3 = DVec3::new(0.0, 1.0, 0.0);
  let center: DVec3 = DVec3::new(13.0, 2.0, 3.0);
  let direction: DVec3 = DVec3::new(0.0, 0.0, 0.0);

  let focal_length: f64 = (center - direction).length();
  let viewport_height: f64 = 2.0 * ((vfov * std::f64::consts::PI / 360.0)).tan() * focal_length;
  let viewport_width: f64 = viewport_height * (dimensions.x as f64 / dimensions.y as f64);
  let w: DVec3 = (center - direction).normalize();
  let u: DVec3 = viewport_width * vup.cross(w).normalize(); 
  let v: DVec3 = viewport_height * -w.cross(u / viewport_width);
  let delta_u: DVec3 = u / dimensions.x as f64;
  let delta_v: DVec3 = v / dimensions.y as f64;
  let upper_left: DVec3 = 
    (center - (focal_length * w) - u / 2.0 - v / 2.0) 
    + 0.5 * (delta_u + delta_v);

  // Set-up rendering settings
  let samples: i32 = 1000;
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
      let r: u8 = (255.0 * (color.x).sqrt()).clamp(0.0, 255.0) as u8;
      let g: u8 = (255.0 * (color.y).sqrt()).clamp(0.0, 255.0) as u8;
      let b: u8 = (255.0 * (color.z).sqrt()).clamp(0.0, 255.0) as u8;

      let pixel = buffer.get_pixel_mut(x, y);
      *pixel = image::Rgb([r, g, b]);
    }

    bar.inc(1);
  }
  bar.finish();

  buffer.save("output.png").unwrap();
}
