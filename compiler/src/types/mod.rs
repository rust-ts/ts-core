use std::collections::HashMap;

mod character_codes;
pub mod compiler_options;
pub mod jsx_flags;
pub mod node_flags;
pub mod syntax_kind;
pub mod text_range;

pub use character_codes::CharacterCodes;

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

pub trait SourcefileLike {
  fn get_position_of_line_and_character(line: u64, character: u32, allow_edits: bool) -> u64;
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
