use crate::Window;

pub trait Application {
  fn window_created(&mut self, window: &Window);
  fn redraw_requested(&mut self, window: &Window);
  fn about_to_wait(&mut self, _window: &Window) {}
}
