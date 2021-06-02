use crate::types::Span;
use std::{error::Error, fmt::Display, num::ParseFloatError};

pub type LexResult<'buf, T> = Result<T, LexError<'buf>>;

/// An error which can be returned when tokenizing a str.
#[derive(Debug, Clone)]
pub enum LexError<'buf> {
  BadIdent(String, Span<'buf>),
  BadString(String, Span<'buf>),
  BadNumber(ParseFloatError, Span<'buf>),
  Unexpected(String, Span<'buf>),
}

impl LexError<'_> {
  /// Creates an unexpected character error.
  pub fn unexpected_char(span: Span<'_>) -> LexError<'_> {
    LexError::Unexpected("Unexpected character".to_string(), span)
  }

  /// Creates a bad identifier error.
  pub fn bad_ident_numeric_before_alpha(span: Span<'_>) -> LexError<'_> {
    LexError::BadIdent(
      "Invalid variable name, expected alphabetic or `_` before numeric".to_string(),
      span,
    )
  }

  /// Creates a bad string error.
  pub fn bad_string_unexpected_eof(span: Span<'_>) -> LexError<'_> {
    LexError::BadString(
      "Invalid string, expected closing quote, got end of file".to_string(),
      span,
    )
  }

  /// Creates a bad string error.
  pub fn bad_string_unexpected_eol(span: Span<'_>) -> LexError<'_> {
    LexError::BadString(
      "Invalid string, expected closing quote, got end of line".to_string(),
      span,
    )
  }
}

impl Display for LexError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for LexError<'_> {}
