use crate::parse::{Position, Span};
use std::{borrow::Cow, fmt::Formatter, ops::Deref};

/// Contains token type, parsed token data and, a span reference to source.
#[derive(Clone)]
pub struct Token<'buf>(pub Span<'buf>, pub TokenKind<'buf>);

impl<'buf> Token<'buf> {
  /// Returns `true` if the token is [`TokenKind::LParen`]
  pub fn is_left_paren(&self) -> bool {
    matches!(self.1, TokenKind::LParen)
  }

  /// Returns `true` if the token is [`TokenKind::RParen`]
  pub fn is_right_paren(&self) -> bool {
    matches!(self.1, TokenKind::RParen)
  }

  /// Returns `true` if the token is [`TokenKind::String`]
  pub fn is_string(&self) -> bool {
    matches!(self.1, TokenKind::String(_))
  }

  /// Returns `true` if the token is [`TokenKind::Number`]
  pub fn is_number(&self) -> bool {
    matches!(self.1, TokenKind::Number(_))
  }

  /// Returns `true` if the token is [`TokenKind::Var`]
  pub fn is_var(&self) -> bool {
    matches!(self.1, TokenKind::Var(_))
  }

  /// Returns `true` if the token is [`TokenKind::Ident`]
  pub fn is_ident(&self) -> bool {
    matches!(self.1, TokenKind::Ident(_))
  }

  /// Returns `true` if the token is [`TokenKind::Add`]
  pub fn is_add(&self) -> bool {
    matches!(self.1, TokenKind::Add)
  }

  /// Returns `true` if the token is [`TokenKind::Sub`]
  pub fn is_sub(&self) -> bool {
    matches!(self.1, TokenKind::Sub)
  }

  /// Returns `true` if the token is [`TokenKind::Div`]
  pub fn is_div(&self) -> bool {
    matches!(self.1, TokenKind::Div)
  }

  /// Returns `true` if the token is [`TokenKind::Mul`]
  pub fn is_mul(&self) -> bool {
    matches!(self.1, TokenKind::Mul)
  }

  /// Returns `true` if the token is [`TokenKind::Pow`]
  pub fn is_pow(&self) -> bool {
    matches!(self.1, TokenKind::Pow)
  }

  /// Returns `true` if the token is [`TokenKind::Mod`]
  pub fn is_mod(&self) -> bool {
    matches!(self.1, TokenKind::Mod)
  }

  /// Returns `true` if the token is [`TokenKind::AddInc`]
  pub fn is_add_inc(&self) -> bool {
    matches!(self.1, TokenKind::AddInc)
  }

  /// Returns `true` if the token is [`TokenKind::SubInc`]
  pub fn is_sub_inc(&self) -> bool {
    matches!(self.1, TokenKind::SubInc)
  }

  /// Returns `true` if the token is [`TokenKind::BOr`]
  pub fn is_binary_or(&self) -> bool {
    matches!(self.1, TokenKind::BOr)
  }

  /// Returns `true` if the token is [`TokenKind::BNot`]
  pub fn is_binary_not(&self) -> bool {
    matches!(self.1, TokenKind::BNot)
  }

  /// Returns `true` if the token is [`TokenKind::BAnd`]
  pub fn is_binary_and(&self) -> bool {
    matches!(self.1, TokenKind::BAnd)
  }

  /// Returns `true` if the token is [`TokenKind::BLShift`]
  pub fn is_left_shift(&self) -> bool {
    matches!(self.1, TokenKind::BLShift)
  }

  /// Returns `true` if the token is [`TokenKind::BRShift`]
  pub fn is_right_shift(&self) -> bool {
    matches!(self.1, TokenKind::BRShift)
  }

  /// Returns `true` if the token is [`TokenKind::Lt`]
  pub fn is_lt(&self) -> bool {
    matches!(self.1, TokenKind::Lt)
  }

  /// Returns `true` if the token is [`TokenKind::LtEq`]
  pub fn is_lt_eq(&self) -> bool {
    matches!(self.1, TokenKind::LtEq)
  }

  /// Returns `true` if the token is [`TokenKind::Gt`]
  pub fn is_gt(&self) -> bool {
    matches!(self.1, TokenKind::Gt)
  }

  /// Returns `true` if the token is [`TokenKind::GtEq`]
  pub fn is_gt_eq(&self) -> bool {
    matches!(self.1, TokenKind::GtEq)
  }
}

impl<'buf> Token<'buf> {
  /// Create token.
  ///
  /// # Arguments
  /// * `span` - The span.
  /// * `kind` - The token kind.
  pub fn new(span: Span<'buf>, kind: TokenKind<'buf>) -> Self {
    Self(span, kind)
  }
}

impl std::fmt::Debug for Token<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Token({:?} @ {:?})", &self.1, self.0)
  }
}

/// The kind of token and relevant metadata.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

impl<'buf> TokenKind<'buf> {
  pub fn into_token(self, span: Span<'buf>) -> Token<'buf> {
    Token::new(span, self)
  }
}

impl std::fmt::Display for TokenKind<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenKind::LParen => write!(f, "("),
      TokenKind::RParen => write!(f, ")"),
      TokenKind::String(inner) => write!(f, "\"{}\"", inner),
      TokenKind::Number(inner) => write!(f, "{}", inner),
      TokenKind::Var(inner) => write!(f, "${}", inner),
      TokenKind::Ident(inner) => write!(f, "{}", inner),
      TokenKind::Add => write!(f, "+"),
      TokenKind::Sub => write!(f, "-"),
      TokenKind::Div => write!(f, "/"),
      TokenKind::Mul => write!(f, "*"),
      TokenKind::Pow => write!(f, "^"),
      TokenKind::Mod => write!(f, "$"),
      TokenKind::AddInc => write!(f, "++"),
      TokenKind::SubInc => write!(f, "--"),
      TokenKind::BOr => write!(f, "|"),
      TokenKind::BNot => write!(f, "^"),
      TokenKind::BAnd => write!(f, "&"),
      TokenKind::BLShift => write!(f, "<<"),
      TokenKind::BRShift => write!(f, ">>"),
      TokenKind::Lt => write!(f, "<"),
      TokenKind::LtEq => write!(f, "<="),
      TokenKind::Gt => write!(f, ">"),
      TokenKind::GtEq => write!(f, ">="),
    }
  }
}
