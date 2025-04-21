mod actor;
mod application;
mod maths;
mod renderer;
mod state;
mod random;
mod fpscalculator;

use crate::application::Application;
use std::process::ExitCode;

pub fn main() -> ExitCode {
  ExitCode::from(Application::run())
}
