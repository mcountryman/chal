use crate::parse::Span;
use std::{
  borrow::Cow,
  fmt::{Display, Formatter},
  ops::Deref,
};

/// Contains token type, parsed token data and, a span reference to source.
#[derive(Clone)]
pub struct Token<'buf> {
  pub span: Span<'buf>,
  pub kind: TokenKind<'buf>,
}

impl Token<'_> {
  /// Returns `true` if the token is [`TokenKind::LParen`]
  pub fn is_left_paren(&self) -> bool {
    matches!(self.kind, TokenKind::LParen)
  }

  /// Returns `true` if the token is [`TokenKind::RParen`]
  pub fn is_right_paren(&self) -> bool {
    matches!(self.kind, TokenKind::RParen)
  }

  /// Returns `true` if the token is [`TokenKind::String`]
  pub fn is_string(&self) -> bool {
    matches!(self.kind, TokenKind::String(_))
  }

  /// Returns `true` if the token is [`TokenKind::Number`]
  pub fn is_number(&self) -> bool {
    matches!(self.kind, TokenKind::Number(_))
  }

  /// Returns `true` if the token is [`TokenKind::Var`]
  pub fn is_var(&self) -> bool {
    matches!(self.kind, TokenKind::Var(_))
  }

  /// Returns `true` if the token is [`TokenKind::Ident`]
  pub fn is_ident(&self) -> bool {
    matches!(self.kind, TokenKind::Ident(_))
  }

  /// Returns `true` if the token is [`TokenKind::Add`]
  pub fn is_add(&self) -> bool {
    matches!(self.kind, TokenKind::Add)
  }

  /// Returns `true` if the token is [`TokenKind::Sub`]
  pub fn is_sub(&self) -> bool {
    matches!(self.kind, TokenKind::Sub)
  }

  /// Returns `true` if the token is [`TokenKind::Div`]
  pub fn is_div(&self) -> bool {
    matches!(self.kind, TokenKind::Div)
  }

  /// Returns `true` if the token is [`TokenKind::Mul`]
  pub fn is_mul(&self) -> bool {
    matches!(self.kind, TokenKind::Mul)
  }

  /// Returns `true` if the token is [`TokenKind::Pow`]
  pub fn is_pow(&self) -> bool {
    matches!(self.kind, TokenKind::Pow)
  }

  /// Returns `true` if the token is [`TokenKind::Mod`]
  pub fn is_mod(&self) -> bool {
    matches!(self.kind, TokenKind::Mod)
  }

  /// Returns `true` if the token is [`TokenKind::AddInc`]
  pub fn is_add_inc(&self) -> bool {
    matches!(self.kind, TokenKind::AddInc)
  }

  /// Returns `true` if the token is [`TokenKind::SubInc`]
  pub fn is_sub_inc(&self) -> bool {
    matches!(self.kind, TokenKind::SubInc)
  }

  /// Returns `true` if the token is [`TokenKind::BOr`]
  pub fn is_binary_or(&self) -> bool {
    matches!(self.kind, TokenKind::BOr)
  }

  /// Returns `true` if the token is [`TokenKind::BNot`]
  pub fn is_binary_not(&self) -> bool {
    matches!(self.kind, TokenKind::BNot)
  }

  /// Returns `true` if the token is [`TokenKind::BAnd`]
  pub fn is_binary_and(&self) -> bool {
    matches!(self.kind, TokenKind::BAnd)
  }

  /// Returns `true` if the token is [`TokenKind::BLShift`]
  pub fn is_left_shift(&self) -> bool {
    matches!(self.kind, TokenKind::BLShift)
  }

  /// Returns `true` if the token is [`TokenKind::BRShift`]
  pub fn is_right_shift(&self) -> bool {
    matches!(self.kind, TokenKind::BRShift)
  }

  /// Returns `true` if the token is [`TokenKind::Lt`]
  pub fn is_lt(&self) -> bool {
    matches!(self.kind, TokenKind::Lt)
  }

  /// Returns `true` if the token is [`TokenKind::LtEq`]
  pub fn is_lt_eq(&self) -> bool {
    matches!(self.kind, TokenKind::LtEq)
  }

  /// Returns `true` if the token is [`TokenKind::Gt`]
  pub fn is_gt(&self) -> bool {
    matches!(self.kind, TokenKind::Gt)
  }

  /// Returns `true` if the token is [`TokenKind::GtEq`]
  pub fn is_gt_eq(&self) -> bool {
    matches!(self.kind, TokenKind::GtEq)
  }
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
}

impl std::fmt::Debug for Token<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Token({:?} @ {:?})", &self.kind, self.span)
  }
}

/// The kind of token and relevant metadata.
#[derive(Clone, PartialEq, PartialOrd)]
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

impl ToString for TokenKind<'_> {
  fn to_string(&self) -> String {
    match self {
      TokenKind::LParen => "(".to_string(),
      TokenKind::RParen => ")".to_string(),
      TokenKind::String(inner) => format!("\"{}\"", inner),
      TokenKind::Number(inner) => inner.to_string(),
      TokenKind::Var(inner) => format!("${}", inner),
      TokenKind::Ident(inner) => inner.to_string(),
      TokenKind::Add => "+".to_string(),
      TokenKind::Sub => "-".to_string(),
      TokenKind::Div => "/".to_string(),
      TokenKind::Mul => "*".to_string(),
      TokenKind::Pow => "^".to_string(),
      TokenKind::Mod => "$".to_string(),
      TokenKind::AddInc => "++".to_string(),
      TokenKind::SubInc => "--".to_string(),
      TokenKind::BOr => "|".to_string(),
      TokenKind::BNot => "^".to_string(),
      TokenKind::BAnd => "&".to_string(),
      TokenKind::BLShift => "<<".to_string(),
      TokenKind::BRShift => ">>".to_string(),
      TokenKind::Lt => "<".to_string(),
      TokenKind::LtEq => "<=".to_string(),
      TokenKind::Gt => ">".to_string(),
      TokenKind::GtEq => ">=".to_string(),
    }
  }
}

impl std::fmt::Debug for TokenKind<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_string())
  }
}
