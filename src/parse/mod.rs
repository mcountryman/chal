pub mod ast;
pub mod errors;
pub mod tokens;
pub mod visit;

pub use errors::*;
pub use tokens::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
  line: usize,
  column: usize,
  offset: usize,
}

impl Position {
  pub fn new(line: usize, column: usize, offset: usize) -> Self {
    Self {
      line,
      column,
      offset,
    }
  }

  pub fn line(&self) -> usize {
    self.line
  }

  pub fn column(&self) -> usize {
    self.column
  }

  pub fn offset(&self) -> usize {
    self.offset
  }
}

impl Default for Position {
  fn default() -> Self {
    Self::new(1, 1, 0)
  }
}

pub trait Positional {
  fn position(&self) -> &Position;
}

#[derive(Debug, Clone)]
pub struct Span<'buf> {
  beg: Position,
  end: Position,
  buf: &'buf str,
}

impl<'buf> Span<'buf> {
  pub fn new(beg: Position, end: Position, buf: &'buf str) -> Self {
    Self { beg, end, buf }
  }

  pub fn as_str(&self) -> &'buf str {
    &self.buf[self.beg.offset..self.end.offset]
  }
}
