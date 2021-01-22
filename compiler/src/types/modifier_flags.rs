#[rustfmt::skip]
pub mod ModifierFlags {
  pub type ModifierFlag = u32;
  pub const None: u32 = 0;
  pub const Export: u32 = 1 << 0;     // Declarations
  pub const Ambient: u32 = 1 << 1;    // Declarations
  pub const Public: u32 = 1 << 2;     // Property/Method
  pub const Private: u32 = 1 << 3;    // Property/Method
  pub const Protected: u32 = 1 << 4;  // Property/Method
  pub const Static: u32 = 1 << 5;     // Property/Method
  pub const Readonly: u32 = 1 << 6;   // Property/Method
  pub const Abstract: u32 = 1 << 7;   // Class/Method/ConstructSignature
  pub const Async: u32 = 1 << 8;      // Property/Method/Function
  pub const Default: u32 = 1 << 9;    // Function/Class (export default declaration)
  pub const Const: u32 = 1 << 11;     // Const enum
  pub const HasComputedJSDocModifiers: u32 = 1 << 12; // Indicates the computed modifier flags include modifiers from JSDoc.

  pub const Deprecated: u32 = 1 << 13;        // Deprecated tag.
  pub const HasComputedFlags: u32 = 1 << 29;  // Modifier flags have been computed

  pub const AccessibilityModifier: u32 = Public | Private | Protected;
  // Accessibility modifiers and 'readonly' can be attached to a parameter in a constructor to make it a property.
  pub const ParameterPropertyModifier: u32 = AccessibilityModifier | Readonly;
  pub const NonPublicAccessibilityModifier: u32 = Private | Protected;

  pub const TypeScriptModifier: u32 =
    Ambient | Public | Private | Protected | Readonly | Abstract | Const;
  pub const ExportDefault: u32 = Export | Default;
  pub const All: u32 = Export
    | Ambient
    | Public
    | Private
    | Protected
    | Static
    | Readonly
    | Abstract
    | Async
    | Default
    | Const
    | Deprecated;
}
