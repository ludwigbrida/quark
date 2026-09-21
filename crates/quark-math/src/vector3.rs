use std::ops::{Mul, MulAssign, Sub, SubAssign};

pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn dot(self, other: Self) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Sub for Vector3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl SubAssign for Vector3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl Mul for Vector3 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
    }
  }
}

impl MulAssign for Vector3 {
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
    self.z *= rhs.z;
  }
}
