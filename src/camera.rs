use vec3::{cross, Vec3};
use ray::Ray;
use std::f64::consts;

pub struct Camera {
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  origin: Vec3
}


impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
    let theta = vfov * consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;

    let origin = lookfrom;
    
    let w = (lookfrom - lookat).unit_vector();
    let u = cross(vup, w).unit_vector();
    let v = cross(w, u);

    let lower_left_corner = origin - half_width * u - half_height * v - w;
    Camera {
      lower_left_corner: lower_left_corner,
      horizontal: 2.0 * half_width * u,
      vertical: 2.0 * half_height * v,
      origin: origin
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    Ray::new(
      self.origin, 
      self.lower_left_corner + 
        s * self.horizontal +
        t * self.vertical -
        self.origin
    )
  }
}
