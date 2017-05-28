use vec3::{cross, dot, Vec3};
use ray::Ray;
use std::f64::consts;
use rand::{random, Closed01};

pub struct Camera {
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  origin: Vec3,
  lens_radius: f64,
  u: Vec3,
  v: Vec3,
  w: Vec3
}

fn random_in_unit_disk() -> Vec3 {
  let mut p: Vec3;
  loop {
    let Closed01(x_rand) = random::<Closed01<f64>>();
    let Closed01(y_rand) = random::<Closed01<f64>>();
    if x_rand < 1.0 && y_rand < 1.0 {
      p = 2.0 * Vec3::new(x_rand, y_rand, 0.0) - Vec3::new(1.0, 1.0, 0.0);
      if dot(p, p) < 1.0 {
        break;
      }
    }
  }
  p
}

impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
    let theta = vfov * consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;

    let origin = lookfrom;
    
    let w = (lookfrom - lookat).unit_vector();
    let u = cross(vup, w).unit_vector();
    let v = cross(w, u);

    let lower_left_corner = origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
    Camera {
      u: u, v: v, w: w,
      lens_radius: aperture / 2.0,
      lower_left_corner: lower_left_corner,
      horizontal: 2.0 * half_width * focus_dist * u,
      vertical: 2.0 * half_height * focus_dist * v,
      origin: origin
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let rd = self.lens_radius * Vec3::new(-0.5, -0.5, 0.0); // random_in_unit_disk();
    let offset = self.u * (*rd.x()) + self.v * (*rd.y());
    Ray::new(
      self.origin, 
      self.lower_left_corner + 
        s * self.horizontal +
        t * self.vertical -
        self.origin - 
        offset
    )
  }
}
