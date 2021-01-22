use crate::text_range::TextRange;

pub struct CommentDirective {
  range: TextRange,
  directive_type: CommentDirectiveType,
}

pub enum CommentDirectiveType {
  ExpectError,
  Ignore,
}
