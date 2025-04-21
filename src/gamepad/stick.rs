use crate::gamepad::axis::PadAxis;
use crate::gamepad::state::PadState;
use crate::maths::vector2::Vec2f;

#[allow(dead_code)]
impl PadState {
  #[inline]
  pub(crate) fn left_stick(&self) -> Vec2f {
    Vec2f::new(self.axis(PadAxis::LeftStickX), self.axis(PadAxis::LeftStickY))
  }
  #[inline]
  pub(crate) fn right_stick(&self) -> Vec2f {
    Vec2f::new(self.axis(PadAxis::RightStickX), self.axis(PadAxis::RightStickY))
  }
  #[inline]
  pub(crate) fn left_trigger(&self) -> f32 { self.axis(PadAxis::LeftTrigger) }
  #[inline]
  pub(crate) fn right_trigger(&self) -> f32 { self.axis(PadAxis::RightTrigger) }
}
