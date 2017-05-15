use vec3::{Vec3, dot};
use ray::Ray;
use material::Material;

pub struct HitRecord<M> where M: Material + Sized {
  pub t: f64,
  pub point: Vec3,
  pub normal: Vec3,
  pub material: M
}

pub trait Shape { // hitable sucks as name, shape is better...
  type M: Material;
  fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord<Self::M>>;
}


pub struct Sphere<Mat> where Mat: Material {
  center: Vec3,
  radius: f64,
  material: Mat
}

impl<Mat> Sphere<Mat> where Mat: Material  {
  pub fn new(center: Vec3, radius: f64, material: Mat) -> Sphere<Mat> {
    Sphere {
      center: center,
      radius: radius,
      material: material
    }
  }
}

impl<Mat> Shape for Sphere<Mat> where Mat: Material {
  type M = Mat;


  fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord<Self::M>> {
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
          point: p,
          normal: (p - self.center) / self.radius,
          material: self.material
        })
      } else {
        let temp = (-b + discriminant.sqrt()) / (2.0 * a);
        if temp > t_min && temp < t_max {
          let p = ray.point_at_parameter(temp);
          Some(HitRecord {
            t: temp,
            point: p,
            normal: (p - self.center) / self.radius,
            material: self.material
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
  type M = S::M;

  fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord<Self::M>> {
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
