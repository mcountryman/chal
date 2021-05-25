use crate::parse::Span;
use std::{borrow::Cow, ops::Deref};

#[derive(Debug, Clone)]
pub struct Token<'buf> {
  span: Span<'buf>,
  kind: TokenKind<'buf>,
}

impl<'buf> Deref for Token<'buf> {
  type Target = Span<'buf>;

  fn deref(&self) -> &Self::Target {
    &self.span
  }
}

impl<'buf> Token<'buf> {
  pub fn new(span: Span<'buf>, kind: TokenKind<'buf>) -> Self {
    Self { span, kind }
  }

  pub fn span(&self) -> &Span<'buf> {
    &self.span
  }

  pub fn kind(&self) -> &TokenKind<'buf> {
    &self.kind
  }
}

#[derive(Debug, Clone)]
pub enum TokenKind<'buf> {
  LParen,
  RParen,

  String(Cow<'buf, str>),
  Number(f64),

  Var(&'buf str),
  Ident(&'buf str),

  // Arithmetic
  Add,
  Sub,
  Div,
  Mul,
  Pow,
  Mod,

  // Compound arithmetic
  AddInc,
  SubInc,

  // Binary
  BOr,
  BNot,
  BAnd,
  BLShift,
  BRShift,

  // Logical
  Lt,
  LtEq,
  Gt,
  GtEq,
}
