use crate::parse::{ParseError, ParseResult};

use super::Span;
use std::{error::Error, fmt::Display, num::ParseFloatError};

pub type TokenizeResult<'buf, T> = Result<T, TokenizeError<'buf>>;

/// An error which can be returned when tokenizing a str.
#[derive(Debug, Clone)]
pub enum TokenizeError<'buf> {
  BadIdent(String, Span<'buf>),
  BadString(String, Span<'buf>),
  BadNumber(ParseFloatError, Span<'buf>),
  Unexpected(String, Span<'buf>),
}

impl TokenizeError<'_> {
  /// Creates an unexpected character error.
  pub fn unexpected_char(span: Span<'_>) -> TokenizeError<'_> {
    TokenizeError::Unexpected("Unexpected character".to_string(), span)
  }

  /// Creates a bad identifier error.
  pub fn bad_ident_numeric_before_alpha(span: Span<'_>) -> TokenizeError<'_> {
    TokenizeError::BadIdent(
      "Invalid variable name, expected alphabetic or `_` before numeric".to_string(),
      span,
    )
  }

  /// Creates a bad string error.
  pub fn bad_string_unexpected_eof(span: Span<'_>) -> TokenizeError<'_> {
    TokenizeError::BadString(
      "Invalid string, expected closing quote, got end of file".to_string(),
      span,
    )
  }

  /// Creates a bad string error.
  pub fn bad_string_unexpected_eol(span: Span<'_>) -> TokenizeError<'_> {
    TokenizeError::BadString(
      "Invalid string, expected closing quote, got end of line".to_string(),
      span,
    )
  }
}

impl Display for TokenizeError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for TokenizeError<'_> {}
