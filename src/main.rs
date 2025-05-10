// External crates
use std::fmt::Write;
use glam::{DVec3, UVec2};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rand::prelude::*;
use image::ImageBuffer;

// Project modules
use pt::{ 
  geometries::{Intersection, Ray},
  scene::{Camera, Scene, Sky},
};

pub fn random_unit_disk_vector() -> DVec3 {
  let mut rng = rand::rng();
  loop {
    let in_unit_disk = DVec3::new(
      rng.random_range(-1.0..1.0),
      rng.random_range(-1.0..1.0),
      0.0
    );

    if in_unit_disk.length_squared() < 1.0 {
      return in_unit_disk;
    }
  }
}

fn cast_ray(scene: &Scene, ray: &Ray, depth: i32) -> DVec3 {
  if depth <= 0 {
    return DVec3::new(0.0, 0.0, 0.0);
  }

  let mut closest_intersection: Option<Intersection> = None;
  let mut closest_distance: f64 = f64::MAX;

  for obj in scene.objects.iter() {
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
      color += attenuation * cast_ray(scene, &scattered, depth-1);
    }
  } else {
    // Sky color (using skybox or procedural gradient)
    match &scene.sky {
      Sky::Gradient => {
        let a: f64 = 0.5 * (ray.direction.normalize().y + 1.0);
        color += (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + (a) * DVec3::new(0.5, 0.7, 1.0);
      }
      Sky::HDRSkybox(hdr_image ) => { color += hdr_image.sample(ray.direction); }
    }
  }

  color
}

// Set-up Scene and render Image
fn main() {
  println!("rachit-pt: Rendering image to output.png\n");

  // Create Image
  println!("[1/4] 📸 Creating image...");
  let dimensions: UVec2 = UVec2::new(800, 600);
  let mut buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> = 
    ImageBuffer::new(dimensions.x, dimensions.y);

  // Scene
  println!("[2/4] 🔧 Constructing scene...");
  let scene: Scene = Scene::random();

  // Camera
  let camera: &Camera = &scene.camera;
  let viewport_height: f64 = 2.0 * ((camera.vfov * std::f64::consts::PI / 360.0)).tan() * camera.focus_dist;
  let viewport_width: f64 = viewport_height * (dimensions.x as f64 / dimensions.y as f64);
  let w: DVec3 = (camera.center - camera.direction).normalize();
  let u: DVec3 = viewport_width * camera.vup.cross(w).normalize(); 
  let v: DVec3 = viewport_height * -w.cross(u / viewport_width);
  let delta_u: DVec3 = u / dimensions.x as f64;
  let delta_v: DVec3 = v / dimensions.y as f64;
  let upper_left: DVec3 = 
    (camera.center - (camera.focus_dist * w) - u / 2.0 - v / 2.0)
    + 0.5 * (delta_u + delta_v);
  let defocus_radius: f64 = camera.focus_dist * ((camera.defocus_angle / 2.0) * std::f64::consts::PI / 360.0).tan();
  let defocus_disk_u: DVec3 = u * defocus_radius;
  let defocus_disk_v: DVec3 = v * defocus_radius;

  // Set-up rendering settings
  let samples: i32 = 10;
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
          if camera.defocus_angle <= 0.0 {
            camera.center
          } else {
            let in_unit_disk: DVec3 = random_unit_disk_vector();
            camera.center + (in_unit_disk.x * defocus_disk_u) + (in_unit_disk.y * defocus_disk_v)
          }, 
          pixel_sample - camera.center
        );
        
        color += cast_ray(&scene, &ray, max_bounces);
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

  println!("[4/4] 📝 Writing image...");
  buffer.save("output.png").unwrap();
 
  println!("[---] ✅ Done!");
}
