use std::{
  cell::RefCell,
  fmt::Debug,
  sync::atomic::{AtomicU16, Ordering},
  time::Instant,
};

static COUNTER: AtomicU16 = AtomicU16::new(0);

thread_local! {
  static EPOCH: RefCell<Instant> = RefCell::new(Instant::now());
}

/// Universally unique identifier.
///
/// An offshoot of uuidv1 without conforming to the binary format.  Calculated by adding
/// a 16bit counter to the nanoseconds since thread start.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Uuid(u128);

impl Uuid {
  /// Create new [`Uuid`]
  pub fn new() -> Self {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let epoch = EPOCH.with(|epoch| epoch.clone().into_inner());

    // If duration since `EPOCH` overflows, replace `EPOCH` with [`Instant::now`] and eval
    // nanos as 0.
    let nanos = match Instant::now().checked_duration_since(epoch) {
      Some(elapsed) => elapsed.as_nanos(),
      None => {
        EPOCH.with(|epoch| epoch.replace(Instant::now()));

        0
      }
    };

    Self(nanos.wrapping_add(count as _))
  }

  /// Create
  pub fn nil() -> Self {
    Self(0)
  }
}

impl Default for Uuid {
  fn default() -> Self {
    Self::new()
  }
}

impl Debug for Uuid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:00000000000000000000000000000000}", self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::Uuid;
  use std::collections::HashSet;

  #[test]
  fn test_unique() {
    let mut ids = HashSet::<Uuid>::new();
    let count = 10_000;

    for _ in 0..count {
      ids.insert(Uuid::default());
    }

    assert_eq!(ids.len(), count);
  }
}
