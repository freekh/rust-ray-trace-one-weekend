use vec3::Vec3;

pub trait Material {
  fn scatter() -> bool;
}

pub struct Lambertian {
  pub albedo: Vec3 // NOT PUB
}

impl Material for Lambertian {
  fn scatter() -> bool {
    false
  }
}
