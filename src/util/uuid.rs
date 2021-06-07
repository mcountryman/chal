use std::{
  sync::atomic::{AtomicU16, Ordering},
  time::Instant,
};

static COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Uuid {
  counter: u16,
  timestamp: Instant,
}

impl Uuid {
  pub fn new() -> Self {
    Self {
      counter: COUNTER.fetch_add(1, Ordering::SeqCst),
      timestamp: Instant::now(),
    }
  }
}

impl Default for Uuid {
  fn default() -> Self {
    Self::new()
  }
}
