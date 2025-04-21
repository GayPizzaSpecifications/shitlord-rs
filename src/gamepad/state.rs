use crate::gamepad::axis::PadAxis;
use crate::gamepad::button::{PadButton, PadButtons};

pub(crate) struct PadState {
  axes: [i16; 6],
  btns: i32,
  btns_impulse: i32
}

impl PadState {
  pub(in crate::gamepad) fn new(axes: [i16; 6], cur: i32, prev: i32) -> Self {
    Self { axes, btns: cur, btns_impulse: cur ^ prev }
  }
}

#[allow(dead_code)]
impl PadState {
  pub(crate) fn axis(&self, axis: PadAxis) -> f32 {
    let raw = self.raw_axis(axis);
    let rescale = if raw < 0 {
      1f32 / -(i16::MIN as i32) as f32
    } else {
      1f32 / i16::MAX as f32
    };
    raw as f32 * rescale
  }
  fn raw_axis(&self, axis: PadAxis) -> i16 {
    self.axes[axis as usize]
  }

  pub(crate) fn down(&self, btn: PadButton) -> bool {
    (self.btns & btn.value()) == btn.value()
  }
  pub(crate) fn pressed(&self, btn: PadButton) -> bool {
    ((self.btns & self.btns_impulse) & btn.value()) == btn.value()
  }
  pub(crate) fn pressed_any(&self, btns: PadButtons) -> bool {
    ((self.btns & self.btns_impulse) & btns.value()) != 0
  }
  pub(crate) fn released(&self, btn: PadButton) -> bool {
    ((self.btns_impulse & !self.btns) & btn.value()) == btn.value()
  }
}
