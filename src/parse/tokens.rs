use super::{ParseResult, Position, Span};
use std::{borrow::Cow, iter::Peekable, str::CharIndices};

#[derive(Debug, Clone)]
pub struct Token<'a> {
  span: Span<'a>,
  kind: TokenKind<'a>,
}

#[derive(Debug, Clone)]
pub enum TokenKind<'a> {
  LParen,
  RParen,

  String(Cow<'a, str>),
  Number(f64),

  Ident(&'a str),
  Operator(Operator),
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
  Add,
  Sub,
  Div,
  Mul,
  Pow,
  Mod,

  AddInc,
  SubInc,

  BOr,
  BNot,
  BAnd,
  BLShift,
  BRShift,
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
  buf: &'a str,
  chars: Peekable<CharIndices<'a>>,
  position: Position,
}

impl<'a> Tokenizer<'a> {
  pub fn new(buf: &'a str) -> Self {
    Self {
      buf,
      chars: buf.char_indices().peekable(),
      position: Position::default(),
    }
  }
}

impl<'a> Iterator for Tokenizer<'a> {
  type Item = ParseResult<Token<'a>>;

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}
