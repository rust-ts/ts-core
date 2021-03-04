use crate::token::{ModifierToken, TokenKind};
use crate::{ModifierFlags, TransformFlags};
use std::collections::BTreeMap;
use std::ptr::NonNull;
use std::rc::Rc;

pub struct Symbol {}

pub type SymbolTable = BTreeMap<String, Symbol>;

pub struct Node {
  pos: usize,
  end: usize,

  kind: TokenKind,
  flags: ModifierFlags::ModifierFlag,
  transformFlags: TransformFlags::TransformFlag,

  symbol: Symbol,
  modifiers: Vec<ModifierToken>,
  decorators: Option<Box<Node>>,
  id: usize,
  // parent: Rc<Node>,
  // original: Option<NonNull<Node>>,
  // locals: Option<SymbolTable>,
  // next_container: Option<NonNull<Node>>,
  // local_Symbol: Option<Symbol>,
  // flow_node: Option<Node>,
}

pub struct SourceFile {
  pub file_name: String,
  pub statements: Vec<u32>,
}
