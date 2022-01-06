use super::{Expr, Fn, PropName};
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

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum LitKind {
  /// `"foo"`
  Str(Symbol),
  /// `1`
  Num(f64),
  /// `true`
  Bool(bool),
  /// `\`foo${bar}\``
  Template(P<TemplateLit>),
  /// `1n`
  BitInt,
  /// `/[a-z]/g`
  RegExp,
  /// `null`
  Null,
  Array(P<ArrayLit>),
  Object(P<ObjectLit>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Lit {
  pub token: token::Lit,
  pub kind: LitKind,
  pub span: Span,
}

/// Same as `Lit` but restricted t ostring literals.
#[derive(Clone, Copy, Encodable, Decodable, Debug)]
pub struct StrLit {
  pub span: Span,
  pub symbol: Symbol,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct TemplateSpan {
  pub span: Span,
  pub expr: P<Expr>,
  pub lit: Option<StrLit>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct TemplateLit {
  pub span: Span,
  pub head: Option<StrLit>,
  pub spans: Vec<P<TemplateSpan>>,
}

#[derive(Clone, Copy, Encodable, Decodable, Debug)]
pub struct NumLit {
  pub span: Span,
  pub value: f64,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct ObjectLit {
  pub multi_line: bool,
  pub props: Vec<P<ObjectLitEl>>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct ObjectLitEl {
  pub kind: ObjectLitElKind,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct PropAssign {
  pub span: Span,
  pub name: PropName,
  pub optional: bool,
  pub definite: bool,
  pub init: P<Expr>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum ObjectLitElKind {
  PropAssign(P<PropAssign>),
  ShortPropAssign(P<PropAssign>),
  SpreadAssign(P<Expr>),
  MethodDecl(P<Fn>),
  Getter(P<Fn>),
  Setter(P<Fn>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct ArrayLit {
  pub elements: Vec<P<Expr>>,
  pub multi_line: bool,
}
