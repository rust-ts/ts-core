use tscore_ast::token::TokenKind;
use tscore_ast::tokenstream::TokenStream;
use tscore_common::ScriptTarget;
use tscore_lexer::tokenize;

use tscore_span::{BytePos, Span};

pub struct Scanner<'a> {
  src: &'a str,
  lang_version: ScriptTarget,
  pos: BytePos,
  end_src_index: usize,
}

impl<'a> Scanner<'a> {
  pub fn new(src: &'a str, lang_version: ScriptTarget, start_pos: BytePos) -> Self {
    Scanner { src, lang_version, pos: start_pos, end_src_index: src.len() }
  }
}
