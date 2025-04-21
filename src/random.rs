/*
pub fn sketchy_hash(mut seed: u32) -> u32 {
  seed = !seed.wrapping_add(4);
  seed ^= seed >> 17;
  seed ^= seed << 3;
  return seed;
}

pub fn wang_hash(mut seed: u32) -> u32 {
  seed = (seed ^ 61) ^ (seed >> 16);
  seed = seed.wrapping_mul(9);
  seed ^= seed >> 4;
  seed = seed.wrapping_mul(0x27d4eb2d);
  seed ^= seed >> 15;
  return seed;
}

pub fn pcg_hash(mut x: u32) -> u32  {
  x = x.wrapping_mul(747796405).wrapping_add(2891336453);
  x ^= x >> (x >> 28).wrapping_add(4);
  x = x.wrapping_mul(277803737);
  x ^= x >> 2;
  return x;
}
*/

#![allow(dead_code)]

pub(crate) mod drand48;
pub(crate) mod java;

trait RandomProvider {
  type Output;

  const MIN: Self::Output;
  const MAX: Self::Output;

  fn next(&mut self) -> Self::Output;
}

trait RandomNextBits {
  type BitsOutput;

  fn next_bits(&mut self, bits: i32) -> Self::BitsOutput;
}

trait RandomSeedable {
  type SeedType;

  fn new_seeded(seed: Self::SeedType) -> Self;
  fn seed(&mut self, seed: Self::SeedType);
}

trait RandomStateAccess {
  type StateType;

  fn get_state(&self) -> Self::StateType;
  fn set_state(&mut self, state: Self::StateType);
}
