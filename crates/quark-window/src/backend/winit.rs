use crate::application::Application;
use crate::{Window, WindowDescriptor};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window as WinitWindow, WindowId};

pub fn run(descriptor: WindowDescriptor, application: impl Application + 'static) {
  let event_loop = EventLoop::new().expect("failed to create event loop");

  event_loop
    .run_app(&mut App {
      descriptor,
      application,
      window: None,
    })
    .expect("event loop failed");
}

struct App<A> {
  descriptor: WindowDescriptor,
  application: A,
  window: Option<Window>,
}

impl<A: Application> ApplicationHandler for App<A> {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.window.is_some() {
      return;
    }

    let attributes = WinitWindow::default_attributes()
      .with_title(self.descriptor.title.as_str())
      .with_inner_size(PhysicalSize::new(
        self.descriptor.inner_size.width,
        self.descriptor.inner_size.height,
      ));

    let window = Window::new(
      event_loop
        .create_window(attributes)
        .expect("failed to create window"),
    );

    self.application.window_created(&window);
    window.request_redraw();

    self.window = Some(window);
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::RedrawRequested => {
        if let Some(window) = &self.window {
          self.application.redraw_requested(window);
        }
      }
      _ => {}
    }
  }

  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    if let Some(window) = &self.window {
      self.application.about_to_wait(window);
    }
  }
}
