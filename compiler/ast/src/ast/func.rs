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

use super::{Block, Decorator, Expr, FnSig, Generics, JSDoc, Lit, Pat, Ty};

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct FnParam {
  pub ty: P<Ty>,
  pub name: P<Pat>,
  pub decorators: Vec<Decorator>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum FnBody {
  Block(P<Block>),
  Expr(P<Expr>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Fn {
  /// Optional for arrow function or anonymous function.
  ///
  /// ```js
  /// const fnA = () => { ... }
  /// const fnB = function() { ... }
  /// const fnC = () => { ... }
  /// ```
  pub name: Option<Ident>,
  /// Function Signature
  pub sig: FnSig,
  /// TODO: what if a ts function declaration?
  pub body: P<FnBody>,
  /// Indicate whether a function is `async` function.
  ///
  /// ```js
  /// async function longTask() {
  ///   // ...
  /// }
  /// ```
  pub asyncness: Option<Span>,
  /// Indicate whether a function is `generator`.
  ///
  /// ```js
  /// function* generator(i) {
  ///   yield i;
  ///   yield i + 10;
  /// }
  /// ```
  pub generator: Option<Span>,
  pub decorators: Vec<Decorator>,
  pub span: Span,
  pub js_docs: Vec<JSDoc>,
}
