use sdl3_sys::gamepad::*;

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(i32)]
pub(crate) enum PadButton {
  East  = 1 << SDL_GamepadButton::EAST.0,
  West  = 1 << SDL_GamepadButton::WEST.0,
  North = 1 << SDL_GamepadButton::NORTH.0,
  South = 1 << SDL_GamepadButton::SOUTH.0,
  Start = 1 << SDL_GamepadButton::START.0,
  Guide = 1 << SDL_GamepadButton::GUIDE.0,
  LeftStick  = 1 << SDL_GamepadButton::LEFT_STICK.0,
  RightStick = 1 << SDL_GamepadButton::RIGHT_STICK.0,
  LeftShoulder  = 1 << SDL_GamepadButton::LEFT_SHOULDER.0,
  RightShoulder = 1 << SDL_GamepadButton::RIGHT_SHOULDER.0,
  DPadLeft  = 1 << SDL_GamepadButton::DPAD_LEFT.0,
  DPadRight = 1 << SDL_GamepadButton::DPAD_RIGHT.0,
  DPadUp    = 1 << SDL_GamepadButton::DPAD_UP.0,
  DPadDown  = 1 << SDL_GamepadButton::DPAD_DOWN.0,
  Misc1 = 1 << SDL_GamepadButton::MISC1.0,
  Misc2 = 1 << SDL_GamepadButton::MISC2.0,
  Misc3 = 1 << SDL_GamepadButton::MISC3.0,
  Misc4 = 1 << SDL_GamepadButton::MISC4.0,
  Misc5 = 1 << SDL_GamepadButton::MISC5.0,
  Misc6 = 1 << SDL_GamepadButton::MISC6.0,
  Paddle1 = 1 << SDL_GamepadButton::RIGHT_PADDLE1.0,
  Paddle2 = 1 << SDL_GamepadButton::LEFT_PADDLE1.0,
  Paddle3 = 1 << SDL_GamepadButton::RIGHT_PADDLE2.0,
  Paddle4 = 1 << SDL_GamepadButton::LEFT_PADDLE2.0,
  TouchPad = 1 << SDL_GamepadButton::TOUCHPAD.0,
}

impl PadButton {
  #[inline]
  pub(crate) const fn value(&self) -> i32 { *self as i32 }
}
