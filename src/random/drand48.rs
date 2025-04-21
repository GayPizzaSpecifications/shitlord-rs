use crate::random::{RandomNextBits, RandomProvider, RandomSeedable, RandomStateAccess};
use std::time::UNIX_EPOCH;

pub(crate) struct Drand48 { seed: u64 }

impl Drand48 {
  const BITS: i32 = 48;
  const MASK: u64 = (1 << Self::BITS) - 1;  // '𝑚'
  const MULTIPLIER: u64 = 0x5DEECE66D;      // '𝑎'
  const INCREMENT: u64 = 0xB;               // '𝑐'

  fn update(&mut self) -> u64 {
    // 𝑿ₙ₊₁=(𝑎𝑿ₙ＋𝑐) mod 𝑚
    let next = self.seed
      .wrapping_mul(Self::MULTIPLIER)
      .wrapping_add(Self::INCREMENT) & Self::MASK;
    self.seed = next;
    next
  }
}

impl Default for Drand48 {
  fn default() -> Self {
    Self::new_seeded(UNIX_EPOCH.elapsed().map_or(0, |x| x.as_nanos() as u64))
  }
}

impl RandomNextBits for Drand48 {
  type BitsOutput = u64;

  fn next_bits(&mut self, bits: i32) -> u64 {
    assert!(bits <= Self::BITS);
    self.update() >> (Self::BITS - bits)
  }
}

impl RandomProvider for Drand48 {
  type Output = u32;

  const MIN: Self::Output = u32::MIN;
  const MAX: Self::Output = u32::MAX;

  fn next(&mut self) -> u32 { self.next_bits(32) as u32 }
}

impl RandomSeedable for Drand48 {
  type SeedType = u64;

  fn new_seeded(seed: u64) -> Self { Self { seed: (seed ^ Self::MULTIPLIER) & Self::MASK } }
  fn seed(&mut self, seed: u64) { self.seed = (seed ^ Self::MULTIPLIER) & Self::MASK }
}

impl RandomStateAccess for Drand48 {
  type StateType = u64;

  fn get_state(&self) -> Self::StateType { self.seed }
  fn set_state(&mut self, state: Self::StateType) { self.seed = state }
}
