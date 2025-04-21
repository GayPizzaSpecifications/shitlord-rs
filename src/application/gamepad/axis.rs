use sdl3_sys::gamepad::*;

#[derive(Copy, Clone)]
#[repr(i32)]
pub(crate) enum PadAxis {
  LeftStickX = SDL_GamepadAxis::LEFTX.0,
  LeftStickY = SDL_GamepadAxis::LEFTY.0,
  RightStickX = SDL_GamepadAxis::RIGHTX.0,
  RightStickY = SDL_GamepadAxis::RIGHTY.0,
  LeftTrigger = SDL_GamepadAxis::LEFT_TRIGGER.0,
  RightTrigger = SDL_GamepadAxis::RIGHT_TRIGGER.0,
}
