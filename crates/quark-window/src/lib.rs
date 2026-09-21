mod backend;
mod descriptor;
mod size;

pub use descriptor::WindowDescriptor;
pub use size::WindowSize;

pub fn run(descriptor: WindowDescriptor) {
  backend::run(descriptor);
}
