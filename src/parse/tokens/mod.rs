pub mod chars;
pub mod error;
pub mod token;

pub use chars::*;
pub use error::*;
pub use token::*;

use super::{Position, Span};
use std::borrow::Cow;

/// An iterator over the tokens of a str.
///
/// # Lifetimes
/// * `'buf` - The lifetime of the source buffer.
#[derive(Debug, Clone)]
pub struct Tokenizer<'buf> {
  buf: &'buf str,
  chars: TokenizerChars<'buf>,
}

impl<'buf> Tokenizer<'buf> {
  /// Create new instance of tokenizer.
  ///
  /// # Arguments
  /// * `buf` - The source buffer to tokenize.
  pub fn new(buf: &'buf str) -> Self {
    Self {
      buf,
      chars: TokenizerChars::new(buf),
    }
  }

  /// Get tokenizer position in source buffer.
  pub fn pos(&self) -> Position {
    self.chars.pos()
  }

  pub fn span(&self) -> Span<'buf> {
    Span::new(self.pos(), self.pos(), self.buf)
  }

  pub fn span_from(&self, beg: Position) -> Span<'buf> {
    Span::new(beg, self.pos(), self.buf)
  }

  /// Consume whitespace characters until <EOS> or non-whitespace character.
  fn eat_whitespace(&mut self) {
    while self.chars.next_if(|ch| ch.is_whitespace()).is_some() {}
  }

  fn eat_comments(&mut self) {
    while let Some('#') = self.chars.peek() {
      // Consume until `\n`
      while self.chars.next_if(|ch| *ch != '\n').is_some() {}

      // Consume `\n`
      self.chars.next();
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
        Some(ch) if ch.is_alphabetic() || *ch == '_' => {
          has_alpha_or_underscore = true;
          self.chars.next();
        }
        Some(ch) if ch.is_numeric() => {
          // If we encounter a numeric character before an alphanumeric or underscore char
          // we indicate the variable is invalid.
          if !has_alpha_or_underscore {
            return Err(TokenizeError::bad_ident_numeric_before_alpha(
              self.span_from(beg),
            ));
          }

          self.chars.next();
        }
        _ => return Ok(&self.chars.as_str()[beg.offset()..self.pos().offset()]),
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
    let beg = self.pos();

    loop {
      match self.chars.peek() {
        Some(ch) if *ch == quote => {
          let beg = beg.offset();
          let end = self.pos().offset();

          // Consume quote
          self.chars.next();

          return Ok(Cow::from(&self.chars.as_str()[beg..end]));
        }
        Some('\n') => {
          return Err(TokenizeError::bad_string_unexpected_eol(
            self.span_from(pos_pre_quote),
          ))
        }
        Some(_) => {
          self.chars.next();
        }
        None => {
          return Err(TokenizeError::bad_string_unexpected_eof(
            self.span_from(pos_pre_quote),
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
    while self
      .chars
      .next_if(|ch| ch.is_numeric() || *ch == '.')
      .is_some()
    {}

    // Get buffer slice for number
    let raw = &self.chars.as_str()[beg.offset()..self.pos().offset()];
    // Parse float
    match raw.parse::<f64>() {
      Ok(num) => Ok(num),
      Err(err) => Err(TokenizeError::BadNumber(err, self.span_from(beg))),
    }
  }

  /// Consume next token and return.
  fn next_token(&mut self) -> TokenizeResult<'buf, Option<Token<'buf>>> {
    // Position before token start
    let beg = self.pos();
    let kind = match self.chars.next() {
      // Prioritize parens
      Some('(') => TokenKind::LParen,
      Some(')') => TokenKind::RParen,

      // Ident
      Some('$') => TokenKind::Ident(self.eat_ident(self.pos(), false)?),
      // Ident
      Some(ch) if ch.is_alphabetic() || ch == '_' => TokenKind::Ident(self.eat_ident(beg, true)?),
      // String
      Some(ch) if ch == '"' || ch == '\'' => TokenKind::String(self.eat_string(beg, ch)?),
      // Number
      Some(ch) if ch.is_numeric() => TokenKind::Number(self.eat_number(beg)?),

      // Simple operators
      Some('*') => TokenKind::Mul,
      Some('/') => TokenKind::Div,
      Some('^') => TokenKind::Pow,
      Some('%') => TokenKind::Mod,
      Some('|') => TokenKind::BOr,
      Some('&') => TokenKind::BAnd,
      Some('!') => TokenKind::BNot,

      // Complex operators
      Some('+') => match self.chars.peek() {
        Some('+') => {
          // Consume peeked `+`
          self.chars.next();
          TokenKind::AddInc
        }
        _ => TokenKind::Add,
      },
      Some('-') => match self.chars.peek() {
        Some('-') => {
          // Consume peeked `-`
          self.chars.next();
          TokenKind::SubInc
        }
        _ => TokenKind::Sub,
      },
      Some('<') => match self.chars.peek() {
        Some('<') => {
          // Consume peeked `<`
          self.chars.next();
          TokenKind::BLShift
        }
        Some('=') => {
          // Consume peeked `=`
          self.chars.next();
          TokenKind::LtEq
        }
        _ => TokenKind::Lt,
      },
      Some('>') => match self.chars.peek() {
        Some('>') => {
          // Consume peeked `>`
          self.chars.next();
          TokenKind::BRShift
        }
        Some('=') => {
          // Consume peeked `=`
          self.chars.next();
          TokenKind::GtEq
        }
        _ => TokenKind::Gt,
      },

      // Handle unexpected character
      Some(_) => {
        let span = Span::new(beg, self.pos(), self.buf);
        let error = TokenizeError::unexpected_char(span);

        return Err(error);
      }

      // Handle end of stream
      None => return Ok(None),
    };

    let span = Span::new(beg, self.pos(), self.buf);
    let token = Token::new(span, kind);

    Ok(Some(token))
  }
}

impl<'buf> Iterator for Tokenizer<'buf> {
  type Item = TokenizeResult<'buf, Token<'buf>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.eat_whitespace();
    self.eat_comments();
    self.eat_whitespace();
    self.next_token().transpose()
  }
}

#[cfg(test)]
mod tests {
  use super::Tokenizer;
  use crate::parse::TokenizeError;

  #[test]
  pub fn test_eat_whitespace_end_at_non_whitespace() {
    let mut tokenizer = Tokenizer::new("  \t\r\n!");
    tokenizer.eat_whitespace();

    // Check last character in buffer
    assert_eq!(tokenizer.chars.next().unwrap(), '!');
  }

  #[test]
  pub fn test_eat_whitespace_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("  \t\r\n");
    tokenizer.eat_whitespace();
  }

  #[test]
  pub fn test_eat_comment_end_at_non_comment() {
    let mut tokenizer = Tokenizer::new("# This is a comment\n# This is another comment\n!");
    tokenizer.eat_comments();

    // Check last character in buffer
    assert_eq!(tokenizer.chars.next().unwrap(), '!');
  }

  #[test]
  pub fn test_eat_comment_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("# This is a comment\n# This is another comment\n");
    tokenizer.eat_comments();

    // Check last character in buffer
    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_ident_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("$aeiöu_0123");
    // Consume leading `$` character
    assert_eq!(tokenizer.chars.next().unwrap(), '$');
    // Consume var ident
    let var = tokenizer.eat_ident(tokenizer.chars.pos(), false).unwrap();

    assert_eq!(var, "aeiöu_0123");
    // Check last character in buffer
    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_ident_end_at_non_var() {
    let mut tokenizer = Tokenizer::new("$aeiöu_0123!");
    // Consume leading `$` character
    assert_eq!(tokenizer.chars.next().unwrap(), '$');
    // Consume var ident
    let var = tokenizer.eat_ident(tokenizer.chars.pos(), false).unwrap();
    assert_eq!(tokenizer.chars.next().unwrap(), '!');

    // Check last character in buffer
    assert_eq!(var, "aeiöu_0123");
  }

  #[test]
  pub fn test_eat_ident_has_alpha_or_underscore_fail() {
    let mut tokenizer = Tokenizer::new("0");
    // Consume var ident
    let var = tokenizer.eat_ident(tokenizer.chars.pos(), false);
    match var {
      Err(TokenizeError::BadIdent(..)) => {}
      _ => panic!("Expected `TokenizeError::BadIdent(..)`"),
    };

    assert_eq!(tokenizer.chars.next(), Some('0'));
  }

  #[test]
  pub fn test_eat_string_end_at_end_of_stream() {
    let mut tokenizer = Tokenizer::new("\"This is a string\"");
    let beg = tokenizer.pos();

    assert_eq!(tokenizer.chars.next(), Some('"'));
    assert_eq!(tokenizer.eat_string(beg, '"').unwrap(), "This is a string");
    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_string_unexpected_eof() {
    let mut tokenizer = Tokenizer::new("'This is a bad string");
    let beg = tokenizer.pos();

    assert_eq!(tokenizer.chars.next(), Some('\''));

    match tokenizer.eat_string(beg, '\'') {
      Err(TokenizeError::BadString(..)) => {}
      _ => panic!("Expected `TokenizeError::BadString(..)`"),
    };

    assert_eq!(tokenizer.chars.next(), None);
  }

  #[test]
  pub fn test_eat_string_unexpected_eol() {
    let mut tokenizer = Tokenizer::new("'This is a bad string\n'");
    let beg = tokenizer.pos();

    assert_eq!(tokenizer.chars.next(), Some('\''));

    match tokenizer.eat_string(beg, '\'') {
      Err(TokenizeError::BadString(..)) => {}
      _ => panic!("Expected `TokenizeError::BadString(..)`"),
    };

    assert_eq!(tokenizer.chars.next(), Some('\n'));
  }

  #[test]
  #[allow(clippy::float_cmp)]
  pub fn test_eat_number_floating() {
    let mut tokenizer = Tokenizer::new("1337.60");
    let beg = tokenizer.pos();

    assert_eq!(tokenizer.chars.next(), Some('1'));
    assert_eq!(tokenizer.eat_number(beg).unwrap(), 1337.60f64);
  }

  #[test]
  #[allow(clippy::float_cmp)]
  pub fn test_eat_number_whole() {
    let mut tokenizer = Tokenizer::new("69420");
    let beg = tokenizer.pos();

    assert_eq!(tokenizer.chars.next(), Some('6'));
    assert_eq!(tokenizer.eat_number(beg).unwrap(), 69420f64);
  }

  #[test]
  pub fn test_eat_number_bad() {
    let mut tokenizer = Tokenizer::new("694.2.0");
    let beg = tokenizer.pos();

    assert_eq!(tokenizer.chars.next(), Some('6'));

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
