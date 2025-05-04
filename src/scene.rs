use glam::DVec3;
use crate::{
  geometries::{Object, Sphere},
  materials::{Dielectric, Lambertian, Metal},
};

pub struct Camera {
  pub vfov: f64,
  pub vup: DVec3,
  pub center: DVec3,
  pub direction: DVec3,
}

impl Camera {
  pub fn new(vfov: f64, vup: DVec3, center: DVec3, direction: DVec3) -> Self {
    Camera {
      vfov,
      vup,
      center,
      direction
    }
  }
}

pub struct Scene {
  pub camera: Camera,
  pub objects: Vec<Box<dyn Object>>
}

impl Scene {
  pub fn new(camera: Camera, objects: Vec<Box<dyn Object>>) -> Self {
    Scene { camera, objects }
  }
}

pub fn generate_random_scene() -> Scene {
  let camera: Camera = Camera::new(
    20.0,
    DVec3::new(0.0, 1.0, 0.0),
    DVec3::new(13.0, 2.0, 3.0),
    DVec3::new(0.0, 0.0, 0.0)
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

  Scene::new(camera, objects)
}

// Uhh I don't know what else to name this but this function name is Maxim-approved!!
pub fn generate_scene_of_three_balls_with_different_materials_viewed_from_a_distance() -> Scene {
  let camera: Camera = Camera::new(
    35.0,
    DVec3::new(0.0, 1.0, 0.0),
    DVec3::new(13.0, 2.0, 3.0),
    DVec3::new(0.0, 0.0, 0.0)
  );
  let mut objects: Vec<Box<dyn Object>> = Vec::new();

  objects.push(Box::new(Sphere::new(Lambertian::new(DVec3::new(0.1, 0.2, 0.5)), DVec3::new(0.0, 0.0, -1.2), 0.5)));
  objects.push(Box::new(Sphere::new(Dielectric::new(1.00 / 1.33), DVec3::new(-1.0, 0.0, -1.0), 0.4)));
  objects.push(Box::new(Sphere::new(Dielectric::new(1.5), DVec3::new(-1.0, 0.0, -1.0), 0.5)));
  objects.push(Box::new(Sphere::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.2), DVec3::new(1.0, 0.0, -1.0), 0.5)));
  objects.push(Box::new(Sphere::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)), DVec3::new(0.0, -100.5, -1.0), 100.0)));

  Scene::new(camera, objects)
}