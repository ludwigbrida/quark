use crate::WindowSize;
use raw_window_handle::{
  DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use std::sync::Arc;
use winit::window::Window as WinitWindow;

#[derive(Clone)]
pub struct Window {
  inner: Arc<WinitWindow>,
}

impl Window {
  pub(crate) fn new(inner: WinitWindow) -> Self {
    Self {
      inner: Arc::new(inner),
    }
  }

  pub fn inner_size(&self) -> WindowSize {
    let size = self.inner.inner_size();

    WindowSize {
      width: size.width,
      height: size.height,
    }
  }

  pub fn request_redraw(&self) {
    self.inner.request_redraw();
  }
}

impl HasWindowHandle for Window {
  fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
    self.inner.window_handle()
  }
}

impl HasDisplayHandle for Window {
  fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
    self.inner.display_handle()
  }
}
