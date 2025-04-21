use std::ops::{Add, Sub};
use crate::maths::rectangle::Rectangle;

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) struct Extent<T> { pub left: T, pub top: T, pub right: T, pub bottom: T }

impl<T> Extent<T> {
  #[inline]
  pub(crate) const fn new(left: T, top: T, right: T, bottom: T) -> Self {
    Self { left, top, right, bottom }
  }
}

impl<T: Copy + Sub<Output = T>> Extent<T> {
  #[inline]
  pub(crate) fn width(&self) -> T { self.right - self.left }
  #[inline]
  pub(crate) fn height(&self) -> T { self.bottom - self.top }
}

impl<T: Copy + Add<Output = T>> From<Rectangle<T>> for Extent<T> {
  #[inline]
  fn from(rect: Rectangle<T>) -> Self {
    Self::new(rect.x, rect.y, rect.right(), rect.bottom())
  }
}
