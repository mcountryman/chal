use crate::parse::Span;
use std::{borrow::Cow, ops::Deref};

/// Contains token type, parsed token data and, a span reference to source.
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
  /// Create token.
  ///
  /// # Arguments
  /// * `span` - The span.
  /// * `kind` - The token kind.
  pub fn new(span: Span<'buf>, kind: TokenKind<'buf>) -> Self {
    Self { span, kind }
  }

  /// Get token span.
  pub fn span(&self) -> &Span<'buf> {
    &self.span
  }

  /// Get token kind.
  pub fn kind(&self) -> &TokenKind<'buf> {
    &self.kind
  }
}

/// The kind of token and relevant metadata.
#[derive(Debug, Clone)]
pub enum TokenKind<'buf> {
  /// Left parenthesis
  LParen,
  /// Right parenthesis
  RParen,

  /// String literal
  String(Cow<'buf, str>),
  /// Number literal
  Number(f64),

  /// User defined variable
  Var(&'buf str),
  /// System defined identifier
  Ident(&'buf str),

  /// Add operator
  Add,
  /// Subtract operator
  Sub,
  /// Divide operator
  Div,
  /// Multiply operator
  Mul,
  /// Power operator
  Pow,
  /// Modulo operator
  Mod,

  /// Incremental add operator
  AddInc,
  /// Incremental subtract operator
  SubInc,

  /// Binary or operator
  BOr,
  /// Binary not operator
  BNot,
  /// Binary and operator
  BAnd,
  /// Left shift operator
  BLShift,
  /// Right shift operator
  BRShift,

  /// Less than operator
  Lt,
  /// Less than equal to operator
  LtEq,
  /// Greater than operator
  Gt,
  /// Greater than equal to operator
  GtEq,
}
