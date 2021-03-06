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

use super::{
  Class, Expr, Fn, FnSig, Generics, Heritage, JSDoc, NameBinding, Pat, PropName, StrLit, Ty,
};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum DeclName {
  Ident(Ident),
  Lit,
}
#[derive(Clone, Encodable, Decodable, Debug)]
pub enum VarDeclKind {
  Var,
  Let,
  Const,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct VarDecl {
  pub name: Pat,
  pub ty: Option<Ty>,
  pub definite: bool,
  pub init: Option<Expr>,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct VarDeclList {
  pub kind: VarDeclKind,
  pub decls: Vec<VarDecl>,
  pub span: Span,
  pub js_docs: Vec<JSDoc>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Decorator {
  pub span: Span,
  pub expr: P<Expr>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum InterfaceElement {
  CallSig(P<Ty>),
  ConstructorSig(P<Ty>),
  PropSig(P<PropName>, P<Ty>),
  MethodSig(P<PropName>, P<Ty>),
  IndexSig(P<Ty>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Interface {
  pub name: Ident,
  pub generics: Generics,
  pub heritages: Vec<Heritage>,
  pub elements: Vec<InterfaceElement>,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct TypeAlias {
  pub name: Ident,
  pub generics: Generics,
  pub ty: P<Ty>,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Import {
  /// Default binding.
  ///
  /// > Default binding can coexists with one of namespace binding or names binding.
  ///
  /// `import defaultExport from 'module-name'`
  ///
  /// Combine with named import:
  /// `import defaultExport, { export1 } from 'module-name'`
  ///
  /// Combine with Namespace imprt:
  /// `import defaultExport, * as name from 'module-name'`
  pub default: Option<Ident>,
  /// Namespace import binding.
  ///
  /// `import * as name from 'module-name'`
  pub namespcae: Option<Ident>,
  /// `import { export1, export2 as alias2 } from 'module-name'`
  pub names: Vec<NameBinding>,
  pub from: StrLit,
  pub type_only: bool,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Export {
  pub type_only: bool,
  ///
  /// `export * as name from 'module-name'`
  ///
  pub namespace: Option<Ident>,
  ///
  /// `export { a, b as b1 } from 'module-name'`
  pub names: Vec<NameBinding>,
  /// Re-export
  ///
  /// ```js
  /// export { ... } from 'module-name'
  /// ```
  pub from: StrLit,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum DeclKind {
  /// A variable(s) declaration statement.
  ///
  /// E.g., `let a = 1`, `let a = 1, b = 2`
  ///
  /// It could has an preceding jsdoc with some meta data:
  /// ```js
  /// /**
  ///  * @type {string}
  ///  */
  /// let name
  /// ```
  Var(P<VarDeclList>),
  Fn(P<Fn>),
  Class(P<Class>),
  Interface(P<Interface>),
  /// The static import statement is used to import read only live bindings which are exported by another module.
  ///
  /// See [Import] for more detail.
  Import(P<Import>),
  Export(P<Export>),
  Type(P<TypeAlias>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Decl {
  pub kind: DeclKind,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct DeclList {
  pub span: Span,
  pub decls: Vec</* VarDecl */ Decl>,
}
