use std::{cmp, fmt::Formatter, iter::FusedIterator};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
  pub line: usize,
  pub column: usize,
  pub offset: usize,
}

impl Default for Position {
  fn default() -> Self {
    Self {
      line: 1,
      column: 0,
      offset: 0,
    }
  }
}

#[derive(Clone)]
pub struct Span<'buf> {
  beg: Position,
  end: Position,
  buf: &'buf str,
}

impl<'buf> Span<'buf> {
  pub fn new(beg: Position, end: Position, buf: &'buf str) -> Self {
    Self { beg, end, buf }
  }

  pub fn as_str(&self) -> &'buf str {
    &self.buf[self.beg.offset..self.end.offset]
  }
}

impl std::fmt::Debug for Span<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let beg_abs = self.beg.offset;
    let beg = cmp::max(0, beg_abs - 5);
    let beg_len = beg_abs - beg;

    let end = self.end.offset;
    let end = cmp::min(self.buf.len(), end + 5);

    writeln!(f, "line {}, column {}", self.end.line, self.end.column,)?;
    writeln!(f, "{}", &self.buf[beg..end])?;
    writeln!(f, "{}^", "-".repeat(beg_len))?;

    Ok(())
  }
}

pub trait Positional {
  fn pos(&self) -> Position;
}

pub trait Spannable<'buf> {
  fn span(&self) -> Span<'buf>;
  fn span_to(&self, to: Position) -> Span<'buf>;
}

pub trait AsStr<'a> {
  fn as_str(&self) -> &'a str;
}

pub trait IntoPeekableExt: Sized + Iterator {
  fn peekable_ext(self) -> PeekableExt<Self>;
}

impl<I: Sized + Iterator> IntoPeekableExt for I {
  fn peekable_ext(self) -> PeekableExt<Self> {
    PeekableExt::new(self)
  }
}

/// An iterator with a `peek()` that returns an optional reference to the next
/// element.
///
/// This `struct` is created by the [`peekable`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`peekable`]: Iterator::peekable
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PeekableExt<I: Iterator> {
  iter: I,
  /// Remember a peeked value, even if it was None.
  peeked: Option<Option<I::Item>>,
  position: Position,
}

impl<I: Iterator + Positional> PeekableExt<I> {
  pub fn new(iter: I) -> PeekableExt<I> {
    PeekableExt {
      iter,
      peeked: None,
      position: iter.pos(),
    }
  }
}

// PeekableExt must remember if a None has been seen in the `.peek()` method.
// It ensures that `.peek(); .peek();` or `.peek(); .next();` only advances the
// underlying iterator at most once. This does not by itself make the iterator
// fused.
impl<I: Iterator> Iterator for PeekableExt<I> {
  type Item = I::Item;

  #[inline]
  fn next(&mut self) -> Option<I::Item> {
    match self.peeked.take() {
      Some(v) => v,
      None => self.iter.next(),
    }
  }
}

impl<I: Iterator> PeekableExt<I> {
  /// Returns a reference to the next() value without advancing the iterator.
  ///
  /// Like [`next`], if there is a value, it is wrapped in a `Some(T)`.
  /// But if the iteration is over, `None` is returned.
  ///
  /// [`next`]: Iterator::next
  ///
  /// Because `peek()` returns a reference, and many iterators iterate over
  /// references, there can be a possibly confusing situation where the
  /// return value is a double reference. You can see this effect in the
  /// examples below.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// let xs = [1, 2, 3];
  ///
  /// let mut iter = xs.iter().peekable();
  ///
  /// // peek() lets us see into the future
  /// assert_eq!(iter.peek(), Some(&&1));
  /// assert_eq!(iter.next(), Some(&1));
  ///
  /// assert_eq!(iter.next(), Some(&2));
  ///
  /// // The iterator does not advance even if we `peek` multiple times
  /// assert_eq!(iter.peek(), Some(&&3));
  /// assert_eq!(iter.peek(), Some(&&3));
  ///
  /// assert_eq!(iter.next(), Some(&3));
  ///
  /// // After the iterator is finished, so is `peek()`
  /// assert_eq!(iter.peek(), None);
  /// assert_eq!(iter.next(), None);
  /// ```
  #[inline]
  pub fn peek(&mut self) -> Option<&I::Item> {
    let iter = &mut self.iter;
    self.peeked.get_or_insert_with(|| iter.next()).as_ref()
  }

  /// Returns a mutable reference to the next() value without advancing the iterator.
  ///
  /// Like [`next`], if there is a value, it is wrapped in a `Some(T)`.
  /// But if the iteration is over, `None` is returned.
  ///
  /// Because `peek_mut()` returns a reference, and many iterators iterate over
  /// references, there can be a possibly confusing situation where the
  /// return value is a double reference. You can see this effect in the examples
  /// below.
  ///
  /// [`next`]: Iterator::next
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// #![feature(peekable_peek_mut)]
  /// let mut iter = [1, 2, 3].iter().peekable();
  ///
  /// // Like with `peek()`, we can see into the future without advancing the iterator.
  /// assert_eq!(iter.peek_mut(), Some(&mut &1));
  /// assert_eq!(iter.peek_mut(), Some(&mut &1));
  /// assert_eq!(iter.next(), Some(&1));
  ///
  /// // Peek into the iterator and set the value behind the mutable reference.
  /// if let Some(p) = iter.peek_mut() {
  ///     assert_eq!(*p, &2);
  ///     *p = &5;
  /// }
  ///
  /// // The value we put in reappears as the iterator continues.
  /// assert_eq!(iter.collect::<Vec<_>>(), vec![&5, &3]);
  /// ```
  #[inline]
  pub fn peek_mut(&mut self) -> Option<&mut I::Item> {
    let iter = &mut self.iter;
    self.peeked.get_or_insert_with(|| iter.next()).as_mut()
  }

  /// Consume and return the next value of this iterator if a condition is true.
  ///
  /// If `func` returns `true` for the next value of this iterator, consume and return it.
  /// Otherwise, return `None`.
  ///
  /// # Examples
  /// Consume a number if it's equal to 0.
  /// ```
  /// let mut iter = (0..5).peekable();
  /// // The first item of the iterator is 0; consume it.
  /// assert_eq!(iter.next_if(|&x| x == 0), Some(0));
  /// // The next item returned is now 1, so `consume` will return `false`.
  /// assert_eq!(iter.next_if(|&x| x == 0), None);
  /// // `next_if` saves the value of the next item if it was not equal to `expected`.
  /// assert_eq!(iter.next(), Some(1));
  /// ```
  ///
  /// Consume any number less than 10.
  /// ```
  /// let mut iter = (1..20).peekable();
  /// // Consume all numbers less than 10
  /// while iter.next_if(|&x| x < 10).is_some() {}
  /// // The next value returned will be 10
  /// assert_eq!(iter.next(), Some(10));
  /// ```
  pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
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

  /// Consume and return the next item if it is equal to `expected`.
  ///
  /// # Example
  /// Consume a number if it's equal to 0.
  /// ```
  /// let mut iter = (0..5).peekable();
  /// // The first item of the iterator is 0; consume it.
  /// assert_eq!(iter.next_if_eq(&0), Some(0));
  /// // The next item returned is now 1, so `consume` will return `false`.
  /// assert_eq!(iter.next_if_eq(&0), None);
  /// // `next_if_eq` saves the value of the next item if it was not equal to `expected`.
  /// assert_eq!(iter.next(), Some(1));
  /// ```
  pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
  where
    T: ?Sized,
    I::Item: PartialEq<T>,
  {
    self.next_if(|next| next == expected)
  }
}

impl<'a, I> AsStr<'a> for PeekableExt<I>
where
  I: Iterator + AsStr<'a>,
{
  fn as_str(&self) -> &'a str {
    self.iter.as_str()
  }
}

impl<I> Positional for PeekableExt<I>
where
  I: Iterator + Positional,
{
  fn pos(&self) -> Position {
    self.iter.pos()
  }
}

impl<'buf, I> Spannable<'buf> for PeekableExt<I>
where
  I: Iterator + Spannable<'buf>,
{
  fn span(&self) -> Span<'buf> {
    self.span()
  }

  fn span_to(&self, to: Position) -> Span<'buf> {
    self.span_to(to)
  }
}
