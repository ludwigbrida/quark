use crate::{KeyInput, MouseButtonInput, MouseMotion, Window, WindowSize};

pub trait Application {
  fn window_created(&mut self, window: &Window);
  fn redraw_requested(&mut self, window: &Window);
  fn about_to_wait(&mut self, _window: &Window) {}
  fn resized(&mut self, _window: &Window, _size: WindowSize) {}
  fn focus_changed(&mut self, _window: &Window, _focused: bool) {}
  fn key_input(&mut self, _window: &Window, _input: KeyInput) {}
  fn mouse_button_input(&mut self, _window: &Window, _input: MouseButtonInput) {}
  fn mouse_motion(&mut self, _window: &Window, _motion: MouseMotion) {}
}
