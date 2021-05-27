use crate::parse::Position;
use std::str::Chars;

/// An iterator over characters with position tracking.
#[derive(Debug, Clone)]
pub struct TokenizerChars<'buf> {
  buf: &'buf str,
  pos: Position,
  peeked: Option<Option<(Position, char)>>,
  chars: Chars<'buf>,
}

impl<'buf> TokenizerChars<'buf> {
  /// Create [`TokenizerChars`]
  pub fn new(buf: &'buf str) -> Self {
    Self {
      buf,
      pos: Position::default(),
      peeked: None,
      chars: buf.chars(),
    }
  }

  /// Get current position of iterator.
  pub fn pos(&self) -> Position {
    self.pos
  }

  /// Get reference to underlying str.
  pub fn as_str(&self) -> &'buf str {
    &self.buf
  }

  /// Peek next char.
  pub fn peek(&mut self) -> Option<&char> {
    let pos = &self.pos;
    let chars = &mut self.chars;

    self
      .peeked
      .get_or_insert_with(|| Self::next_ch(pos, chars))
      .as_ref()
      .map(|(_, ch)| ch)
  }

  /// Consumes next char and returns result if predicate returns true.
  ///
  /// # Arguments
  /// * `func` - The predicate used to determine if iterator will proceed.
  pub fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
    let next = match self.peeked.take() {
      Some(peeked) => peeked,
      None => Self::next_ch(&self.pos, &mut self.chars),
    };

    match next {
      Some((pos, ch)) if func(&ch) => {
        self.pos = pos;
        Some(ch)
      }
      other => {
        // Since we called `self.next()`, we consumed `self.peeked`.
        assert!(self.peeked.is_none());
        self.peeked = Some(other);
        None
      }
    }
  }

  fn next_ch(pos: &Position, chars: &mut Chars<'buf>) -> Option<(Position, char)> {
    let ch = chars.next();
    match ch {
      Some(ch) => {
        let mut line = pos.line();
        let mut column = pos.column();
        let offset = pos.offset() + ch.len_utf8();

        if ch == '\n' {
          line += 1;
          column = 1;
        } else {
          column += 1;
        }

        Some((Position::new(line, column, offset), ch))
      }
      None => None,
    }
  }
}

impl Iterator for TokenizerChars<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let next = match self.peeked.take() {
      Some(peeked) => peeked,
      None => Self::next_ch(&self.pos, &mut self.chars),
    };

    match next {
      Some((pos, ch)) => {
        self.pos = pos;
        Some(ch)
      }
      None => None,
    }
  }
}
