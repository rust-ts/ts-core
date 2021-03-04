use crate::context::ParsingContext;
use crate::diagnostic::DiagnosticWithDetachedLocation;
use std::collections::BTreeMap;
use tscore_ast::token::TokenKind;
use tscore_ast::{Node, NodeFlags, SourceFile};
use tscore_common::path::{is_declaration_file, normalize_path};
use tscore_common::{LanguageVariant, ScriptKind, ScriptTarget};

pub trait SyntaxCursor {
  fn current_node(&self, pos: usize) -> Node;
}

pub struct Parser<SyntaxCursor> {
  file_name: String,
  source_text: String,
  source_flags: NodeFlags::NodeFlag,

  lang_version: ScriptTarget,
  script_kind: ScriptKind,
  lang_variant: LanguageVariant,
  parse_diagnostics: Vec<DiagnosticWithDetachedLocation>,
  jsdoc_diagnostics: Vec<DiagnosticWithDetachedLocation>,
  syntax_cursor: Option<SyntaxCursor>,

  token: TokenKind,
  node_count: usize,
  idents: BTreeMap<String, String>,
  private_idents: BTreeMap<String, String>,
  idents_count: usize,

  parsing_ctx: ParsingContext,

  not_parenthesized_arrow: Option<usize>,

  ctx_flags: NodeFlags::NodeFlag,

  top_level: bool,

  parse_error_before_next_finished_node: bool,
}

impl<T: SyntaxCursor> Default for Parser<T> {
  fn default() -> Self {
    Self {
      script_kind: ScriptKind::TS,
      token: TokenKind::Eof,
      parsing_ctx: ParsingContext::SourceElements,
      top_level: true,
      ..Default::default()
    }
  }
}

impl<T: SyntaxCursor> Parser<T> {
  pub fn new() -> Self {
    Parser::default()
  }

  pub(crate) fn initialize(
    &mut self,
    file_name: String,
    source: String,
    lang_version: ScriptTarget,
    syntax_cursor: Option<T>,
    script_kind: ScriptKind,
  ) {
    self.file_name = normalize_path(file_name);
    self.source_text = source;
    self.lang_version = lang_version;
    self.syntax_cursor = syntax_cursor;
    self.script_kind = script_kind;
    self.lang_variant = LanguageVariant::from(script_kind);
    self.reset();

    match script_kind {
      ScriptKind::JS | ScriptKind::JSX => self.ctx_flags = NodeFlags::JavaScriptFile,
      ScriptKind::JSON => self.ctx_flags = NodeFlags::JavaScriptFile | NodeFlags::JsonFile,
      _ => self.ctx_flags = NodeFlags::None,
    }

    self.parse_error_before_next_finished_node = false;
  }

  pub(crate) fn reset(&mut self) {
    self.source_text = "".to_string();
    self.lang_version = ScriptTarget::ES2015;
    self.syntax_cursor = None;
    self.script_kind = ScriptKind::TS;
    self.lang_variant = LanguageVariant::Standard;
    self.parse_diagnostics = vec![];
    self.jsdoc_diagnostics = vec![];
    self.parsing_ctx = ParsingContext::SourceElements;
    self.idents = Default::default();
    self.private_idents = Default::default();
    self.idents_count = 0;
    self.node_count = 0;
    self.source_flags = 0;
    self.top_level = true;
  }

  pub fn parse_source_file(
    &mut self,
    lang_version: ScriptTarget,
    set_parent_nodes: bool,
    script_kind: ScriptKind,
  ) {
    let is_declaration_file = is_declaration_file(&self.file_name);
    if is_declaration_file {
      self.ctx_flags |= NodeFlags::Ambient;
    }

    self.source_flags = self.ctx_flags;
  }

  fn next_token() -> TokenKind {
    // if self.token.is_keyword() && self.
    todo!()
  }
}
