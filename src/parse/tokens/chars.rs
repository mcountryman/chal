use crate::parse::{AsStr, Position, Positional, Span, Spannable};
use std::str::Chars;

/// An iterator over characters with position tracking.
#[derive(Debug, Clone)]
pub struct TokenizerChars<'buf> {
  buf: &'buf str,
  pos: Position,
  chars: Chars<'buf>,
}

impl<'buf> TokenizerChars<'buf> {
  /// Create [`TokenizerChars`]
  pub fn new(buf: &'buf str) -> Self {
    Self {
      buf,
      pos: Position::default(),
      chars: buf.chars(),
    }
  }
}

impl Iterator for TokenizerChars<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let ch = self.chars.next();
    match ch {
      Some(ch) => {
        self.pos.offset += ch.len_utf8();

        if ch == '\n' {
          self.pos.line += 1;
          self.pos.column = 1;
        } else {
          self.pos.column += 1;
        }

        Some(ch)
      }
      None => None,
    }
  }
}

impl<'a> AsStr<'a> for TokenizerChars<'a> {
  fn as_str(&self) -> &'a str {
    self.chars.as_str()
  }
}

impl Positional for TokenizerChars<'_> {
  fn pos(&self) -> Position {
    self.pos
  }
}

impl<'buf> Spannable<'buf> for TokenizerChars<'buf> {
  fn span(&self) -> Span<'buf> {
    Span::new(self.pos, self.pos, self.buf)
  }

  fn span_to(&self, to: Position) -> Span<'buf> {
    Span::new(self.pos, to, self.buf)
  }
}
