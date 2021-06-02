use crate::parse::{Span, Token, TokenizeError};
use std::{error::Error, fmt::Display};

pub type ParseResult<'buf, T> = Result<T, ParseError<'buf>>;

/// An error which can be returned when tokenizing a str.
#[derive(Clone)]
pub enum ParseError<'buf> {
  Tokenize(TokenizeError<'buf>),
  Unexpected(String, Span<'buf>),
  UnexpectedToken(String, Token<'buf>),
  Missing(String, Span<'buf>),
  EmptyExpression(String, Span<'buf>),
}

impl<'buf> ParseError<'buf> {
  pub fn expected_left_paren(span: &Span<'buf>) -> Self {
    Self::Missing("Missing open delimiter".to_string(), span.clone())
  }

  pub fn expected_right_paren(span: &Span<'buf>) -> Self {
    Self::Missing("Missing closing delimiter".to_string(), span.clone())
  }

  pub fn empty_expression_eof(span: &Span<'buf>) -> Self {
    Self::EmptyExpression(
      "Expected expression got end of file".to_string(),
      span.clone(),
    )
  }

  pub fn unexpected_token(token: &Token<'buf>) -> Self {
    Self::UnexpectedToken("Unexpected token".to_string(), token.clone())
  }

  pub fn unexpected_eof(span: &Span<'buf>) -> Self {
    Self::Unexpected("Unexpected end of file".to_string(), span.clone())
  }

  pub fn expected_ident(span: &Span<'buf>) -> Self {
    Self::Missing("Expected variable identifier".to_string(), span.clone())
  }

  pub fn expected_if_condition(span: &Span<'buf>) -> Self {
    Self::Missing("Expected if condition".to_string(), span.clone())
  }

  pub fn expected_if_body(span: &Span<'buf>) -> Self {
    Self::Missing("Expected if body".to_string(), span.clone())
  }

  pub fn expected_var_expr(span: &Span<'buf>) -> Self {
    Self::Missing("Expected variable expression".to_string(), span.clone())
  }

  pub fn expected_func_body(span: &Span<'buf>) -> Self {
    Self::Missing("Expected function body".to_string(), span.clone())
  }

  pub fn expected_op_lhs(span: &Span<'buf>) -> Self {
    Self::Missing("Expected operator lhs".to_string(), span.clone())
  }

  pub fn expected_op_rhs(span: &Span<'buf>) -> Self {
    Self::Missing("Expected operator lhs".to_string(), span.clone())
  }

  pub fn expected_op_operand(span: &Span<'buf>) -> Self {
    Self::Missing("Expected operator lhs".to_string(), span.clone())
  }
}

impl<'buf> From<TokenizeError<'buf>> for ParseError<'buf> {
  fn from(inner: TokenizeError<'buf>) -> Self {
    ParseError::Tokenize(inner)
  }
}

impl std::fmt::Debug for ParseError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParseError::Tokenize(err) => write!(f, "{:?}", err),
      ParseError::Unexpected(message, span) => write!(f, "{} at {:?}", message, span),
      ParseError::UnexpectedToken(message, token) => {
        write!(f, "{} `{:?}` at {:?}", message, token.1, token.0)
      }
      ParseError::Missing(message, span) => write!(f, "{} at {:?}", message, span),
      ParseError::EmptyExpression(message, span) => write!(f, "{} at {:?}", message, span),
    }
  }
}

impl std::fmt::Display for ParseError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for ParseError<'_> {}
