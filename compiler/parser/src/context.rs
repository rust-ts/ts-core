#[rustfmt::skip]
pub enum ParsingContext {
  SourceElements,            // Elements in source file
  BlockStatements,           // Statements in block
  SwitchClauses,             // Clauses in switch statement
  SwitchClauseStatements,    // Statements in switch clause
  TypeMembers,               // Members in interface or type literal
  ClassMembers,              // Members in class declaration
  EnumMembers,               // Members in enum declaration
  HeritageClauseElement,     // Elements in a heritage clause
  VariableDeclarations,      // Variable declarations in variable statement
  ObjectBindingElements,     // Binding elements in object binding list
  ArrayBindingElements,      // Binding elements in array binding list
  ArgumentExpressions,       // Expressions in argument list
  ObjectLiteralMembers,      // Members in object literal
  JsxAttributes,             // Attributes in jsx element
  JsxChildren,               // Things between opening and closing JSX tags
  ArrayLiteralMembers,       // Members in array literal
  Parameters,                // Parameters in parameter list
  JSDocParameters,           // JSDoc parameters in parameter list of JSDoc function type
  RestProperties,            // Property names in a rest type list
  TypeParameters,            // Type parameters in type parameter list
  TypeArguments,             // Type arguments in type argument list
  TupleElementTypes,         // Element types in tuple element type list
  HeritageClauses,           // Heritage clauses for a class or interface declaration.
  ImportOrExportSpecifiers,  // Named import clause's import specifier list
  Count                      // Number of parsing contexts
}

pub mod TokenFlags {
  pub const None: u16 = 0;
  pub const PrecedingLineBreak: u16 = 1 << 0;
  pub const PrecedingJSDocComment: u16 = 1 << 1;
  pub const Unterminated: u16 = 1 << 2;
  pub const ExtendedUnicodeEscape: u16 = 1 << 3;
  pub const Scientific: u16 = 1 << 4; // e.g. `10e2`
  pub const Octal: u16 = 1 << 5; // e.g. `0777`
  pub const HexSpecifier: u16 = 1 << 6; // e.g. `0x00000000`
  pub const BinarySpecifier: u16 = 1 << 7; // e.g. `0b0110010000000000`
  pub const OctalSpecifier: u16 = 1 << 8; // e.g. `0o777`
  pub const ContainsSeparator: u16 = 1 << 9; // e.g. `0b1100_0101`
  pub const UnicodeEscape: u16 = 1 << 10;
  pub const ContainsInvalidEscape: u16 = 1 << 11; // e.g. `\uhello`
  pub const BinaryOrOctalSpecifier: u16 = BinarySpecifier | OctalSpecifier;
  pub const NumericLiteralFlags: u16 =
    Scientific | Octal | HexSpecifier | BinaryOrOctalSpecifier | ContainsSeparator;
  pub const TemplateLiteralLikeFlags: u16 = ContainsInvalidEscape | ContainsInvalidEscape;
}
