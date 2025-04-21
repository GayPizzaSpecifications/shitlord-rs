use std::time::Duration;

pub struct FPSCalculator {
  accumulator: Duration,
  frames_count: usize,
}

impl Default for FPSCalculator {
  fn default() -> Self {
    FPSCalculator {
      accumulator: Duration::new(0, 0),
      frames_count: 0,
    }
  }
}

impl FPSCalculator {
  pub(crate) fn frame<F>(&mut self, delta: Duration, result: F) where F: FnOnce(usize) {
    self.frames_count += 1;
    self.accumulator += delta;

    if self.accumulator >= Duration::new(1, 0) {
      result(self.frames_count);

      self.frames_count = 0;
      self.accumulator = Duration::new(0, self.accumulator.subsec_nanos());
    }
  }
}

pub(crate) trait DurationConv {
  fn from_performance(counter: u64, frequency: u64) -> Self;
}

impl DurationConv for Duration {
  fn from_performance(counter_delta: u64, frequency: u64) -> Self {
    let dividend = counter_delta as f64;
    let divisor = frequency as f64;
    Self::from_secs_f64(dividend / divisor)
  }
}
