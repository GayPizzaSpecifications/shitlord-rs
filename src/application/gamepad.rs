use std::borrow::Cow;
use std::ffi::CStr;
use crate::application::gamepad::state::PadState;
use sdl3_sys::gamepad::*;
use sdl3_sys::joystick::SDL_JoystickID;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

pub(crate) mod state;
pub(crate) mod button;
pub(crate) mod axis;
pub(crate) mod deadzone;
mod stick;

pub(crate) struct GamePad {
  joy_instance: SDL_JoystickID,
  sdl_pad: *mut SDL_Gamepad,
  axes: [i16; 6],
  btns_cur: i32,
  btns_prev: i32,
}

impl GamePad {
  fn open(id: SDL_JoystickID) -> Option<Self> {
    let sdl_pad = unsafe { SDL_OpenGamepad(id).as_mut() }?;
    Some(Self {
      joy_instance: id,
      sdl_pad,
      axes: Default::default(),
      btns_cur: 0,
      btns_prev: 0,
    })
  }

  pub(crate) fn name(&self) -> Cow<'_, str> {
    unsafe { CStr::from_ptr(SDL_GetGamepadName(self.sdl_pad)).to_string_lossy() }
  }
}

impl Drop for GamePad {
  fn drop(&mut self) {
    unsafe { SDL_CloseGamepad(self.sdl_pad) };
  }
}

unsafe impl Send for GamePad {}  // Allow sending *mut SDL_Gamepad

static GAME_PADS: Mutex<Vec<GamePad>> = Mutex::new(vec![]);
static FIRST_ID: AtomicU32 = AtomicU32::new(0);

impl GamePad {
  pub(crate) fn current() -> Option<PadState> {
    Self::state(FIRST_ID.load(Ordering::Relaxed))
  }

  pub(crate) fn state(id: u32) -> Option<PadState> {
    Some(GAME_PADS.lock()
      .expect("lock not obtained")
      .iter().find(|x| x.joy_instance == id)?.get_state())
  }

  fn get_state(&self) -> PadState {
    PadState::new(self.axes, self.btns_cur, self.btns_prev)
  }
}

impl GamePad {
  pub(in crate::application) fn connected_event(id: SDL_JoystickID) {
    let mut pads = GAME_PADS.lock().expect("lock not obtained");
    if pads.iter().any(|x| x.joy_instance == id) {
      return;
    }
    if let Some(pad) = Self::open(id) {
      eprintln!("Using gamepad #{} \"{}\"", id, pad.name());
      if FIRST_ID.load(Ordering::Relaxed) == 0 {
        FIRST_ID.store(id, Ordering::Relaxed);
      }
      pads.push(pad);
    }
  }

  pub(in crate::application) fn removed_event(id: SDL_JoystickID) {
    let mut pads = GAME_PADS.lock().expect("lock not obtained");
    if let Some(idx) = pads.iter().position(|x| x.joy_instance == id) {
      pads.remove(idx);
    }
    if FIRST_ID.load(Ordering::Relaxed) == id {
      FIRST_ID.store(pads.first().map_or(0, |x| x.joy_instance), Ordering::Relaxed);
    }
  }

  pub(in crate::application) fn button_event(id: SDL_JoystickID, btn: SDL_GamepadButton, down: bool) {
    let mut pads = GAME_PADS.lock().expect("lock not obtained");
    if let Some(pad) = pads.iter_mut().find(|x| x.joy_instance == id) {
      let btn_mask = 1 << btn.0;
      pad.btns_cur = if down { pad.btns_cur | btn_mask } else { pad.btns_cur & !btn_mask };
    }
  }

  pub(in crate::application) fn axis_event(id: SDL_JoystickID, axis: SDL_GamepadAxis, value: i16) {
    let mut pads = GAME_PADS.lock().expect("lock not obtained");
    if let Some(pad) = pads.iter_mut().find(|x| x.joy_instance == id) {
      pad.axes[axis.0 as usize] = value;
    }
  }

  pub(in crate::application) fn advance_frame() {
    for pad in GAME_PADS.lock().expect("lock not obtained").iter_mut() {
      pad.btns_prev = pad.btns_cur;
    }
  }
}
