use std::ops::*;
use std::fmt;

/* This struct is completly unecessary of course, however following the book here so... */
#[derive(Copy,Clone)]
pub struct Vec3 {
  e: [f64; 3]
}

impl Vec3 {
  pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
    Vec3 {
      e: [e0, e1, e2]
    }
  }

  pub fn x(&self) -> &f64 { &self.e[0] }
  pub fn y(&self) -> &f64 { &self.e[1] }
  pub fn z(&self) -> &f64 { &self.e[2] }

  pub fn r(&self) -> &f64 { &self.e[0] }
  pub fn g(&self) -> &f64 { &self.e[1] }
  pub fn b(&self) -> &f64 { &self.e[2] }

  pub fn length(&self) -> f64 {
    f64::sqrt(
      &self.e[0] * &self.e[0] +
        &self.e[1] * &self.e[1] +
        &self.e[2] * &self.e[2]
    )
  }

  pub fn unit_vector(&self) -> Vec3 {
    *self / self.length()
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] + other.e[0],
        self.e[1] + other.e[1],
        self.e[2] + other.e[2]
      ]
    }
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Vec3 {
      e: [
        self.e[0] - other.e[0],
        self.e[1] - other.e[1],
        self.e[2] - other.e[2]
      ]
    }
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, vec: Vec3) -> Vec3 {
    vec * self
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] * rhs,
        self.e[1] * rhs,
        self.e[2] * rhs
      ]
    }
  }
}

impl Div for Vec3 {
  type Output = Vec3;

  fn div(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] * other.e[0],
        self.e[1] * other.e[1],
        self.e[2] * other.e[2]
      ]
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f64) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] / rhs,
        self.e[1] / rhs,
        self.e[2] / rhs
      ]
    }
  }
}

impl fmt::Display for Vec3 {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({},{},{})", self.e[0], self.e[1], self.e[2])
  }
}
