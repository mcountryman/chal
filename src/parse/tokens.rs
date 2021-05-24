use crate::parse::ParseError;

use super::{ParseResult, Position, Span};
use std::{
  borrow::Cow,
  iter::Peekable,
  str::{CharIndices, Chars},
};

#[derive(Debug, Clone)]
pub struct Token<'buf> {
  span: Span<'buf>,
  kind: TokenKind<'buf>,
}

impl Token<'_> {
  pub fn span(&self) -> &Span<'_> {
    &self.span
  }

  pub fn kind(&self) -> &TokenKind<'_> {
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

#[derive(Debug, Clone)]
pub struct Tokenizer<'buf> {
  buf: &'buf str,
  chars: TokenizerChars<'buf>,
}

impl<'buf> Tokenizer<'buf> {
  pub fn new(buf: &'buf str) -> Self {
    Self {
      buf,
      chars: TokenizerChars::new(buf),
    }
  }

  fn span_current(&self, beg: Position) -> Span<'buf> {
    Span::new(beg, self.chars.pos(), self.buf)
  }

  /// Consume whitespace characters until <EOS> or non-whitespace character.
  fn eat_whitespace(&mut self) {
    while self.chars.next_if(|(_, ch)| ch.is_whitespace()).is_some() {}
  }

  /// Consume var.
  fn eat_var(&mut self, beg: Position) -> ParseResult<'buf, &'buf str> {
    let mut end = beg;
    let mut has_alpha_or_underscore = false;
    'iter: loop {
      match self.chars.peek() {
        Some((_, ch)) if ch.is_alphabetic() || *ch == '_' => {
          has_alpha_or_underscore = true;
          self.chars.next();
        }
        Some((_, ch)) if ch.is_numeric() => {
          // If we encounter a numeric character before an alphanumeric or underscore char
          // we indicate the variable is invalid.
          if !has_alpha_or_underscore {
            return Err(ParseError::bad_var_name_numeric_before_alpha(
              self.span_current(beg),
            ));
          }

          self.chars.next();
        }
        _ => break 'iter,
      }
    }

    Ok(&self.chars.as_str()[beg.offset..end.offset])
  }

  /// Consume identifier excluding first character.
  fn eat_ident(&mut self, ch: (Position, char)) -> &'buf str {
    todo!()
  }

  fn eat_string(&mut self, ch: (Position, char)) -> ParseResult<'buf, Cow<'buf, str>> {
    todo!()
  }

  fn eat_number(&mut self, ch: char) -> ParseResult<'buf, f64> {
    todo!()
  }

  fn next_token(&mut self) -> ParseResult<'buf, Option<Token<'buf>>> {
    let beg = self.chars.pos();
    let kind = match self.chars.next() {
      // Prioritize parens
      Some((_, '(')) => TokenKind::LParen,
      Some((_, ')')) => TokenKind::RParen,

      // Ident
      Some((pos, '$')) => TokenKind::Ident(self.eat_var(pos)?),
      // Ident
      Some(ch) if ch.1.is_alphabetic() || ch.1 == '_' => TokenKind::Ident(self.eat_ident(ch)),
      // String
      Some((pos, '"')) => TokenKind::String(self.eat_string((pos, '"'))?),
      Some((pos, '\'')) => TokenKind::String(self.eat_string((pos, '\''))?),
      // Number
      Some((_, ch)) if ch.is_numeric() => TokenKind::Number(self.eat_number(ch)?),

      // Simple operators
      Some((_, '*')) => TokenKind::Mul,
      Some((_, '/')) => TokenKind::Div,
      Some((_, '^')) => TokenKind::Pow,
      Some((_, '%')) => TokenKind::Mod,
      Some((_, '|')) => TokenKind::BOr,
      Some((_, '&')) => TokenKind::BAnd,
      Some((_, '!')) => TokenKind::BNot,

      // Complex operators
      Some((_, '+')) => match self.chars.peek() {
        Some((_, '+')) => {
          // Consume peeked `+`
          self.chars.next();
          TokenKind::AddInc
        }
        _ => TokenKind::Add,
      },
      Some((_, '-')) => match self.chars.peek() {
        Some((_, '-')) => {
          // Consume peeked `-`
          self.chars.next();
          TokenKind::SubInc
        }
        _ => TokenKind::Sub,
      },
      Some((_, '<')) => match self.chars.peek() {
        Some((_, '<')) => {
          // Consume peeked `<`
          self.chars.next();
          TokenKind::BLShift
        }
        Some((_, '=')) => {
          // Consume peeked `=`
          self.chars.next();
          TokenKind::LtEq
        }
        _ => TokenKind::Lt,
      },
      Some((_, '>')) => match self.chars.peek() {
        Some((_, '>')) => {
          // Consume peeked `>`
          self.chars.next();
          TokenKind::BRShift
        }
        Some((_, '=')) => {
          // Consume peeked `=`
          self.chars.next();
          TokenKind::GtEq
        }
        _ => TokenKind::Gt,
      },

      // Handle unexpected character
      Some((pos, _)) => {
        let span = Span::new(beg, pos, self.buf);
        let error = ParseError::unexpected_char("Unexpected character", span);

        return Err(error);
      }

      // Handle end of stream
      None => return Ok(None),
    };

    let span = Span::new(beg, self.chars.pos(), self.buf);
    let token = Token { span, kind };

    Ok(Some(token))
  }
}

impl<'buf> Iterator for Tokenizer<'buf> {
  type Item = ParseResult<'buf, Token<'buf>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.eat_whitespace();
    self.next_token().transpose()
  }
}

#[derive(Debug, Clone)]
pub struct TokenizerChars<'buf> {
  buf: &'buf str,
  pos: Position,
  peeked: Option<Option<(Position, char)>>,
  chars: Chars<'buf>,
}

impl<'buf> TokenizerChars<'buf> {
  pub fn new(buf: &'buf str) -> Self {
    Self {
      buf,
      pos: Position::default(),
      peeked: None,
      chars: buf.chars(),
    }
  }

  pub fn pos(&self) -> Position {
    self.pos
  }

  pub fn as_str(&self) -> &'buf str {
    &self.buf
  }

  pub fn peek(&mut self) -> Option<&(Position, char)> {
    let pos = &mut self.pos;
    let chars = &mut self.chars;

    self
      .peeked
      .get_or_insert_with(|| Self::next_ch(pos, chars))
      .as_ref()
  }

  pub fn next_if(
    &mut self,
    func: impl FnOnce(&(Position, char)) -> bool,
  ) -> Option<(Position, char)> {
    match self.next() {
      Some(matched) if func(&matched) => Some(matched),
      other => {
        // Since we called `self.next()`, we consumed `self.peeked`.
        assert!(self.peeked.is_none());
        self.peeked = Some(other);
        None
      }
    }
  }

  fn next_ch(pos: &mut Position, chars: &mut Chars<'buf>) -> Option<(Position, char)> {
    let ch = chars.next();
    match ch {
      Some('\n') => {
        *pos = Position::new(pos.line + 1, 1, pos.offset + 1);
        Some((*pos, '\n'))
      }
      Some(ch) => {
        *pos = Position::new(pos.line, pos.column + 1, pos.offset + 1);
        Some((*pos, ch))
      }
      None => None,
    }
  }
}

impl Iterator for TokenizerChars<'_> {
  type Item = (Position, char);

  fn next(&mut self) -> Option<Self::Item> {
    match self.peeked.take() {
      Some(peeked) => peeked,
      None => Self::next_ch(&mut self.pos, &mut self.chars),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::parse::Position;

  use super::Tokenizer;

  #[test]
  pub fn test_eat_whitespace_end_at_non_whitespace() {
    let mut tokenizer = Tokenizer::new("  \t\r\n!");
    tokenizer.eat_whitespace();

    let (_, last) = tokenizer.chars.next().unwrap();
    assert_eq!(last, '!');
  }

  #[test]
  pub fn test_eat_whitespace_end_end_of_stream() {
    let mut tokenizer = Tokenizer::new("  \t\r\n");
    tokenizer.eat_whitespace();

    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_var() {
    let mut tokenizer = Tokenizer::new("$aeiöu_0123");
    // Consume leading `$` character
    assert_eq!(tokenizer.chars.next().unwrap().1, '$');
    // Consume var ident
    let var = tokenizer.eat_var(tokenizer.chars.pos()).unwrap();

    assert_eq!(var, "aeiöu_0123");
  }
}
