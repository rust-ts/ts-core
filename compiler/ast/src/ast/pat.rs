use super::{ArrayLit, ObjectLit};
use crate::ptr::P;
use tscore_span::symbol::Ident;

#[derive(Clone, Encodable, Decodable, Debug)]
pub enum PatKind {
  Ident(Ident),
  Array(P<ArrayLit>),
  Object(P<ObjectLit>),
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Pat {
  kind: PatKind,
}
