pub mod chars;
pub mod error;
pub mod token;

pub use chars::*;
pub use error::*;
pub use token::*;

use super::{Position, Span};
use std::{borrow::Cow, iter::Peekable};

/// An iterator over the tokens of a str.
///
/// # Lifetimes
/// * `'buf` - The lifetime of the source buffer.
#[derive(Debug, Clone)]
pub struct Tokenizer<'buf> {
  buf: &'buf str,
  chars: Peekable<TokenizerChars<'buf>>,
}

impl<'buf> Tokenizer<'buf> {
  /// Create new instance of tokenizer.
  ///
  /// # Arguments
  /// * `buf` - The source buffer to tokenize.
  pub fn new(buf: &'buf str) -> Self {
    Self {
      buf,
      chars: TokenizerChars::new(buf).peekable(),
    }
  }

  fn span_at(&mut self, beg: Position) -> Span<'buf> {
    match self.chars.peek() {
      Some((end, _)) => Span::new(beg, *end, self.buf),
      None => Span::new(beg, beg, self.buf),
    }
  }

  fn eat_whitespace_and_comments(&mut self) {
    loop {
      match self.chars.peek() {
        Some((_, '#')) => 'comment: loop {
          match self.chars.next() {
            Some((_, '\n')) => break 'comment,
            None => break 'comment,
            _ => {}
          }
        },
        Some((_, x)) if x.is_whitespace() => {
          self.chars.next();
        }
        _ => break,
      }
    }
  }

  /// Consume var or identifier token metadata.
  ///
  /// # Arguments
  /// * `beg` - The position before token starts (used for marking locations in errors)
  /// * `has_alpha_or_underscore` - If prior chars are alphabetic or underscore.
  fn eat_ident(
    &mut self,
    beg: Position,
    mut has_alpha_or_underscore: bool,
  ) -> TokenizeResult<'buf, &'buf str> {
    loop {
      match self.chars.peek() {
        Some((_, ch)) if ch.is_alphabetic() || *ch == '_' => {
          has_alpha_or_underscore = true;
          self.chars.next();
        }
        Some((pos, ch)) if ch.is_numeric() => {
          // If we encounter a numeric character before an alphanumeric or underscore char
          // we indicate the variable is invalid.
          if !has_alpha_or_underscore {
            return Err(TokenizeError::bad_ident_numeric_before_alpha(Span::new(
              beg, *pos, self.buf,
            )));
          }

          self.chars.next();
        }
        Some((end, _)) => return Ok(&self.buf[beg.offset..end.offset]),
        None => return Ok(&self.buf[beg.offset..]),
      }
    }
  }

  /// Consume the rest of a string literal.
  ///
  /// # Arguments
  /// * `beg` - The position before token starts (used for marking locations in errors)
  /// * `quote` - The opening quote character.
  fn eat_string(&mut self, beg: Position, quote: char) -> TokenizeResult<'buf, Cow<'buf, str>> {
    let pos_pre_quote = beg;
    let beg = beg.extend(quote);

    loop {
      match self.chars.peek() {
        Some((end, ch)) if *ch == quote => {
          let inner = Cow::from(&self.buf[beg.offset..end.offset]);

          // Consume quote
          self.chars.next();

          return Ok(inner);
        }
        Some((_, '\n')) => {
          return Err(TokenizeError::bad_string_unexpected_eol(
            self.span_at(pos_pre_quote),
          ))
        }
        Some(_) => {
          self.chars.next();
        }
        None => {
          return Err(TokenizeError::bad_string_unexpected_eof(
            self.span_at(pos_pre_quote),
          ))
        }
      }
    }
  }

  /// Consume the rest of the number token.
  ///
  /// # Arguments
  /// * `beg` - The position before token starts (used for marking locations in errors)
  fn eat_number(&mut self, beg: Position) -> TokenizeResult<'buf, f64> {
    // Consume numeric characters and decimal characters.
    let mut eat_number = || loop {
      match self.chars.peek() {
        Some((_, ch)) if ch.is_numeric() || *ch == '.' => {
          self.chars.next();
        }
        Some((end, _)) => return &self.buf[beg.offset..end.offset],
        None => return &self.buf[beg.offset..],
      }
    };

    // Get buffer slice for number
    let raw = eat_number();
    // Parse float
    match raw.parse::<f64>() {
      Ok(num) => Ok(num),
      Err(err) => Err(TokenizeError::BadNumber(err, self.span_at(beg))),
    }
  }

  /// Consume next token and return.
  fn next_token(&mut self) -> TokenizeResult<'buf, Option<Token<'buf>>> {
    // Position before token start
    let (beg, kind) = match self.chars.next() {
      // Prioritize parens
      Some((pos, '(')) => (pos, TokenKind::LParen),
      Some((pos, ')')) => (pos, TokenKind::RParen),

      // Ident
      Some((pos, '$')) => (pos, TokenKind::Var(self.eat_ident(pos, false)?)),
      // Ident
      Some((pos, ch)) if ch.is_alphabetic() || ch == '_' => {
        (pos, TokenKind::Ident(self.eat_ident(pos, true)?))
      }
      // String
      Some((pos, ch)) if ch == '"' || ch == '\'' => {
        (pos, TokenKind::String(self.eat_string(pos, ch)?))
      }
      // Number
      Some((pos, ch)) if ch.is_numeric() => (pos, TokenKind::Number(self.eat_number(pos)?)),

      // Simple operators
      Some((pos, '*')) => (pos, TokenKind::Mul),
      Some((pos, '/')) => (pos, TokenKind::Div),
      Some((pos, '^')) => (pos, TokenKind::Pow),
      Some((pos, '%')) => (pos, TokenKind::Mod),
      Some((pos, '|')) => (pos, TokenKind::BOr),
      Some((pos, '&')) => (pos, TokenKind::BAnd),
      Some((pos, '!')) => (pos, TokenKind::BNot),

      // Complex operators
      Some((pos, '+')) => (
        pos,
        match self.chars.peek() {
          Some((_, '+')) => {
            // Consume peeked `+`
            self.chars.next();
            TokenKind::AddInc
          }
          _ => TokenKind::Add,
        },
      ),
      Some((pos, '-')) => (
        pos,
        match self.chars.peek() {
          Some((_, '-')) => {
            // Consume peeked `-`
            self.chars.next();
            TokenKind::SubInc
          }
          _ => TokenKind::Sub,
        },
      ),
      Some((pos, '<')) => (
        pos,
        match self.chars.peek() {
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
      ),
      Some((pos, '>')) => (
        pos,
        match self.chars.peek() {
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
      ),

      // Handle unexpected character
      Some((pos, ch)) => {
        let span = Span::new(pos, pos.extend(ch), self.buf);
        let error = TokenizeError::unexpected_char(span);

        return Err(error);
      }

      // Handle end of stream
      None => return Ok(None),
    };

    Ok(Some(kind.into_token(self.span_at(beg))))
  }
}

impl<'buf> Iterator for Tokenizer<'buf> {
  type Item = TokenizeResult<'buf, Token<'buf>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.eat_whitespace_and_comments();
    self.next_token().transpose()
  }
}

#[cfg(test)]
mod tests {
  use super::Tokenizer;
  use crate::parse::{Position, TokenizeError};

  #[test]
  pub fn test_eat_whitespace_end_at_non_whitespace() {
    let mut tokenizer = Tokenizer::new("  \t\r\n!");
    tokenizer.eat_whitespace_and_comments();

    // Check last character in buffer
    assert_eq!(tokenizer.chars.next().unwrap().1, '!');
  }

  #[test]
  pub fn test_eat_whitespace_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("  \t\r\n");
    tokenizer.eat_whitespace_and_comments();
  }

  #[test]
  pub fn test_eat_comment_end_at_non_comment() {
    let mut tokenizer = Tokenizer::new("# This is a comment\n# This is another comment\n!");
    tokenizer.eat_whitespace_and_comments();

    // Check last character in buffer
    assert_eq!(tokenizer.chars.next().unwrap().1, '!');
  }

  #[test]
  pub fn test_eat_comment_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("# This is a comment\n# This is another comment\n");
    tokenizer.eat_whitespace_and_comments();

    // Check last character in buffer
    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_whitespace_and_comments() {
    let mut tokenizer =
      Tokenizer::new("# This is a comment\n  # This is another comment\n   \r\t!");
    tokenizer.eat_whitespace_and_comments();

    // Check last character in buffer
    assert_eq!(tokenizer.chars.next().unwrap().1, '!');
  }

  #[test]
  pub fn test_eat_ident_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("$aeiöu_0123");
    // Consume leading `$` character
    assert_eq!(tokenizer.chars.next().unwrap().1, '$');
    // Consume var ident
    let var = tokenizer
      .eat_ident(Position::default().extend('$'), false)
      .unwrap();

    assert_eq!(var, "aeiöu_0123");
    // Check last character in buffer
    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_ident_end_at_non_var() {
    let mut tokenizer = Tokenizer::new("$aeiöu_0123!");
    // Consume leading `$` character
    assert_eq!(tokenizer.chars.next().unwrap().1, '$');
    // Consume var ident
    let var = tokenizer
      .eat_ident(Position::default().extend('$'), false)
      .unwrap();
    assert_eq!(tokenizer.chars.next().unwrap().1, '!');

    // Check last character in buffer
    assert_eq!(var, "aeiöu_0123");
  }

  #[test]
  pub fn test_eat_ident_has_alpha_or_underscore_fail() {
    let mut tokenizer = Tokenizer::new("0");
    // Consume var ident
    let var = tokenizer.eat_ident(Position::default(), false);
    match var {
      Err(TokenizeError::BadIdent(..)) => {}
      _ => panic!("Expected `TokenizeError::BadIdent(..)`"),
    };

    assert_eq!(tokenizer.chars.next().unwrap().1, ('0'));
  }

  #[test]
  pub fn test_eat_string_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("\"This is a string\"");
    let beg = Position::default();

    assert_eq!(tokenizer.chars.next().unwrap().1, ('"'));
    assert_eq!(tokenizer.eat_string(beg, '"').unwrap(), "This is a string");
    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_string_unexpected_eof() {
    let mut tokenizer = Tokenizer::new("'This is a bad string");
    let beg = Position::default();

    assert_eq!(tokenizer.chars.next().unwrap().1, ('\''));

    match tokenizer.eat_string(beg, '\'') {
      Err(TokenizeError::BadString(..)) => {}
      _ => panic!("Expected `TokenizeError::BadString(..)`"),
    };

    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_string_unexpected_eol() {
    let mut tokenizer = Tokenizer::new("'This is a bad string\n'");
    let beg = Position::default();

    assert_eq!(tokenizer.chars.next().unwrap().1, ('\''));

    match tokenizer.eat_string(beg, '\'') {
      Err(TokenizeError::BadString(..)) => {}
      _ => panic!("Expected `TokenizeError::BadString(..)`"),
    };

    assert_eq!(tokenizer.chars.next().unwrap().1, ('\n'));
  }

  #[test]
  #[allow(clippy::float_cmp)]
  pub fn test_eat_number_floating() {
    let mut tokenizer = Tokenizer::new("1337.60");
    let beg = Position::default();

    assert_eq!(tokenizer.chars.next().unwrap().1, ('1'));
    assert_eq!(tokenizer.eat_number(beg).unwrap(), 1337.60f64);
  }

  #[test]
  #[allow(clippy::float_cmp)]
  pub fn test_eat_number_whole() {
    let mut tokenizer = Tokenizer::new("69420");
    let beg = Position::default();

    assert_eq!(tokenizer.chars.next().unwrap().1, ('6'));
    assert_eq!(tokenizer.eat_number(beg).unwrap(), 69420f64);
  }

  #[test]
  pub fn test_eat_number_bad() {
    let mut tokenizer = Tokenizer::new("694.2.0");
    let beg = Position::default();

    assert_eq!(tokenizer.chars.next().unwrap().1, ('6'));

    match tokenizer.eat_number(beg) {
      Err(TokenizeError::BadNumber(..)) => {}
      _ => panic!("Expected `TokenizeError::BadNumber(..)`"),
    };

    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_tokenize_errors_chal() {
    Tokenizer::new(include_str!("../../../data/errors.chal"))
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
  }

  #[test]
  pub fn test_tokenize_fizzbuzz_chal() {
    Tokenizer::new(include_str!("../../../data/fizzbuzz.chal"))
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
  }

  #[test]
  pub fn test_tokenize_math_chal() {
    Tokenizer::new(include_str!("../../../data/math.chal"))
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
  }

  #[test]
  pub fn test_tokenize_recursion_chal() {
    Tokenizer::new(include_str!("../../../data/recursion.chal"))
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
  }

  #[test]
  pub fn test_tokenize_string_chal() {
    Tokenizer::new(include_str!("../../../data/string.chal"))
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
  }

  #[test]
  pub fn test_tokenize_whitespace_chal() {
    Tokenizer::new(include_str!("../../../data/whitespace.chal"))
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
  }

  #[test]
  #[cfg_attr(miri, ignore)]
  pub fn test_tokenize_stress() {
    let merged = concat!(
      include_str!("../../../data/errors.chal"),
      include_str!("../../../data/fizzbuzz.chal"),
      include_str!("../../../data/math.chal"),
      include_str!("../../../data/recursion.chal"),
      include_str!("../../../data/string.chal"),
      include_str!("../../../data/whitespace.chal"),
    )
    .repeat(1_000);

    Tokenizer::new(&merged)
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
  }
}
