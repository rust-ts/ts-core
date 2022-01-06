use std::str::Chars;
use tscore_span::{BytePos, Pos};

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `nth_char` method,
/// and position can be shifted forward via `bump` method.
pub(crate) struct Cursor<'a> {
  src: &'a str,
  chars: Chars<'a>,
  #[cfg(debug_assertions)]
  prev: char,

  start_pos: BytePos,
  pos: BytePos,
  end_src_index: usize,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
  pub(crate) fn new(src: &'a str, start_pos: BytePos) -> Cursor<'a> {
    Cursor {
      src,
      chars: src.chars(),
      #[cfg(debug_assertions)]
      prev: EOF_CHAR,
      start_pos,
      pos: start_pos,
      end_src_index: src.len(),
    }
  }

  /// Returns the last eaten symbol (or `'\0'` in release builds).
  /// (For debug assertions only.)
  pub(crate) fn prev(&self) -> char {
    #[cfg(debug_assertions)]
    {
      self.prev
    }

    #[cfg(not(debug_assertions))]
    {
      EOF_CHAR
    }
  }

  /// Returns nth character relative to the current cursor position.
  /// If requested position doesn't exist, `EOF_CHAR` is returned.
  /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
  /// it should be checked with `is_eof` method.
  fn nth_char(&self, n: usize) -> char {
    self.chars().nth(n).unwrap_or(EOF_CHAR)
  }

  /// Peeks the next symbol from the input stream without consuming it.
  pub(crate) fn first(&self) -> char {
    self.nth_char(0)
  }

  /// Peeks the second symbol from the input stream without consuming it.
  pub(crate) fn second(&self) -> char {
    self.nth_char(1)
  }

  /// Checks if the pos is on start pos
  pub(crate) fn is_bof(&self) -> bool {
    self.pos == self.start_pos
  }

  /// Checks if there is nothing more to consume.
  pub(crate) fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  /// Returns amount of already consumed symbols.
  pub(crate) fn len_consumed(&self) -> usize {
    self.src.len() - self.chars.as_str().len()
  }

  /// Returns a `Chars` iterator over the remaining characters.
  fn chars(&self) -> Chars<'a> {
    self.chars.clone()
  }

  /// Moves to the next character.
  pub(crate) fn bump(&mut self) -> Option<char> {
    let c = self.chars.next()?;

    #[cfg(debug_assertions)]
    {
      self.prev = c;
    }

    Some(c)
  }

  /// Eats symbols while predicate returns true or until the end of file is reached.
  pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.first()) && !self.is_eof() {
      self.bump();
    }
  }

  pub(crate) fn cur_pos(&self) -> BytePos {
    self.pos
  }

  pub(crate) fn text(&self) -> &'a str {
    self.chars.as_str()
  }

  pub(crate) fn forword_by_usize(&mut self, size: usize) -> BytePos {
    self.pos + BytePos::from_usize(size);

    self.pos
  }
  pub(crate) fn src_index(&self, pos: BytePos) -> usize {
    (pos - self.start_pos).to_usize()
  }

  /// Slice of the source text from `start` up to but excluding `self.pos`,
  /// meaning the slice does not include the character `self.ch`.
  pub(crate) fn str_from(&self, start: BytePos) -> &str {
    self.str_from_to(start, self.pos)
  }

  /// Slice of the source text spanning from `start` up to but excluding `end`.
  pub(crate) fn str_from_to(&self, start: BytePos, end: BytePos) -> &str {
    &self.src[self.src_index(start)..self.src_index(end)]
  }
}
