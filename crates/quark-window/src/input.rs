pub use winit::keyboard::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
  Pressed,
  Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyInput {
  pub code: KeyCode,
  pub state: KeyState,
  pub repeat: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMotion {
  pub delta: [f64; 2],
}
