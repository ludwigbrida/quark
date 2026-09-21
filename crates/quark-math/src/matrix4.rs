pub struct Matrix4 {
  data: [[f32; 4]; 4],
}

impl Matrix4 {
  pub const fn new(data: [[f32; 4]; 4]) -> Self {
    Self { data }
  }

  pub const IDENTITY: Self = Self::new([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]);
}
