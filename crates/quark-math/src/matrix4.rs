use crate::Vector3;
use std::ops::{Index, IndexMut, Mul};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
  data: [[f32; 4]; 4],
}

impl Matrix4 {
  pub const IDENTITY: Self = Self::new([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);

  pub const fn new(data: [[f32; 4]; 4]) -> Self {
    Self { data }
  }

  pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
    let focal_length = 1.0 / (fov_y / 2.0).tan();
    let inverse_depth_range = 1.0 / (near - far);

    Self::new([
      [focal_length / aspect, 0.0, 0.0, 0.0],
      [0.0, focal_length, 0.0, 0.0],
      [0.0, 0.0, far * inverse_depth_range, -1.0],
      [0.0, 0.0, far * near * inverse_depth_range, 0.0],
    ])
  }

  pub const fn translation(translation: Vector3) -> Self {
    Self::new([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [translation.x, translation.y, translation.z, 1.0],
    ])
  }

  pub fn rotation_x(radians: f32) -> Self {
    let cosine = radians.cos();
    let sine = radians.sin();

    Self::new([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, cosine, sine, 0.0],
      [0.0, -sine, cosine, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn rotation_y(radians: f32) -> Self {
    let cosine = radians.cos();
    let sine = radians.sin();

    Self::new([
      [cosine, 0.0, -sine, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [sine, 0.0, cosine, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn rotation_z(radians: f32) -> Self {
    let cosine = radians.cos();
    let sine = radians.sin();

    Self::new([
      [cosine, sine, 0.0, 0.0],
      [-sine, cosine, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub const fn columns(&self) -> &[[f32; 4]; 4] {
    &self.data
  }
}

impl Index<usize> for Matrix4 {
  type Output = [f32; 4];

  fn index(&self, column: usize) -> &Self::Output {
    &self.data[column]
  }
}

impl IndexMut<usize> for Matrix4 {
  fn index_mut(&mut self, column: usize) -> &mut Self::Output {
    &mut self.data[column]
  }
}

impl Mul for Matrix4 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let mut result = Self::new([[0.0; 4]; 4]);

    for column in 0..4 {
      for row in 0..4 {
        for index in 0..4 {
          result[column][row] += self[index][row] * rhs[column][index];
        }
      }
    }

    result
  }
}
