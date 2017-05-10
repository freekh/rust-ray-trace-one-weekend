use vec3::Vec3;

#[derive(Copy,Clone)]
pub struct Ray {
  a: Vec3,
  b: Vec3
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Ray {
    Ray {
      a: origin,
      b: direction
    }
  }

  pub fn origin(&self) -> Vec3 {
    self.a
  }

  pub fn direction(&self) -> Vec3 {
    self.b
  }

  pub fn point_at_parameter(&self, t: f64) -> Vec3 {
    self.a + t * self.b
  }
}
