use crate::ptr::P;
use crate::token::{self, CommentKind, DelimToken, Token};
use crate::tokenstream::{DelimSpan, LazyTokenStream, TokenStream, TokenTree};

use super::{Decl, DeclList, Expr, Pat, StrLit, VarDeclList};

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
pub struct Block {
  pub stmts: Vec<Stmt>,
  pub span: Span,
  pub multi_line: Option<bool>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum ForVal {
  Expr(P<Expr>),
  Decl(P<VarDeclList>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct ForCond {
  pub init: Option<P<Expr>>,
  pub test: Option<P<Expr>>,
  pub update: Option<P<Expr>>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum IterCondKind {
  Do(P<Expr>),
  For(P<ForCond>),
  ForInOf(P<ForVal>, P<Expr>, /* awaitness */ bool),
  While(P<Expr>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct IterCond {
  pub kind: IterCondKind,
  pub stmts: Vec<Stmt>,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Label(Ident);

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum SwitchBranch {
  Case(P<Expr>, Vec<Stmt>),
  Default(Vec<Stmt>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct CatchBlock {
  decl: Option</* VarDecl */ Decl>,
  block: Block,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum StmtKind {
  NotEmitted,
  EndOfDeclMark,
  MergeDeclMark,
  /// An empty statement is used to provide no statement,
  /// although the JavaScript syntax would expect one.
  Empty,
  /// The `debugger` statement invokes any available debugging functionality, such as setting a breakpoint.
  /// If no debugging functionality is available, this statement has no effect.
  Debugger,
  /// The `block` statement (or compound statement in other languages) is used to group zero or more `statements`.
  ///
  /// E.g., `{ .. }` as  in `function foo() { .. }`.
  Block(P<Block>),
  /// The `if` statement executes a statement if a specified `expression` returns truthy.
  /// Otherwise, another optional statement can be executed.
  ///
  /// `if (expr) { block } else { block }`
  If(P<Expr>, P<Stmt>, Option<P<Stmt>>),
  /// General iteration statement.
  ///
  /// See [IterCond] for more detail.
  Iter(P<IterCond>, P<Stmt>),
  /// The `break` statement terminates the current loop, switch, or label statement
  /// and transfers program control to the statement following the terminated statement.
  Break(Option<Label>),
  /// The `continue` statement terminates execution of the statements in the current iteration of the current or labeled loop, and continues execution of the loop with the next iteration.
  Continue(Option<Label>),
  /// The `return` statement ends function execution and specifies a value to be returned to the function caller.
  Ret(Option<P<Expr>>),
  /// The `with` statement extends the scope chain for a statement.
  ///
  /// ```js
  /// with(o) {
  ///   console.log(o)
  /// }
  /// ```
  With(P<Expr>, P<Stmt>),
  /// A `switch` statement evaluates an `expression`, matching the expression's value to a case clause,
  /// and executes statements associated with that case, as well as statements in cases that follow the matching case.
  ///
  /// ```js
  /// switch (name) {
  ///   ..
  /// }
  /// ```
  Switch(P<Expr>, P<Block>),
  /// A `case` or `default` branch in `switch`.
  ///
  /// ```js
  /// switch (name) {
  ///   case 'foo':
  ///     name + 'foo'
  ///     break;
  ///   default:
  ///     name + 'bar'
  /// }
  /// ```
  SwitchBranch(SwitchBranch),
  /// The labeled statement can be used with break or continue statements.
  /// It is prefixing a statement with an identifier which you can refer to.
  ///
  /// ```js
  /// loop1:
  /// for (i = 0; i < 3; i++) {     //The first for statement is labeled "loop1"
  ///   loop2:
  ///   for (j = 0; j < 3; j++) {   //The second for statement is labeled "loop2"
  ///     if (i === 1 && j === 1) {
  ///       continue loop1
  ///     }
  ///     console.log('i = ' + i + ', j = ' + j)
  ///   }
  /// }
  /// ```
  Label(Label, P<Stmt>),
  /// The `throw` statement throws a user-defined exception.
  Throw(P<Expr>),
  /// The try...catch statement marks a block of statements to try
  /// and specifies a response should an exception be thrown.
  ///
  /// `catch` block and `finally` block are both optional.
  ///
  /// ```js
  /// try {
  ///   nonExistentFunction()
  /// } catch (error) {
  ///   console.error(error)
  /// } finally {
  ///   cleanup()
  /// }
  /// ```
  Try(P<Block>, Option<P<CatchBlock>>, Option<Block>),
  /// The general declaration statement.
  ///
  /// See [Decl] for more detail.
  Decl(P<Decl>),
  /// General JavaScript/TypeScript expression.
  ///
  /// See [Expr] for more detail.
  Expr(P<Expr>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Stmt {
  pub kind: StmtKind,
  pub span: Span,
}
