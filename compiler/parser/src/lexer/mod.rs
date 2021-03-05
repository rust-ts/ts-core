pub mod flags;

use tscore_ast::token::TokenKind;
use tscore_common::ScriptTarget;
use tscore_lexer::tokenize;

pub use flags::TokenFlags;

pub struct Scanner<'a> {
  src: &'a str,
  lang_version: ScriptTarget,

  token: TokenKind,
  token_flags: TokenFlags::TokenFlag,
  token_value: &'a str,
}

impl<'a> Scanner<'a> {
  fn new(src: &'a str, lang_version: ScriptTarget) -> Self {
    Self { src, lang_version, token: TokenKind::Unknown, token_flags: 0, token_value: "" }
  }

  pub fn next_token() -> TokenKind {
    todo!()
  }

  pub fn has_unicode_escape(&self) -> bool {
    (self.token_flags & TokenFlags::UnicodeEscape) != 0
  }

  pub fn has_extended_unicode_escapse(&self) -> bool {
    (self.token_flags & TokenFlags::ExtendedUnicodeEscape) != 0
  }
  pub fn has_perceding_line_break(&self) -> bool {
    (self.token_flags & TokenFlags::PrecedingLineBreak) != 0
  }
  pub fn has_perceding_js_doc_comment(&self) -> bool {
    (self.token_flags & TokenFlags::PrecedingJSDocComment) != 0
  }
}
