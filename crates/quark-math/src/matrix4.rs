pub struct Matrix4 {
  data: [[f32; 4]; 4],
}

impl Matrix4 {
  pub const fn new(data: [[f32; 4]; 4]) -> Self {
    Self { data }
  }
}
