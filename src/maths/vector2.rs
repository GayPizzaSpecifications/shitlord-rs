use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::maths::scalar::{ScalarSqrt, ScalarTrig};

#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub struct Vector2<T> { pub x: T, pub y: T }

impl<T: Default> Vector2<T> {
  #[inline(always)]
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

pub trait VectorConstants { const ZERO: Self; const ONE: Self; }
impl VectorConstants for f32 {
  const ZERO: Self = 0.0;
  const ONE: Self = 1.0;
}
impl VectorConstants for f64 {
  const ZERO: Self = 0.0;
  const ONE: Self = 1.0;
}
impl VectorConstants for i32 {
  const ZERO: Self = 0;
  const ONE: Self = 1;
}
impl VectorConstants for u32 {
  const ZERO: Self = 0;
  const ONE: Self = 1;
}

#[allow(dead_code)]
impl<T: VectorConstants> Vector2<T> {
  pub const ZERO: Self = Self { x: T::ZERO, y: T::ZERO };
  pub const ONE: Self = Self { x: T::ONE, y: T::ONE };

  pub const X: Self = Self { x: T::ONE, y: T::ZERO };
  pub const Y: Self = Self { x: T::ZERO, y: T::ONE };
}

impl<T: AddAssign> AddAssign for Vector2<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

impl<T: MulAssign + Clone> MulAssign for Vector2<T> {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
  }
}

impl<T: DivAssign> DivAssign for Vector2<T> {
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    self.x /= rhs.x;
    self.y /= rhs.y;
  }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    Self { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
  type Output = Self;
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self {
    Self { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}

impl<T: Mul<Output = T>> Mul for Vector2<T> {
  type Output = Self;
  #[inline(always)]
  fn mul(self, rhs: Self) -> Self {
    Self { x: self.x * rhs.x, y: self.y * rhs.y }
  }
}

impl<T: Div<Output = T>> Div for Vector2<T> {
  type Output = Self;
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    Self { x: self.x / rhs.x, y: self.y / rhs.y }
  }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
  type Output = Vector2<T>;
  #[inline(always)]
  fn mul(self, rhs: T) -> Self {
    Self { x: self.x * rhs, y: self.y * rhs }
  }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector2<T> {
  type Output = Vector2<T>;
  #[inline(always)]
  fn div(self, rhs: T) -> Self {
    Self { x: self.x / rhs, y: self.y / rhs }
  }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vector2<T> {
  #[inline(always)]
  fn dot(&self, rhs: Self) -> T { self.x * rhs.x + self.y * rhs.y }
  #[inline(always)]
  fn mag2(&self) -> T { self.dot(*self) }
}

impl<T: ScalarSqrt + Add<Output = T> + Mul<Output = T> + Copy> Vector2<T> {
  #[inline(always)]
  pub(crate) fn mag(self) -> T { self.mag2().sqrt() }
}

impl<T: ScalarTrig + Copy> Vector2<T> {
  pub(crate) fn angle(&self) -> T { T::atan2(self.y, self.x) }
}


pub type Vec2f = Vector2<f32>;
#[allow(dead_code)]
pub type Vec2d = Vector2<f64>;
