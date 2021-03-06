//! An "interner" is a data structure that associates values with usize tags and
//! allows bidirectional lookup; i.e., given a value, one can easily find the
//! type, and vice versa.

use rustc_arena::DroplessArena;
use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::stable_hasher::{HashStable, StableHasher, ToStableHashKey};
use rustc_macros::HashStable_Generic;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

use std::cmp::{Ord, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str;

use crate::{Span, DUMMY_SP, SESSION_GLOBALS};

#[cfg(test)]
mod tests;

// The proc macro code for this is in `src/librustc_macros/src/symbols.rs`.
symbols! {
    // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
    // this should be rarely necessary though if the keywords are kept in alphabetic order.
    Keywords {
      Empty: "",
      Abstract: "abstract",
      Any: "any",
      As: "as",
      Asserts: "asserts",
      Async: "async",
      Await: "await",
      Break: "break",
      Case: "case",
      Catch: "catch",
      Class: "class",
      Const: "const",
      Constructor: "constructor",
      Continue: "continue",
      Debugger: "debugger",
      Declare: "declare",
      Default: "default",
      Delete: "delete",
      Do: "do",
      Else: "else",
      Enum: "enum",
      Export: "export",
      Extends: "extends",
      False: "false",
      Finally: "finally",
      For: "for",
      From: "from",
      Function: "function",
      Get: "get",
      Global: "global",
      If: "if",
      Implements: "implements",
      Import: "import",
      Infer: "infer",
      In: "in",
      InstanceOf: "instanceOf",
      Interface: "interface",
      Intrinsic: "intrinsic",
      Is: "is",
      KeyOf: "keyOf",
      Let: "let",
      Module: "module",
      Namespace: "namespace",
      Never: "never",
      New: "new",
      Null: "null",
      Of: "of",
      Package: "package",
      Private: "private",
      Protected: "protected",
      Public: "public",
      Readonly: "readonly",
      Require: "require",
      Return: "return",
      Static: "static",
      Super: "super",
      Switch: "switch",
      This: "this",
      Throw: "throw",
      True: "true",
      Try: "try",
      Type: "type",
      TypeOf: "typeOf",
      Undefined: "undefined",
      Unique: "unique",
      Unknown: "unknown",
      Var: "var",
      Void: "void",
      While: "while",
      With: "with",
      Yield: "yield",
    }

    // Pre-interned symbols that can be referred to with `rustc_span::sym::*`.
    //
    // The symbol is the stringified identifier unless otherwise specified, in
    // which case the name should mention the non-identifier punctuation.
    // E.g. `sym::proc_dash_macro` represents "proc-macro", and it shouldn't be
    // called `sym::proc_macro` because then it's easy to mistakenly think it
    // represents "proc_macro".
    //
    // As well as the symbols listed, there are symbols for the strings
    // "0", "1", ..., "9", which are accessible via `sym::integer`.
    //
    // The proc macro will abort if symbols are not in alphabetical order (as
    // defined by `impl Ord for str`) or if any symbols are duplicated. Vim
    // users can sort the list by selecting it and executing the command
    // `:'<,'>!LC_ALL=C sort`.
    //
    // There is currently no checking that all symbols are used; that would be
    // nice to have.
    Symbols {
      BigInt,
      Boolean,
      Number,
      Object,
      String,
      // Symbol,
    }
}

#[derive(Copy, Clone, Eq, HashStable_Generic, Encodable, Decodable)]
pub struct Ident {
  pub name: Symbol,
  pub span: Span,
}

impl Ident {
  #[inline]
  /// Constructs a new identifier from a symbol and a span.
  pub const fn new(name: Symbol, span: Span) -> Ident {
    Ident { name, span }
  }

  /// Constructs a new identifier with a dummy span.
  #[inline]
  pub const fn with_dummy_span(name: Symbol) -> Ident {
    Ident::new(name, DUMMY_SP)
  }

  #[inline]
  pub fn invalid() -> Ident {
    Ident::with_dummy_span(kw::Empty)
  }

  /// Maps a string to an identifier with a dummy span.
  pub fn from_str(string: &str) -> Ident {
    Ident::with_dummy_span(Symbol::intern(string))
  }

  /// Maps a string and a span to an identifier.
  pub fn from_str_and_span(string: &str, span: Span) -> Ident {
    Ident::new(Symbol::intern(string), span)
  }

  /// Replaces `lo` and `hi` with those from `span`, but keep hygiene context.
  pub fn with_span_pos(self, span: Span) -> Ident {
    Ident::new(self.name, span)
  }

  pub fn without_first_quote(self) -> Ident {
    Ident::new(Symbol::intern(self.as_str().trim_start_matches('\'')), self.span)
  }

  /// Convert the name to a `SymbolStr`. This is a slowish operation because
  /// it requires locking the symbol interner.
  pub fn as_str(self) -> SymbolStr {
    self.name.as_str()
  }
}

impl PartialEq for Ident {
  fn eq(&self, rhs: &Self) -> bool {
    self.name == rhs.name
  }
}

impl Hash for Ident {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
  }
}

impl fmt::Debug for Ident {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

/// This implementation is supposed to be used in error messages, so it's expected to be identical
/// to printing the original identifier token written in source code (`token_to_string`),
/// except that AST identifiers don't keep the rawness flag, so we have to guess it.
impl fmt::Display for Ident {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&IdentPrinter::new(self.name, self.is_raw_guess()), f)
  }
}

/// This is the most general way to print identifiers.
/// AST pretty-printer is used as a fallback for turning AST structures into token streams for
/// proc macros. Additionally, proc macros may stringify their input and expect it survive the
/// stringification (especially true for proc macro derives written between Rust 1.15 and 1.30).
/// So we need to somehow pretty-print `$crate` in a way preserving at least some of its
/// hygiene data, most importantly name of the crate it refers to.
/// As a result we print `$crate` as `crate` if it refers to the local crate
/// and as `::other_crate_name` if it refers to some other crate.
/// Note, that this is only done if the ident token is printed from inside of AST pretty-pringing,
/// but not otherwise. Pretty-printing is the only way for proc macros to discover token contents,
/// so we should not perform this lossy conversion if the top level call to the pretty-printer was
/// done for a token stream or a single token.
pub struct IdentPrinter {
  symbol: Symbol,
  is_raw: bool,
}

impl IdentPrinter {
  /// The most general `IdentPrinter` constructor. Do not use this.
  pub fn new(symbol: Symbol, is_raw: bool) -> IdentPrinter {
    IdentPrinter { symbol, is_raw }
  }

  /// This implementation is supposed to be used when printing identifiers
  /// as a part of pretty-printing for larger AST pieces.
  /// Do not use this either.
  pub fn for_ast_ident(ident: Ident, is_raw: bool) -> IdentPrinter {
    IdentPrinter::new(ident.name, is_raw)
  }
}

impl fmt::Display for IdentPrinter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_raw {
      f.write_str("r#")?;
    }

    fmt::Display::fmt(&self.symbol, f)
  }
}

/// An interned string.
///
/// Internally, a `Symbol` is implemented as an index, and all operations
/// (including hashing, equality, and ordering) operate on that index. The use
/// of `rustc_index::newtype_index!` means that `Option<Symbol>` only takes up 4 bytes,
/// because `rustc_index::newtype_index!` reserves the last 256 values for tagging purposes.
///
/// Note that `Symbol` cannot directly be a `rustc_index::newtype_index!` because it
/// implements `fmt::Debug`, `Encodable`, and `Decodable` in special ways.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(SymbolIndex);

rustc_index::newtype_index! {
    pub struct SymbolIndex { .. }
}

impl Symbol {
  const fn new(n: u32) -> Self {
    Symbol(SymbolIndex::from_u32(n))
  }

  /// Maps a string to its interned representation.
  pub fn intern(string: &str) -> Self {
    with_interner(|interner| interner.intern(string))
  }

  /// Convert to a `SymbolStr`. This is a slowish operation because it
  /// requires locking the symbol interner.
  pub fn as_str(self) -> SymbolStr {
    with_interner(|interner| unsafe {
      SymbolStr { string: std::mem::transmute::<&str, &str>(interner.get(self)) }
    })
  }

  pub fn as_u32(self) -> u32 {
    self.0.as_u32()
  }

  pub fn is_empty(self) -> bool {
    self == kw::Empty
  }

  /// This method is supposed to be used in error messages, so it's expected to be
  /// identical to printing the original identifier token written in source code
  /// (`token_to_string`, `Ident::to_string`), except that symbols don't keep the rawness flag
  /// or edition, so we have to guess the rawness using the global edition.
  pub fn to_ident_string(self) -> String {
    Ident::with_dummy_span(self).to_string()
  }
}

impl fmt::Debug for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.as_str(), f)
  }
}

impl fmt::Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.as_str(), f)
  }
}

impl<S: Encoder> Encodable<S> for Symbol {
  fn encode(&self, s: &mut S) -> Result<(), S::Error> {
    s.emit_str(&self.as_str())
  }
}

impl<D: Decoder> Decodable<D> for Symbol {
  #[inline]
  fn decode(d: &mut D) -> Result<Symbol, D::Error> {
    Ok(Symbol::intern(&d.read_str()?))
  }
}

impl<CTX> HashStable<CTX> for Symbol {
  #[inline]
  fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
    self.as_str().hash_stable(hcx, hasher);
  }
}

impl<CTX> ToStableHashKey<CTX> for Symbol {
  type KeyType = SymbolStr;

  #[inline]
  fn to_stable_hash_key(&self, _: &CTX) -> SymbolStr {
    self.as_str()
  }
}

// The `&'static str`s in this type actually point into the arena.
//
// The `FxHashMap`+`Vec` pair could be replaced by `FxIndexSet`, but #75278
// found that to regress performance up to 2% in some cases. This might be
// revisited after further improvements to `indexmap`.
#[derive(Default)]
pub struct Interner {
  arena: DroplessArena,
  names: FxHashMap<&'static str, Symbol>,
  strings: Vec<&'static str>,
}

impl Interner {
  fn prefill(init: &[&'static str]) -> Self {
    Interner {
      strings: init.into(),
      names: init.iter().copied().zip((0..).map(Symbol::new)).collect(),
      ..Default::default()
    }
  }

  #[inline]
  pub fn intern(&mut self, string: &str) -> Symbol {
    if let Some(&name) = self.names.get(string) {
      return name;
    }

    let name = Symbol::new(self.strings.len() as u32);

    // `from_utf8_unchecked` is safe since we just allocated a `&str` which is known to be
    // UTF-8.
    let string: &str =
      unsafe { str::from_utf8_unchecked(self.arena.alloc_slice(string.as_bytes())) };
    // It is safe to extend the arena allocation to `'static` because we only access
    // these while the arena is still alive.
    let string: &'static str = unsafe { &*(string as *const str) };
    self.strings.push(string);
    self.names.insert(string, name);
    name
  }

  // Get the symbol as a string. `Symbol::as_str()` should be used in
  // preference to this function.
  pub fn get(&self, symbol: Symbol) -> &str {
    self.strings[symbol.0.as_usize()]
  }
}

// This module has a very short name because it's used a lot.
/// This module contains all the defined keyword `Symbol`s.
///
/// Given that `kw` is imported, use them like `kw::keyword_name`.
/// For example `kw::Loop` or `kw::Break`.
pub mod kw {
  pub use super::kw_generated::*;
}

// This module has a very short name because it's used a lot.
/// This module contains all the defined non-keyword `Symbol`s.
///
/// Given that `sym` is imported, use them like `sym::symbol_name`.
/// For example `sym::rustfmt` or `sym::u8`.
pub mod sym {
  use super::Symbol;
  use std::convert::TryInto;

  #[doc(inline)]
  pub use super::sym_generated::*;

  /// Get the symbol for an integer.
  ///
  /// The first few non-negative integers each have a static symbol and therefore
  /// are fast.
  pub fn integer<N: TryInto<usize> + Copy + ToString>(n: N) -> Symbol {
    if let Result::Ok(idx) = n.try_into() {
      if idx < 10 {
        return Symbol::new(super::SYMBOL_DIGITS_BASE + idx as u32);
      }
    }
    Symbol::intern(&n.to_string())
  }
}

impl Symbol {
  pub fn is_reserved(self) -> bool {
    todo!()
  }

  /// Returns `true` if the symbol is `true` or `false`.
  pub fn is_bool_lit(self) -> bool {
    self == kw::True || self == kw::False
  }

  /// Returns `true` if this symbol can be a raw identifier.
  pub fn can_be_raw(self) -> bool {
    self != kw::Empty
  }
}

impl Ident {
  /// Returns `true` if the token is either a special identifier or a keyword.
  pub fn is_reserved(self) -> bool {
    // Note: `span.edition()` is relatively expensive, don't call it unless necessary.
    self.name.is_reserved()
  }

  /// We see this identifier in a normal identifier position, like variable name or a type.
  /// How was it written originally? Did it use the raw form? Let's try to guess.
  pub fn is_raw_guess(self) -> bool {
    self.name.can_be_raw() && self.is_reserved()
  }
}

#[inline]
fn with_interner<T, F: FnOnce(&mut Interner) -> T>(f: F) -> T {
  SESSION_GLOBALS.with(|session_globals| f(&mut *session_globals.symbol_interner.lock()))
}

/// An alternative to [`Symbol`], useful when the chars within the symbol need to
/// be accessed. It deliberately has limited functionality and should only be
/// used for temporary values.
///
/// Because the interner outlives any thread which uses this type, we can
/// safely treat `string` which points to interner data, as an immortal string,
/// as long as this type never crosses between threads.
//
// FIXME: ensure that the interner outlives any thread which uses `SymbolStr`,
// by creating a new thread right after constructing the interner.
#[derive(Clone, Eq, PartialOrd, Ord)]
pub struct SymbolStr {
  string: &'static str,
}

// This impl allows a `SymbolStr` to be directly equated with a `String` or
// `&str`.
impl<T: std::ops::Deref<Target = str>> std::cmp::PartialEq<T> for SymbolStr {
  fn eq(&self, other: &T) -> bool {
    self.string == other.deref()
  }
}

impl !Send for SymbolStr {}
impl !Sync for SymbolStr {}

/// This impl means that if `ss` is a `SymbolStr`:
/// - `*ss` is a `str`;
/// - `&*ss` is a `&str` (and `match &*ss { ... }` is a common pattern).
/// - `&ss as &str` is a `&str`, which means that `&ss` can be passed to a
///   function expecting a `&str`.
impl std::ops::Deref for SymbolStr {
  type Target = str;
  #[inline]
  fn deref(&self) -> &str {
    self.string
  }
}

impl fmt::Debug for SymbolStr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(self.string, f)
  }
}

impl fmt::Display for SymbolStr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self.string, f)
  }
}

impl<CTX> HashStable<CTX> for SymbolStr {
  #[inline]
  fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
    self.string.hash_stable(hcx, hasher)
  }
}

impl<CTX> ToStableHashKey<CTX> for SymbolStr {
  type KeyType = SymbolStr;

  #[inline]
  fn to_stable_hash_key(&self, _: &CTX) -> SymbolStr {
    self.clone()
  }
}
