use tscore_span::symbol::Ident;
use tscore_span::Span;

use super::{ExprWithTypeArgs, FnParam, Lit, PropName};
use crate::ptr::P;

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct GenericParam {
  pub name: Ident,
  pub constraint: Option<P<Ty>>,
  pub defualt: Option<P<Ty>>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Generics {
  pub span: Span,
  pub params: Vec<GenericParam>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct FnSig {
  pub generics: Generics,
  pub params: Vec<FnParam>,
  pub ret: FnRet,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum FnRet {
  Void,
  Ty(P<Ty>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum EntityName {
  Ident(Ident),
  /// ```ts
  /// type A = Module.Namespace.A
  ///          ^^^^^^^^^^^^^^^^ ^
  /// //        QualifiedName   Ident
  /// ```
  Qualified(P<EntityName>, Ident),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum RefTyKind {
  Ident(P<EntityName>, Vec<P<Ty>>),
  Expr(P<ExprWithTypeArgs>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct RefTy {
  pub span: Span,
  pub kind: RefTyKind,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct CondTy(
  /* check */ P<Ty>,
  /* extends */ P<Ty>,
  /* true */ P<Ty>,
  /* false */ P<Ty>,
);

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum OpTyKind {
  Keyof,
  Readonly,
  Uniq,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct OpTy {
  pub kind: OpTyKind,
  pub target: P<Ty>,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct IndexTy(/* object */ P<Ty>, /* index */ P<Ty>);

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct TemplateTySpan {
  pub span: Span,
  pub ty: P<Ty>,
  pub lit: Option<P<Ty>>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct TemplateTy {
  pub span: Span,
  pub head: Option<Span>,
  pub spans: Vec<P<TemplateTySpan>>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum ModFlag {
  Raw,
  Plus,
  Minus,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct MapTy {
  pub readonly_flag: Option<ModFlag>,
  pub optional_flag: Option<ModFlag>,
  pub type_params: Generics,
  pub optional: bool,
  pub ty: Ty,
  pub span: Span,
  /// TBD
  pub name_type: Option<Ty>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum TyKind {
  /// ```ts
  /// let a: import('module-name').A
  /// ```
  Import,
  /// ```ts
  /// class A {
  ///   a(a: this) {
  ///     // ^^^^
  ///   }
  /// }
  /// ```
  This,
  Fn(P<FnSig>),
  Constructor(P<FnSig>),
  /// ```ts
  /// type A = Dictionary<string, Person>
  /// //       ^^^^^^^^^^^^^^^^^^^^^^^^^^
  /// ```
  Ref(P<RefTy>),
  /// ```ts
  /// let a = 'foo'
  /// type A = typeof a
  /// //       ^^^^^^
  /// ```
  Query(P<EntityName>),
  /// ```ts
  /// let a: string
  /// //     ^^^^^^ StringKeyword
  /// ```
  Keyword,
  /// ```ts
  /// let a: string | null
  /// //              ^^^^
  /// ```
  Lit(P<Lit>),
  /// ```ts
  /// let a: { name: string }
  /// //     ^^^^^^^^^^^^^^^^
  /// ```
  TyLit(Vec<P<Ty>>),
  /// ```ts
  /// let a: A[]
  /// ```
  Arr(P<Ty>),
  /// ```ts
  /// let a: [number, string]
  /// ```
  Tuple(Vec<P<Ty>>),
  /// ```ts
  /// let a: [number, ...string, number]
  /// //              ^^^^^^^^^
  /// ```
  Rest(P<Ty>),
  /// ```ts
  ///
  /// let a: [number, ...string, number?]
  /// //                         ^^^^^^^
  /// ```
  Optional(P<Ty>),
  /// ```ts
  /// let a: A | B
  /// ```
  Union(Vec<P<Ty>>),
  /// ```ts
  /// let a: A & B
  /// ```
  Intersection(Vec<P<Ty>>),
  /// ```ts
  /// let a: A extends string ? A : B
  /// ```
  Cond(P<CondTy>),
  /// ```ts
  /// let a: keyof A
  /// let a: readonly A
  /// let a: unique symbol
  /// ```
  Op(P<OpTy>),
  /// ```ts
  /// let a: (A | B) & C
  /// //     ^^^^^^^
  /// ```
  Paren(P<Ty>),
  /// ```ts
  /// declare function A<T>(): T extends Any<infer U> ? U : never
  /// //                                     ^^^^^^^          
  /// ```
  Infer(P<GenericParam>),
  /// ```ts
  /// { [ P in K ] : T }
  /// { [ P in K ] ? : T }
  /// { -readonly [ P in K ] : T }
  /// { readonly [ P in K ] ? : T }
  /// ```
  Map(P<MapTy>),
  /// ```ts
  /// type P1 = Thing["name"];
  /// ```
  Index(P<IndexTy>),
  /// ```ts
  /// type EventName<T extends string> = `${T}Changed`;
  /// type T3 = `${'top' | 'bottom'}-${'left' | 'right'}`;
  /// ```
  Template(P<TemplateTy>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Ty {
  pub kind: TyKind,
  pub name: Option<PropName>,
  pub optional: bool,
  pub span: Span,
}
