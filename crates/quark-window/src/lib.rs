mod application;
mod backend;
mod descriptor;
mod input;
mod size;
mod window;

pub use application::Application;
pub use descriptor::WindowDescriptor;
pub use input::{
  ButtonState, KeyCode, KeyInput, KeyState, MouseButton, MouseButtonInput, MouseMotion,
};
pub use size::WindowSize;
pub use window::Window;

pub fn run(descriptor: WindowDescriptor, application: impl Application + 'static) {
  backend::run(descriptor, application);
}
