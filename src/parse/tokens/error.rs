use super::Span;
use std::{error::Error, fmt::Display, num::ParseFloatError};

pub type TokenizeResult<'buf, T> = Result<T, TokenizeError<'buf>>;

#[derive(Debug, Clone)]
pub enum TokenizeError<'buf> {
  BadIdent(String, Span<'buf>),
  BadString(String, Span<'buf>),
  BadNumber(ParseFloatError, Span<'buf>),
  Unexpected(String, Span<'buf>),
}

impl TokenizeError<'_> {
  pub fn unexpected_char(span: Span<'_>) -> TokenizeError<'_> {
    TokenizeError::Unexpected("Unexpected character".to_string(), span)
  }

  pub fn bad_ident_numeric_before_alpha(span: Span<'_>) -> TokenizeError<'_> {
    TokenizeError::BadIdent(
      "Invalid variable name, expected alphabetic or `_` before numeric".to_string(),
      span,
    )
  }

  pub fn bad_string_unexpected_eos(span: Span<'_>) -> TokenizeError<'_> {
    TokenizeError::BadString(
      "Invalid string, expected closing quote, got `<EOS>`".to_string(),
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
