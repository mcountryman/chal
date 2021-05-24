pub mod tokens;
pub use tokens::*;

pub mod errors;
pub use errors::*;

#[derive(Debug, Clone, Copy)]
pub struct Position {
  line: usize,
  column: usize,
  offset: usize,
}

impl Position {
  pub fn new() -> Self {
    Self {
      line: 1,
      column: 1,
      offset: 0,
    }
  }
}

impl Default for Position {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Clone)]
pub struct Span<'a> {
  beg: Position,
  end: Position,
  buf: &'a str,
}
