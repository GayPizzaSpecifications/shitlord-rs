pub(crate) mod vector2;
pub(crate) mod extent;
pub(crate) mod rectangle;

mod scalar {
  pub(crate) trait ScalarSqrt { fn sqrt(self) -> Self; }
  impl ScalarSqrt for f32 { fn sqrt(self) -> Self { self.sqrt() } }
  impl ScalarSqrt for f64 { fn sqrt(self) -> Self { self.sqrt() } }

  #[allow(dead_code)]
  pub(crate) trait ScalarTrig {
    fn sine(self) -> Self;
    fn cosine(self) -> Self;
    fn tangent(self) -> Self;
    fn atan2(y: Self, x: Self) -> Self;
  }

  macro_rules! trig_implementation {
    ($t:ty) => {
      impl ScalarTrig for $t {
        fn sine(self) -> $t { self.sin() }
        fn cosine(self) -> $t { self.cos() }
        fn tangent(self) -> $t { self.tan() }
        fn atan2(y: $t, x: $t) -> Self { y.atan2(x) }
      }
    };
  }

  trig_implementation!(f32);
  trig_implementation!(f64);
}
