use super::{Class, Fn, Generics, Lit, Spanned, TemplateLit, Ty};
use crate::ptr::P;
use tscore_span::{symbol::Ident, Span};

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum BinOpKind {
  /// The `+` operator (addition)
  Add,
  /// The `-` operator (subtraction)
  Sub,
  /// The `*` operator (multiplication)
  Mul,
  /// The `/` operator (division)
  Div,
  /// The `%` operator (modulus)
  Mod,
  /// The `&&` operator (logical and)
  And,
  /// The `||` operator (logical or)
  Or,
  /// The `??` operator (coalesce)
  Qq,
  /// The `^` operator (bitwise xor)
  BitXor,
  /// The `&` operator (bitwise and)
  BitAnd,
  /// The `|` operator (bitwise or)
  BitOr,
  /// The `<<` operator (shift left)
  Shl,
  /// The `>>` operator (signed shift right)
  Shr,
  /// The `>>>` operator (unsigned shift right)
  Ushr,
  /// The `**` operator (exponentiate)
  Exp,
  /// The `==` operator (equality with casting)
  EqEq,
  /// The `===` operator (strongly equality)
  EqEqEq,
  /// The `<` operator (less than)
  Lt,
  /// The `<=` operator (less than or equal to)
  Le,
  /// The `!=` operator (not equal to with casting)
  NotEq,
  /// The `!==` operator (strongly not equal to)
  NotEqEq,
  /// The `>=` operator (greater than or equal to)
  Ge,
  /// The `>` operator (greater than)
  Gt,
  /// The `in` operator (field detection)
  In,
  /// The `instanceof` operator
  InstanceOf,
}

pub type BinOp = Spanned<BinOpKind>;

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum AssignOpKind {
  /// `=`
  Assign,
  /// `+=`
  AddAssign,
  /// `-=`
  SubAssign,
  /// `*=`
  MulAssign,
  /// `/=`
  DivAssign,
  /// `%=`
  ModAssign,
  /// `<<=`
  ShlAssign,
  /// `>>=`
  ShrAssign,
  /// `>>>=`
  UshrAssign,
  /// `|=`
  BitOrAssign,
  /// `^=`
  BitXorAssign,
  /// `&==`
  BitAndAssign,
  /// `**=`
  ExpAssign,
  /// `&&=`
  AndAssign,
  /// `||=`
  OrAssign,
  /// `??=`
  QqAssign,
}

pub type AssignOp = Spanned<AssignOpKind>;

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct AssignExpr {
  pub span: Span,
  pub left: P<Expr>,
  pub operator: AssignOp,
  pub right: P<Expr>,
}

// #[derive(Clone, Encodable, Decodable, Debug)]
// pub enum PrimaryExpr {
//   This,
//   Super,
//   Lit(P<Lit>),
//   Ident(Ident),
//   Fn(P<Fn>),
//   Class(P<Class>),
//   Paren(P<Expr>),
// }

// #[derive(Clone, Encodable, Decodable, Debug)]
// pub enum LeftHandSideExpr {
//   Primary(PrimaryExpr),
//   New,
//   Call(P<Call>),
//   NonNull,
//   // `!`
//   // `as`
// }

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum UnaryOpKind {
  Del,
  Void,
  Typeof,
  Await,
  Plus,
  Minus,
  Tilde,
}

pub type UnaryOp = Spanned<UnaryOpKind>;

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum UpdateOpKind {
  PlusPlus,
  MinusMinus,
}

pub type UpdateOp = Spanned<UpdateOpKind>;

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct CondExpr {
  pub span: Span,
  pub test: P<Expr>,
  pub consequent: P<Expr>,
  pub alternate: P<Expr>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct PropAccessExpr {
  pub expr: P<Expr>,
  pub optional: bool,
  pub name: Ident,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct ElementAccessExpr {
  pub expr: P<Expr>,
  pub optional: bool,
  pub member: P<Expr>,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct CallExpr {
  pub span: Span,
  pub expr: P<Expr>,
  pub args: Vec<P<Expr>>,
  pub type_params: Generics,
  pub optional: bool,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct NewExpr {
  pub span: Span,
  pub expr: P<Expr>,
  pub args: Vec<P<Expr>>,
  pub type_params: Generics,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JsxEl {
  pub span: Span,
  /// optional for fragment element
  pub openingEl: Option<JsxOpeningEl>,
  /// optional for fragment or self closing element
  pub closingEl: Option<JsxClosingEl>,
  pub children: Vec<P<JsxChild>>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JsxProp {
  pub span: Span,
  pub name: Ident,
  pub init: P<Expr>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JsxOpeningEl {
  pub span: Span,
  pub tag: JsxTagNameKind,
  pub type_args: Generics,
  pub props: Vec<P<JsxProp>>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum JsxTagNameKind {
  Ident(Ident),
  This,
  PropAccess(P<PropAccessExpr>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JsxClosingEl {
  pub span: Span,
  pub tag: JsxTagNameKind,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JsxExpr {
  pub span: Span,
  pub spread: bool,
  pub epxr: P<Expr>,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum JsxChildKind {
  Text,
  Expr(P<JsxExpr>),
  El(P<JsxEl>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JsxChild {
  pub kind: JsxChildKind,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum ExprKind {
  Omitted,
  PartialEmitted(P<Expr>),
  Unary(UnaryOp, P<Expr>),
  Update(UpdateOp, P<Expr>, /* prefix */ bool),
  Binary(BinOp, P<Expr>, P<Expr>),
  Lit(P<Lit>),
  This,
  Super,
  Import,
  Yield(Option<P<Expr>>, /* TBD */ bool),
  // TBD
  // SyntheticExpression
  Cond(P<CondExpr>),
  Fn(P<Fn>),
  ArrowFn(P<Fn>),
  Paren(P<Expr>),
  Spread(P<Expr>),
  PropAccess(P<PropAccessExpr>),
  ElementAccess(P<ElementAccessExpr>),
  Call(P<CallExpr>),
  New(P<NewExpr>),
  TaggedTemplate(P<Expr>, P<TemplateLit>),
  Meta(Ident),
  JsxEl(P<JsxEl>),
  Assign(P<AssignExpr>),

  // ts expressions
  Assert(P<Ty>, P<Expr>),
  As(P<Expr>, P<Ty>),
  NonNull(P<Expr>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Expr {
  pub span: Span,
  pub kind: ExprKind,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct ExprWithTypeArgs {}
