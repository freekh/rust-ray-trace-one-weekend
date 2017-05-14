use vec3::{Vec3, dot};
use ray::Ray;

pub struct HitRecord {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3
}

pub trait Shape { // hitable sucks as name, shape is better...
  fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub struct Sphere {
  center: Vec3,
  radius: f64
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64) -> Sphere {
    Sphere {
      center: center,
      radius: radius
    }
  }
}

impl Shape for Sphere {
  fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = ray.origin() - self.center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(oc, ray.direction());
    let c = dot(oc, oc) - self.radius * self.radius;
    let discriminant = b * b - 4.0 * a * c; // "the square part"

    if discriminant > 0.0 { 
      let temp = (-b - discriminant.sqrt()) / (2.0 * a);
      if temp > t_min && temp < t_max {
        let p = ray.point_at_parameter(temp);
        Some(HitRecord {
          t: temp,
          p: p,
          normal: (p - self.center) / self.radius
        })
      } else {

        let temp = (-b + discriminant.sqrt()) / (2.0 * a);
        if temp > t_min && temp < t_max {
          let p = ray.point_at_parameter(temp);
          Some(HitRecord {
            t: temp,
            p: p,
            normal: (p - self.center) / self.radius
          })
        } else {
          None
        }
      }
    } else {
      None
    }
  }
}

pub struct Shapes<S>(pub Vec<S>) where S : Shape + Sized;

impl<S> Shape for Shapes<S> where S : Shape + Sized {
  fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let &Shapes(ref inner) = self;
    inner
      .iter()
      .fold(None, |acc, curr| match acc {
        Some(acc_rec) => {
          let potential_hit = curr.hit(ray, t_min, acc_rec.t);
          if potential_hit.is_some() {
            potential_hit
            } else {
            Some(acc_rec)
          }
        }
        None => curr.hit(ray, t_min, t_max)
      })
  }
}
