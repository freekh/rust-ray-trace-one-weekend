use rand::{random, Closed01};

use vec3::{dot, Vec3};
use ray::Ray;
use shape::HitRecord;

pub trait Material : Copy { // TODO: it would be better to force borrowing on material in Shape - we don't really want to copy here...
  fn scatter(&self, r_in: Ray, rec: &HitRecord<Self>) -> Option<(Vec3, Ray)>;
}

#[derive(Copy,Clone)]
pub struct Lambertian {
  albedo: Vec3
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Lambertian {
    Lambertian {
      albedo: albedo
    }
  }
}


fn random_in_unit_sphere() -> Vec3 {
  let mut p: Vec3;
  while {
    let Closed01(x_rand) = random::<Closed01<f64>>();
    let Closed01(y_rand) = random::<Closed01<f64>>();
    let Closed01(z_rand) = random::<Closed01<f64>>();
    p = 2.0 * Vec3::new(
      x_rand, y_rand, z_rand
    ) - Vec3::new(1.0, 1.0, 1.0);
    p.squared_length() >= 1.0
  } {}
  p
}

impl Material for Lambertian {
  fn scatter(&self, r_in: Ray, rec: &HitRecord<Self>) -> Option<(Vec3, Ray)> {
    let target = rec.point + rec.normal + random_in_unit_sphere();
    let scattered = Ray::new(rec.point, target - rec.point);
    let attunation = self.albedo;
    Some((attunation, scattered))
  }
}


fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - 2.0 * dot(v, n) * n
}

#[derive(Copy,Clone)]
pub struct Metal {
 albedo: Vec3
}

impl Metal {
  pub fn new(albedo: Vec3) -> Metal {
    Metal {
      albedo: albedo
    }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: Ray, rec: &HitRecord<Self>) -> Option<(Vec3, Ray)> {
    let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
    let scattered = Ray::new(rec.point, reflected);
    let attunation = self.albedo;
    if dot(scattered.direction(), rec.normal) > 0.0 {
      Some((attunation, scattered))
    } else {
      None
    } 
  }
}
