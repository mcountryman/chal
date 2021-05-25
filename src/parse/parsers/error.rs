use crate::parse::{Span, Token, TokenizeError};
use std::{error::Error, fmt::Display};

pub type ParseResult<'buf, T> = Result<T, ParseError<'buf>>;

/// An error which can be returned when tokenizing a str.
#[derive(Debug, Clone)]
pub enum ParseError<'buf> {
  Tokenize(TokenizeError<'buf>),
  Unexpected(String, Span<'buf>),
  MissingToken(String, Span<'buf>),
  MissingIdent(String, Span<'buf>),
  EmptyExpression(String, Span<'buf>),
}

impl ParseError<'_> {
  pub fn expected_left_paren(span: Span<'_>) -> ParseError<'_> {
    ParseError::MissingToken("Missing open delimiter".to_string(), span)
  }

  pub fn expected_right_paren(span: Span<'_>) -> ParseError<'_> {
    ParseError::MissingToken("Missing closing delimiter".to_string(), span)
  }

  pub fn empty_expression_eof(span: Span<'_>) -> ParseError<'_> {
    ParseError::EmptyExpression("Expected expression got end of file".to_string(), span)
  }

  pub fn unexpected_token(token: Token<'_>) -> ParseError<'_> {
    ParseError::Unexpected("Unexpected token".to_string(), token.span)
  }

  pub fn expected_ident(span: Span<'_>) -> ParseError<'_> {
    ParseError::MissingIdent("Expected variable identifier".to_string(), span)
  }
}

impl<'buf> From<TokenizeError<'buf>> for ParseError<'buf> {
  fn from(inner: TokenizeError<'buf>) -> Self {
    ParseError::Tokenize(inner)
  }
}

impl Display for ParseError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for ParseError<'_> {}
