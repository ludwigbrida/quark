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
