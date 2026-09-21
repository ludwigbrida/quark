use crate::WindowDescriptor;
use crate::application::Application;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

pub fn run(descriptor: WindowDescriptor, application: impl Application + 'static) {
  let event_loop = EventLoop::new().expect("failed to create event loop");

  let mut app = App {
    descriptor,
    window: None,
  };

  event_loop.run_app(&mut app).expect("event loop failed");
}

struct App {
  descriptor: WindowDescriptor,
  window: Option<Window>,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let attributes = Window::default_attributes()
      .with_title(self.descriptor.title.as_str())
      .with_inner_size(PhysicalSize::new(
        self.descriptor.inner_size.width,
        self.descriptor.inner_size.height,
      ));

    self.window = Some(
      event_loop
        .create_window(attributes)
        .expect("failed to create window"),
    );
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
