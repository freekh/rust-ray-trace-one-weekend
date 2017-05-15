use vec3::Vec3;

pub trait Material : Copy { // TODO: it would be better to force borrowing on material in Shape - we don't really want to copy here...
  fn scatter() -> bool;
}

#[derive(Copy,Clone)]
pub struct Lambertian {
  pub albedo: Vec3 // NOT PUB
}

impl Material for Lambertian {
  fn scatter() -> bool {
    false
  }
}
