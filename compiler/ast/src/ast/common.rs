use tscore_span::symbol::Ident;
use tscore_span::Span;

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct NameBinding {
  name: Ident,
  /// with preceding `as` keyword
  alias: Option<Ident>,
  span: Span,
}

#[derive(Clone, Encodable, Decodable, Debug)]
pub struct Spanned<T> {
  pub node: T,
  pub span: Span,
}
