mod application;
mod backend;
mod descriptor;
mod size;

pub use application::Application;
pub use descriptor::WindowDescriptor;
pub use size::WindowSize;

pub fn run(descriptor: WindowDescriptor, application: impl Application + 'static) {
  backend::run(descriptor, application);
}
