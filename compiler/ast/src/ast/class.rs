use crate::ptr::P;
use crate::token::{self, CommentKind, DelimToken, Token};
use crate::tokenstream::{DelimSpan, LazyTokenStream, TokenStream, TokenTree};

use rustc_data_structures::stable_hasher::{HashStable, StableHasher};
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_data_structures::sync::Lrc;
use rustc_data_structures::thin_vec::ThinVec;
use rustc_macros::HashStable_Generic;
use rustc_serialize::{self, Decoder, Encoder};
use tscore_span::source_map::{respan, Spanned};
use tscore_span::symbol::{kw, sym, Ident, Symbol};
use tscore_span::{Span, DUMMY_SP};

use super::{Block, Decorator, Expr, ExprWithTypeArgs, Fn, Generics, JSDoc, Lit, Pat, Ty};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum PropName {
  Ident(Ident),
  /// only StrLit and NumLit available
  NumLit(Lit),
  Expr(P<Expr>),
  /// `#name`
  PrivateIdent(Ident),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Prop {
  pub name: Ident,
  pub ty: Option<Ty>,
  pub init: Option<Expr>,
  pub decorators: Vec<Decorator>,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum ClassElementKind {
  /// ES6 allows these as class elements.
  Semi,
  /// separate constructor from normal method for potensial processing
  Constructor(P<Fn>),
  Getter(P<Fn>),
  Setter(P<Fn>),
  IndexSig(P<Ty>),
  Method(P<Fn>),
  Property(P<Prop>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum Visibility {
  Public,
  Protected,
  Private,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct ClassElement {
  pub kind: ClassElementKind,
  pub vis: Visibility,
  pub name: PropName,
  pub is_static: bool,
  pub is_abstract: bool,
  pub definite: bool,
  pub optional: bool,
  pub readonly: bool,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum Heritage {
  Extends(ExprWithTypeArgs),
  Impl(ExprWithTypeArgs),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Class {
  /// May be optional in `export default class { ... }`.
  pub name: Option<Ident>,
  pub generics: Generics,
  pub heritages: Vec<Heritage>,
  pub span: Span,
  pub elementss: Vec<ClassElement>,
  pub is_abstract: bool,
  pub decorators: Vec<Decorator>,
}
