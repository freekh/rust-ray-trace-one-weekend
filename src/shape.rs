use vec3::{Vec3, dot};
use ray::Ray;

pub struct HitRecord {
  t: f64,
  p: Vec3,
  normal: Vec3,
  origin: Vec3
}

trait Shape { // hitable sucks as name, shape is better...
  fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}


pub struct Sphere {
  radius: f64,
  center: Vec3
}

impl Shape for Sphere {
  fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let oc = ray.origin() - self.center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(oc, ray.direction());
    let c = dot(oc, oc) - self.radius * self.radius;
    let discriminant = b * b - 4.0 * a * c; // "the square part"

    if discriminant > 0.0 { 
      let mut temp = (-b - discriminant.sqrt()) / (2.0 * a);
      if (temp > t_min && temp < t_max) {
        rec.t = temp;
        rec.p = ray.point_at_parameter(temp);
        rec.normal = (rec.p - self.center) / self.radius;
        true
      } else {

        let temp = (-b + discriminant.sqrt()) / (2.0 * a);
        if (temp > t_min && temp < t_max) {
          rec.t = temp;
          rec.p = ray.point_at_parameter(temp);
          rec.normal = (rec.p - self.center) / self.radius;
          true
        } else {
          false
        }
      }
    } else {
      false
    }
  }
}

struct Shapes<S>(Vec<S>) where S : Shape + Sized;

impl<S> Shape for Shapes<S> where S : Shape + Sized {
  fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    false
  }
}
