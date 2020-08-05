pub mod compiler_options;
pub mod jsx_flags;
pub mod node_flags;
pub mod syntax_kind;
pub mod text_range;

pub mod relation_comparison_result {
  pub const Succeeded: u8 = 1 << 0; // Should be truthy
  pub const Failed: u8 = 1 << 1;
  pub const Reported: u8 = 1 << 2;

  pub const ReportsUnmeasurable: u8 = 1 << 3;
  pub const ReportsUnreliable: u8 = 1 << 4;
  pub const ReportsMask: u8 = ReportsUnmeasurable | ReportsUnreliable;
}
