use tscore_span::{symbol::Ident, Span};

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JSDocTag {
  pub name: Ident,
  pub span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct JSDoc {
  pub tags: Vec<JSDocTag>,
  pub span: Span,
}
