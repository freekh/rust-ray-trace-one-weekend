use std::ops::*;
use std::fmt;

/* This struct is completly unecessary of course, however following the book here so... */
#[derive(Copy, Clone)]
pub struct Vec3 {
  e: [f64; 3]
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
  v1.x() * v2.x() +
    v1.y() * v2.y() +
    v1.z() * v2.z()
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
  Vec3::new(
    v1.y() * v2.z() - v1.z() * v2.y(),
    -(v1.x() * v2.z() - v1.z() * v2.x()),
    v1.x() * v2.y() - v1.y() * v2.x(),
  )
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

  pub fn squared_length(&self) -> f64 {
    self.e[0] * self.e[0] + 
      self.e[1] * self.e[1] + 
      self.e[2] * self.e[2]
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
        other.e[0] + self.e[0],
        other.e[1] + self.e[1],
        other.e[2] + self.e[2]
      ]
    }
  }
}

impl Add<f64> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: f64) -> Vec3 {
    Vec3 {
      e: [
        rhs + self.e[0],
        rhs + self.e[1],
        rhs + self.e[2]
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


impl Sub<f64> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: f64) -> Vec3 {
    Vec3 {
      e: [
        rhs - self.e[0],
        rhs - self.e[1],
        rhs - self.e[2]
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


impl Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, vec: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] * vec.e[0],
        self.e[1] * vec.e[1],
        self.e[2] * vec.e[2]
      ]
    }
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
        other.e[0] / self.e[0],
        other.e[1] / self.e[1],
        other.e[2] / self.e[2]
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

impl Neg for Vec3 {
  type Output = Vec3;
  
  fn neg(self) -> Vec3 {
    Vec3 {
      e: [
        -self.e[0],
        -self.e[1],
        -self.e[2]
      ]
    }
  }
}

impl fmt::Display for Vec3 {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({},{},{})", self.e[0], self.e[1], self.e[2])
  }
}
