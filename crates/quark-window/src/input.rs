pub use winit::{event::MouseButton, keyboard::KeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
  Pressed,
  Released,
}

pub type KeyState = ButtonState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyInput {
  pub code: KeyCode,
  pub state: ButtonState,
  pub repeat: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButtonInput {
  pub button: MouseButton,
  pub state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMotion {
  pub delta: [f64; 2],
}
