use std::ops::Sub;

pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
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
