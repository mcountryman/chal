use std::{error::Error, fmt::Display};

use super::Span;

pub type ParseResult<'buf, T> = Result<T, ParseError<'buf>>;

#[derive(Debug, Clone)]
pub enum ParseError<'buf> {
  BadVarName(String, Span<'buf>),
  Unexpected(String, Span<'buf>),
}

impl ParseError<'_> {
  pub fn unexpected_char<'buf>(message: &str, span: Span<'buf>) -> ParseError<'buf> {
    ParseError::Unexpected(message.to_string(), span)
  }

  pub fn bad_var_name_numeric_before_alpha<'buf>(span: Span<'buf>) -> ParseError<'buf> {
    ParseError::BadVarName(
      "Invalid variable name, expected alphabetic or `_` before numeric".to_string(),
      span,
    )
  }
}

impl Display for ParseError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for ParseError<'_> {}
