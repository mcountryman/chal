pub mod error;

pub use error::*;

use std::iter::Peekable;

use super::Tokenizer;

#[derive(Debug, Clone)]
pub struct Parser<'buf> {
  tokens: Peekable<Tokenizer<'buf>>,
}
