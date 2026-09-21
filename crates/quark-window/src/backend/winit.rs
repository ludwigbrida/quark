use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

pub fn run() {
  let event_loop = EventLoop::new();
  let mut app = App::default();

  event_loop.unwrap().run_app(&mut app).unwrap()
}

#[derive(Default)]
struct App {
  window: Option<Window>,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.window = event_loop.create_window(Window::default_attributes()).ok();
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    if matches!(event, WindowEvent::CloseRequested) {
      event_loop.exit();
    }
  }
}
