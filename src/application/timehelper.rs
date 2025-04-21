use std::time::Duration;
use sdl3_sys::timer::*;
#[cfg(feature = "use-performance-counter")]
use crate::fpscalculator::DurationConv;

#[derive(Default)]
pub(in crate::application) struct TimeHelper {
  prev: u64,
  cur: u64,
  #[cfg(feature = "use-performance-counter")]
  freq: u64,
}

impl TimeHelper {
  pub(in crate::application) fn init(&mut self) {
    self.prev = self.instant();
  }

  pub(in crate::application) fn frame_advance(&mut self) {
    self.prev = self.cur;
    self.cur = self.instant();
  }

  #[inline]
  const fn raw_delta(&self) -> u64 {
    self.cur.wrapping_sub(self.prev)
  }
}

#[cfg(not(feature = "use-performance-counter"))]
impl TimeHelper {
  pub(in crate::application) fn get_deltatime(&self) -> f64 {
    const NS: f64 = 1.0 / 1_000_000_000.0;
    self.raw_delta() as f64 * NS
  }

  pub(in crate::application) fn get_duration(&self) -> Duration {
    Duration::from_nanos(self.raw_delta())
  }

  fn instant(&mut self) -> u64 {
    unsafe { SDL_GetTicksNS() }
  }
}

#[cfg(feature = "use-performance-counter")]
impl TimeHelper {
  pub(in crate::application) fn get_deltatime(&self) -> f64 {
     self.raw_delta() as f64 / self.freq as f64
  }

  pub(in crate::application) fn get_duration(&self) -> Duration {
    Duration::from_performance(self.cur.wrapping_sub(self.prev), self.freq)
  }

  fn instant(&mut self) -> u64 {
    unsafe {
      self.freq = SDL_GetPerformanceFrequency();
      SDL_GetPerformanceCounter()
    }
  }
}
