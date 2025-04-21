use std::ops::{Add, Sub};
use crate::maths::extent::Extent;

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) struct Rectangle<T> { pub x: T, pub y: T, pub w: T, pub h: T }

impl<T> Rectangle<T> {
  #[inline]
  pub(crate) const fn new(x: T, y: T, w: T, h: T) -> Self { Self { x, y, w, h } }
}

impl<T: Copy + Add<Output = T>> Rectangle<T> {
  #[inline]
  pub(crate) fn right(&self) -> T { self.x + self.w }
  #[inline]
  pub(crate) fn bottom(&self) -> T { self.y + self.h }
}

impl<T: Copy + Sub<Output = T>> From<Extent<T>> for Rectangle<T> {
  #[inline]
  fn from(extent: Extent<T>) -> Self {
    Self::new(extent.left, extent.top, extent.width(), extent.height())
  }
}

pub type Rect = Rectangle<i32>;
pub type FRect = Rectangle<f32>;
