use std::collections::HashMap;

mod character_codes;
pub mod comment_directive;
pub mod compiler_options;
pub mod jsx_flags;
pub mod modifier_flags;
pub mod node_flags;
pub mod source_file;
pub mod syntax_kind;
pub mod text_range;
pub mod token_flags;

pub use character_codes::CharacterCodes;
pub use jsx_flags::JsxFlags::{self, JsxFlag};
pub use modifier_flags::ModifierFlags::{self, ModifierFlag};
pub use syntax_kind::SyntaxKinds::{self, SyntaxKind};
pub use token_flags::TokenFlags::{self, TokenFlag};

pub mod relation_comparison_result {
  pub const Succeeded: u8 = 1 << 0; // Should be truthy
  pub const Failed: u8 = 1 << 1;
  pub const Reported: u8 = 1 << 2;

  pub const ReportsUnmeasurable: u8 = 1 << 3;
  pub const ReportsUnreliable: u8 = 1 << 4;
  pub const ReportsMask: u8 = ReportsUnmeasurable | ReportsUnreliable;
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticCategory {
  Warning,
  Error,
  Suggestion,
  Message,
}

pub struct DiagnosticMessage {
  pub key: String,
  pub category: DiagnosticCategory,
  pub code: u64,
  pub message: String,
  pub reports_unnecessary: Option<HashMap<String, String>>,
  pub reports_deprecated: Option<HashMap<String, String>>,
  pub(crate) elided_in_compatability_pyramid: Option<bool>,
}
