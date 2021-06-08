use std::{
  sync::atomic::{AtomicU16, Ordering},
  time::Instant,
};

static COUNTER: AtomicU16 = AtomicU16::new(0);

thread_local! {
  static EPOCH: Instant = Instant::now();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Uuid(u128);

impl Uuid {
  pub fn new() -> Self {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let duration = EPOCH.with(|epoch| epoch.elapsed());

    Self(duration.as_nanos().wrapping_add(count as _))
  }
}

impl Default for Uuid {
  fn default() -> Self {
    Self::new()
  }
}
