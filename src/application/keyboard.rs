pub(crate) mod key;

use sdl3_sys::scancode::SDL_Scancode;
use std::sync::Mutex;
pub(crate) use crate::application::keyboard::key::Key;

#[allow(dead_code)]
impl Keyboard {
  pub(crate) fn down(key: Key) -> bool {
    KEYBOARD_STATE.lock().expect("lock not obtained")
      [SDL_Scancode::from(key).0 as usize] & Keyboard::_DOWN != 0
  }

  pub(crate) fn pressed(key: Key) -> bool {
    KEYBOARD_STATE.lock().expect("lock not obtained")
      [SDL_Scancode::from(key).0 as usize] == Keyboard::_PRESS
  }

  pub(crate) fn repeat(key: Key) -> bool {
    KEYBOARD_STATE.lock().expect("lock not obtained")
      [SDL_Scancode::from(key).0 as usize] & !Keyboard::_REPEAT == Keyboard::_PRESS
  }

  pub(crate) fn released(key: Key) -> bool {
    KEYBOARD_STATE.lock().expect("lock not obtained")
      [SDL_Scancode::from(key).0 as usize] == Keyboard::_RELEASE
  }
}

pub(crate) struct Keyboard();

static KEYBOARD_STATE: Mutex<[u8; SDL_Scancode::COUNT.0 as usize]>
  = Mutex::new([Keyboard::_UP; SDL_Scancode::COUNT.0 as usize]);

impl Keyboard {
  const _UP: u8      = 0b000;
  const _DOWN: u8    = 0b010;
  const _IMPULSE: u8 = 0b001;
  const _REPEAT: u8  = 0b100;

  const _PRESS: u8 = Self::_DOWN | Self::_IMPULSE;
  const _RELEASE: u8 = Self::_UP | Self::_IMPULSE;

  pub(in crate::application) fn key_event(code: SDL_Scancode, down: bool, repeat: bool) {
    KEYBOARD_STATE.lock().expect("lock not obtained")[code.0 as usize] = if down {
      if repeat { Self::_REPEAT | Self::_PRESS } else { Self::_PRESS }
    } else {
      Self::_RELEASE
    };
  }

  pub(in crate::application) fn advance_frame() {
    KEYBOARD_STATE.lock().expect("lock not obtained").iter_mut()
      .for_each(|x| *x &= !(Self::_IMPULSE | Self::_REPEAT));
  }
}
