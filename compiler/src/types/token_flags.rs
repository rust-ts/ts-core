#[rustfmt::skip]
pub mod TokenFlags {
  pub type TokenFlag = u16;
  pub const None: u16 = 0;
  /* @internal */
  pub(crate) const PrecedingLineBreak: u16 = 1 << 0;
  /* @internal */
  pub(crate) const PrecedingJSDocComment: u16 = 1 << 1;
  /* @internal */
  pub(crate) const Unterminated: u16 = 1 << 2;
  /* @internal */
  pub(crate) const ExtendedUnicodeEscape: u16 = 1 << 3;
  pub const Scientific: u16 = 1 << 4;        // e.g. `10e2`
  pub const Octal: u16 = 1 << 5;             // e.g. `0777`
  pub const HexSpecifier: u16 = 1 << 6;      // e.g. `0x00000000`
  pub const BinarySpecifier: u16 = 1 << 7;   // e.g. `0b0110010000000000`
  pub const OctalSpecifier: u16 = 1 << 8;    // e.g. `0o777`
  /* @internal */
  pub(crate) const ContainsSeparator: u16 = 1 << 9; // e.g. `0b1100_0101`
  /* @internal */
  pub(crate) const UnicodeEscape: u16 = 1 << 10;
  /* @internal */
  pub(crate) const ContainsInvalidEscape: u16 = 1 << 11;    // e.g. `\uhello`
  /* @internal */
  pub(crate) const BinaryOrOctalSpecifier: u16 = BinarySpecifier | OctalSpecifier;
  /* @internal */
  pub(crate) const NumericLiteralFlags: u16 = Scientific | Octal | HexSpecifier | BinaryOrOctalSpecifier | ContainsSeparator;
  /* @internal */
  pub(crate) const TemplateLiteralLikeFlags: u16 = ContainsInvalidEscape;
}
