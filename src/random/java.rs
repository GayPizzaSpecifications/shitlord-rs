use std::ops::Range;
use crate::random::drand48::Drand48;
use crate::random::{RandomNextBits, RandomProvider};

/// Welcome to the world of Java Sound
pub(crate) trait JavaExtensions {
  #[allow(dead_code)]
  fn next_int(&mut self) -> i32;
  fn next_bound(&mut self, bound: i32) -> i32;
  fn next_range(&mut self, range: Range<i32>) -> i32;
  fn next_float(&mut self) -> f32;
}

impl JavaExtensions for Drand48 {
  fn next_int(&mut self) -> i32 { self.next_bits(32) as i32 }

  fn next_bound(&mut self, bound: i32) -> i32 {
    assert!(bound > 0, "Bounded next called with empty bound");
    assert_eq!(Self::MIN, 0,
      "Range operations are unsupported on random providers with a non-zero minimum");
    //assert!(Self::MAX >= bound as u32,
    //  "Maximum raw random provider output is smaller than requested bound");

    let mask = bound - 1;
    if bound & mask == bound {  // No need to modulo if we're power of two
      let num_bits = 31 - bound.leading_zeros();
      return self.next_bits(num_bits as i32) as i32
    }
    loop {
      let bits = self.next_bits(31) as i32;
      let result = bits % bound;
      if bits - result + mask >= 0 {
        return result
      }
    }
  }

  fn next_range(&mut self, range: Range<i32>) -> i32 {
    assert!(!range.is_empty(), "Ranged next called with empty range");
    range.start + self.next_bound(range.end - range.start)
  }

  fn next_float(&mut self) -> f32 {
    const RESCALE: f32 = 1.0 / (1 << 24) as f32;  // hexf32!("0x1.0p-24");
    self.next_bits(24) as f32 * RESCALE
  }
}
