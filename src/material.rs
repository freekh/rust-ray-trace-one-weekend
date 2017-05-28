use rand::{random, Closed01};

use vec3::{dot, Vec3};
use ray::Ray;
use shape::HitRecord;

pub trait Material{
  fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

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
  fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    let target = rec.point + rec.normal + random_in_unit_sphere();
    let scattered = Ray::new(rec.point, target - rec.point);
    let attunation = self.albedo;
    Some((attunation, scattered))
  }
}


fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - 2.0 * dot(v, n) * n
}

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
  fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
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

fn refract(v: Vec3, n: Vec3, ni_over_t: f64) -> Option<Vec3> {
  let uv = v.unit_vector();
  let dt = dot(uv, n);
  let discriminant = 1.0 - ni_over_t * ni_over_t * (1.0 - dt * dt);
  if discriminant > 0.0 {
    let refracted = ni_over_t * (uv - n * dt) - n * discriminant.sqrt();
    Some(refracted)
  } else {
    None
  }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0 * r0;
  r0 + (1.0 - r0) * (1.0 - cosine).powf(0.5)
}

pub struct Dielectric {
  ref_idx: f64
}

impl Material for Dielectric {
  fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    let attunation = Vec3::new(1.0, 1.0, 1.0);
    let reflected = reflect(r_in.direction(), rec.normal);
    
    let outward_normal: Vec3;
    let ni_over_t: f64;
    let cosine: f64;
    if dot(r_in.direction(), rec.normal) > 0.00001 {
      outward_normal = -rec.normal;
      ni_over_t = self.ref_idx;
      cosine = self.ref_idx * dot(r_in.direction(), rec.normal) / r_in.direction().length();
    } else {
      outward_normal = rec.normal;
      ni_over_t = 1.0 / self.ref_idx;
      cosine = -dot(r_in.direction(), rec.normal) / r_in.direction().length();
    }
    
    match refract(r_in.direction(), outward_normal, ni_over_t) {
      Some(refracted) => {
        let reflect_prob = schlick(cosine, self.ref_idx);
        let Closed01(rand) = random::<Closed01<f64>>();
        if rand < 1.0 && rand < reflect_prob {
          Some((attunation, Ray::new(rec.point, refracted)))
        } else {
          Some((attunation, Ray::new(rec.point, refracted)))
        }
      }
      None => {
        Some((attunation, Ray::new(rec.point, reflected)))
      }
    }
  }
}

impl Dielectric {
  pub fn new(ri: f64) -> Dielectric {
    Dielectric {
      ref_idx: ri
    }
  }
}
