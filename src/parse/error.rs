use crate::parse::{Span, Token, TokenizeError};
use std::{error::Error, fmt::Display};

pub type ParseResult<'buf, T> = Result<T, ParseError<'buf>>;

/// An error which can be returned when tokenizing a str.
#[derive(Debug, Clone)]
pub enum ParseError<'buf> {
  Tokenize(TokenizeError<'buf>),
  Unexpected(String, Span<'buf>),
  UnexpectedToken(String, Token<'buf>),
  Missing(String, Span<'buf>),
  EmptyExpression(String, Span<'buf>),
}

impl ParseError<'_> {
  pub fn expected_left_paren(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Missing open delimiter".to_string(), span)
  }

  pub fn expected_right_paren(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Missing closing delimiter".to_string(), span)
  }

  pub fn empty_expression_eof(span: Span<'_>) -> ParseError<'_> {
    ParseError::EmptyExpression("Expected expression got end of file".to_string(), span)
  }

  pub fn unexpected_token(token: Token<'_>) -> ParseError<'_> {
    ParseError::UnexpectedToken("Unexpected token".to_string(), token)
  }

  pub fn unexpected_eof(span: Span<'_>) -> ParseError<'_> {
    ParseError::Unexpected("Unexpected end of file".to_string(), span)
  }

  pub fn expected_ident(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected variable identifier".to_string(), span)
  }

  pub fn expected_if_condition(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected if condition".to_string(), span)
  }

  pub fn expected_if_body(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected if body".to_string(), span)
  }

  pub fn expected_var_expr(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected variable expression".to_string(), span)
  }

  pub fn expected_func_body(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected function body".to_string(), span)
  }

  pub fn expected_op_lhs(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected operator lhs".to_string(), span)
  }

  pub fn expected_op_rhs(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected operator lhs".to_string(), span)
  }

  pub fn expected_op_operand(span: Span<'_>) -> ParseError<'_> {
    ParseError::Missing("Expected operator lhs".to_string(), span)
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
