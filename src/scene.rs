use std::fs::File;
use glam::DVec3;
use hdrldr::load;

use crate::{
  geometries::{Object, Sphere},
  materials::{Dielectric, Lambertian, Metal},
};

pub struct Camera {
  pub vfov: f64,
  pub vup: DVec3,
  pub center: DVec3,
  pub direction: DVec3,

  pub defocus_angle: f64,
  pub focus_dist: f64,
}

impl Camera {
  pub fn new(vfov: f64, vup: DVec3, center: DVec3, direction: DVec3, defocus_angle: f64, focus_dist: f64) -> Self {
    Camera {
      vfov,
      vup,
      center,
      direction,
      defocus_angle,
      focus_dist,
    }
  }
}
pub struct HDRImage {
  pub width: usize,
  pub height: usize,
  pub data: Vec<f32>, // RGB floats
}

impl HDRImage {
  pub fn load_from_file(path: &str) -> Self {
    let file = File::open(path).expect("Failed to open HDR file");
    let image = load(file).expect("Failed to parse HDR file");

    let mut float_data = Vec::with_capacity(image.data.len() * 3);
    for pixel in image.data {
      float_data.push(pixel.r);
      float_data.push(pixel.g);
      float_data.push(pixel.b);
    }

    HDRImage {
      width: image.width,
      height: image.height,
      data: float_data,
    }
  }
}

impl HDRImage {
  pub fn sample(&self, direction: DVec3) -> DVec3 {
    let dir: DVec3 = direction.normalize();
    let u: f64 = 0.5 + dir.x.atan2(-dir.z) / (2.0 * std::f64::consts::PI);
    let v: f64 = 0.5 - dir.y.asin() / std::f64::consts::PI;

    let u: f64 = u.clamp(0.0, 1.0);
    let v: f64 = v.clamp(0.0, 1.0);

    let x: usize = (u * (self.width as f64 - 1.0)) as usize;
    let y: usize = (v * (self.height as f64 - 1.0)) as usize;

    let idx: usize = (y * self.width + x) * 3;
    let linear: DVec3 = DVec3::new(
      self.data[idx] as f64,
      self.data[idx + 1] as f64,
      self.data[idx + 2] as f64,
    );

    // Gamma correction (sRGB approximation)
    DVec3::new(
      linear.x.powf(1.0 / 2.2),
      linear.y.powf(1.0 / 2.2),
      linear.z.powf(1.0 / 2.2),
    )
  }
}

pub enum Sky {
  Gradient,           // Procedural blue sky
  HDRSkybox(HDRImage) // HDR environment map
}

pub struct Scene {
  pub camera: Camera,
  pub objects: Vec<Box<dyn Object>>,
  pub sky: Sky,
}

impl Scene {
  pub fn new(camera: Camera, objects: Vec<Box<dyn Object>>, sky: Sky) -> Self {
    Scene { camera, objects, sky }
  }

  // Cover image of "Raytracing in One Weekend" + skybox
  pub fn random() -> Self {
    let camera: Camera = Camera::new(
      20.0,
      DVec3::new(0.0, 1.0, 0.0),
      DVec3::new(13.0, 2.0, 3.0),
      DVec3::new(0.0, 0.0, 0.0),
      0.6,
      10.0,
    );
    let mut objects: Vec<Box<dyn Object>> = Vec::new();
  
    // Ground material (Lambertian)
    let ground_material: Lambertian = Lambertian::new(DVec3::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(ground_material, DVec3::new(0.0, -1000.0, 0.0), 1000.0)));
  
    // Adding random spheres to the scene
    for a in -11..11 {
      for b in -11..11 {
        let choose_mat: f64 = rand::random::<f64>();  // Equivalent to random_double()
        let center: DVec3 = 
          DVec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
  
        if (center - DVec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
          if choose_mat < 0.8 {
            // Lambertian material (diffuse)
            let albedo: DVec3 = DVec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) 
                              * DVec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>());
            objects.push(Box::new(Sphere::new(Lambertian::new(albedo), center, 0.2)));
  
          } else if choose_mat < 0.95 {
            // Metal material
            let albedo: DVec3 = DVec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) 
                              * DVec3::new(0.5, 1.0, 0.5);
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

    let hdr_image = HDRImage::load_from_file("assets/evening_road_01_puresky_4k.hdr");
    Scene::new(camera, objects, Sky::HDRSkybox(hdr_image))
  }

  // Three spheres in a row with different materials
  pub fn materials_test() -> Self {
    let camera: Camera = Camera::new(
      90.0,
      DVec3::new(0.0, 1.0, 0.0),
      DVec3::new(-2.0,2.0, 1.0),
      DVec3::new(0.0, 0.0, -1.0),
      10.0,
      3.4
    );
    let mut objects: Vec<Box<dyn Object>> = Vec::new();
  
    objects.push(Box::new(Sphere::new(Lambertian::new(DVec3::new(0.1, 0.2, 0.5)), DVec3::new(0.0, 0.0, -1.2), 0.5)));
    objects.push(Box::new(Sphere::new(Dielectric::new(1.00 / 1.33), DVec3::new(-1.0, 0.0, -1.0), 0.4)));
    objects.push(Box::new(Sphere::new(Dielectric::new(1.5), DVec3::new(-1.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Sphere::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.0), DVec3::new(1.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Sphere::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)), DVec3::new(0.0, -100.5, -1.0), 100.0)));
  
    Scene::new(camera, objects, Sky::Gradient)
  }
}
