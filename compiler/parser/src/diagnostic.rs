use tscore_ast::SourceFile;

pub enum DiagnosticCategory {
  Warning,
  Error,
  Suggestion,
  Message,
}

pub struct DiagnosticMessage {
  msg_text: String,
  category: DiagnosticCategory,
  code: usize,
}

pub struct DiagnosticRelatedInformation {
  category: DiagnosticCategory,
  code: usize,
  file: Option<SourceFile>,
  start: Option<usize>,
  length: Option<usize>,
  msg_text: Vec<DiagnosticMessage>,
}

pub struct Diagnostic {
  reports_unnecessary: Option</* TBD */ u8>,
  reports_deprecated: Option</* TBD */ u8>,
  pub source: Option<String>,
  pub related_info: Option<Vec<DiagnosticRelatedInformation>>,
  pub info: DiagnosticRelatedInformation,
  /// ?
  pub(crate) skipped_on: Option<String>,
}

pub struct DiagnosticWithDetachedLocation {
  file: String,
  start: usize,
  length: usize,
  diagnostic: Diagnostic,
}
