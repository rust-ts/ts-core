#[rustfmt::skip]
pub mod NodeFlags {
  pub type NodeFlag = u32;
  pub const None: u32               = 0;
  pub const Let: u32                = 1 << 0;              // Variable declaration
  pub const Const: u32              = 1 << 1;              // Variable declaration
  pub const NestedNamespace: u32    = 1 << 2;              // Namespace declaration
  pub const Synthesized: u32        = 1 << 3;              // Node was synthesized during transformation
  pub const Namespace: u32          = 1 << 4;              // Namespace declaration
  pub const OptionalChain: u32      = 1 << 5;              // Chained MemberExpression rooted to a pseudo-OptionalExpression
  pub const ExportContext: u32      = 1 << 6;              // Export context (initialized by binding)
  pub const ContainsThis: u32       = 1 << 7;              // Interface contains references to "this"
  pub const HasImplicitReturn: u32  = 1 << 8;              // If function implicitly returns on one of codepaths (initialized by binding)
  pub const HasExplicitReturn: u32  = 1 << 9;              // If function has explicit reachable return on one of codepaths (initialized by binding)
  pub const GlobalAugmentation: u32 = 1 << 10;             // Set if module declaration is an augmentation for the global scope
  pub const HasAsyncFunctions: u32  = 1 << 11;             // If the file has async functions (initialized by binding)
  pub const DisallowInContext: u32  = 1 << 12;             // If node was parsed in a context where 'in-expressions' are not allowed
  pub const YieldContext: u32       = 1 << 13;             // If node was parsed in the 'yield' context created when parsing a generator
  pub const DecoratorContext: u32   = 1 << 14;             // If node was parsed as part of a decorator
  pub const AwaitContext: u32       = 1 << 15;             // If node was parsed in the 'await' context created when parsing an async function
  pub const ThisNodeHasError: u32   = 1 << 16;             // If the parser encountered an error when parsing the code that created this node
  pub const JavaScriptFile: u32     = 1 << 17;             // If node was parsed in a JavaScript
  pub const ThisNodeOrAnySubNodesHasError: u32 = 1 << 18;  // If this node or any of its children had an error
  pub const HasAggregatedChildData: u32 = 1 << 19;

  // These flags will be set when the parser encounters a dynamic import expression or 'import.meta' to avoid
  // walking the tree if the flags are not set. However, these flags are just a approximation
  // (hence why it's named "PossiblyContainsDynamicImport") because once set, the flags never get cleared.
  // During editing, if a dynamic import is removed, incremental parsing will *NOT* clear this flag.
  // This means that the tree will always be traversed during module resolution, or when looking for external module indicators.
  // However, the removal operation should not occur often and in the case of the
  // removal, it is likely that users will add the import anyway.
  // The advantage of this approach is its simplicity. For the case of batch compilation,
  // we guarantee that users won't have to pay the price of walking the tree if a dynamic import isn't used.
  pub const PossiblyContainsDynamicImport: u32 = 1 << 20;
  pub const PossiblyContainsImportMeta: u32    = 1 << 21;

  pub const JSDoc: u32                         = 1 << 22; // If node was parsed inside jsdoc
  pub const Ambient: u32                       = 1 << 23; // If node was inside an ambient context -- a declaration file, or inside something with the `declare` modifier.
  pub const InWithStatement: u32               = 1 << 24; // If any ancestor of node was the `statement` of a WithStatement (not the `expression`)
  pub const JsonFile: u32                      = 1 << 25; // If node was parsed in a Json
  pub const TypeCached: u32                    = 1 << 26; // If a type was cached for node at any point
  pub const Deprecated: u32                    = 1 << 27; // If has '@deprecated' JSDoc tag
  
  pub const BlockScoped: u32 = Let | Const;

  pub const ReachabilityCheckFlags: u32 = HasImplicitReturn | HasExplicitReturn;
  pub const ReachabilityAndEmitFlags: u32 = ReachabilityCheckFlags | HasAsyncFunctions;

  // Parsing context flags
  pub const ContextFlags: u32 = DisallowInContext | YieldContext | DecoratorContext | AwaitContext | JavaScriptFile | InWithStatement | Ambient;

  // Exclude these flags when parsing a Type
  pub const TypeExcludesFlags: u32 = YieldContext | AwaitContext;

  // Represents all flags that are potentially set once and
  // never cleared on SourceFiles which get re-used in between incremental parses.
  // See the comment above on `PossiblyContainsDynamicImport` and `PossiblyContainsImportMeta`.
  /* @internal */
  pub const PermanentlySetIncrementalFlags: u32 = PossiblyContainsDynamicImport | PossiblyContainsImportMeta;
}

#[rustfmt::skip]
pub mod ModifierFlags {
  pub type ModifierFlag = u32;
  pub const None: u32 =               0;
  pub const Export: u32 =             1 << 0;  // Declarations
  pub const Ambient: u32 =            1 << 1;  // Declarations
  pub const Public: u32 =             1 << 2;  // Property/Method
  pub const Private: u32 =            1 << 3;  // Property/Method
  pub const Protected: u32 =          1 << 4;  // Property/Method
  pub const Static: u32 =             1 << 5;  // Property/Method
  pub const Readonly: u32 =           1 << 6;  // Property/Method
  pub const Abstract: u32 =           1 << 7;  // Class/Method/ConstructSignature
  pub const Async: u32 =              1 << 8;  // Property/Method/Function
  pub const Default: u32 =            1 << 9;  // Function/Class (export default declaration)
  pub const Const: u32 =              1 << 11; // Const enum
  pub const HasComputedJSDocModifiers: u32 = 1 << 12; // Indicates the computed modifier flags include modifiers from JSDoc.

  pub const Deprecated: u32 =         1 << 13; // Deprecated tag.
  pub const HasComputedFlags: u32 =   1 << 29; // Modifier flags have been computed 

  
  pub const AccessibilityModifier: u32 =  Public | Private | Protected;
  // Accessibility modifiers and 'readonly' can be attached to a parameter in a constructor to make it a property.
  pub const ParameterPropertyModifier: u32 =  AccessibilityModifier | Readonly;
  pub const NonPublicAccessibilityModifier: u32 =  Private | Protected;

  pub const TypeScriptModifier: u32 =  Ambient | Public | Private | Protected | Readonly | Abstract | Const;
  pub const ExportDefault: u32 =  Export | Default;
  pub const All: u32 =  Export | Ambient | Public | Private | Protected | Static | Readonly | Abstract | Async | Default | Const | Deprecated;
}

pub mod JsxFlags {
  pub const None: u8 = 0;
  /** An element from a named property of the JSX.IntrinsicElements interface */
  pub const IntrinsicNamedElement: u8 = 1 << 0;
  /** An element inferred from the string index signature of the JSX.IntrinsicElements interface */
  pub const IntrinsicIndexedElement: u8 = 1 << 1;

  pub const IntrinsicElement: u8 = IntrinsicNamedElement | IntrinsicIndexedElement;
}

#[rustfmt::skip]
pub mod RelationComparisonResult {
  pub const Succeeded: u32           = 1 << 0; // Should be truthy;
  pub const Failed: u32              = 1 << 1;
  pub const Reported: u32            = 1 << 2;

  pub const ReportsUnmeasurable: u32 = 1 << 3;
  pub const ReportsUnreliable: u32   = 1 << 4;
  pub const ReportsMask: u32         = ReportsUnmeasurable | ReportsUnreliable;
}

#[rustfmt::skip]
pub mod TransformFlags {
  pub type TransformFlag = u32;
  pub const None: u32 = 0;

  // Facts
  // - Flags used to indicate that a node or subtree contains syntax that requires transformation.
  pub const ContainsTypeScript: u32 = 1 << 0;
  pub const ContainsJsx: u32 = 1 << 1;
  pub const ContainsESNext: u32 = 1 << 2;
  pub const ContainsES2020: u32 = 1 << 3;
  pub const ContainsES2019: u32 = 1 << 4;
  pub const ContainsES2018: u32 = 1 << 5;
  pub const ContainsES2017: u32 = 1 << 6;
  pub const ContainsES2016: u32 = 1 << 7;
  pub const ContainsES2015: u32 = 1 << 8;
  pub const ContainsGenerator: u32 = 1 << 9;
  pub const ContainsDestructuringAssignment: u32 = 1 << 10;

  // Markers
  // - Flags used to indicate that a subtree contains a specific transformation.
  pub const ContainsTypeScriptClassSyntax: u32 = 1 << 11; // Decorators, Property Initializers, Parameter Property Initializers
  pub const ContainsLexicalThis: u32 = 1 << 12;
  pub const ContainsRestOrSpread: u32 = 1 << 13;
  pub const ContainsObjectRestOrSpread: u32 = 1 << 14;
  pub const ContainsComputedPropertyName: u32 = 1 << 15;
  pub const ContainsBlockScopedBinding: u32 = 1 << 16;
  pub const ContainsBindingPattern: u32 = 1 << 17;
  pub const ContainsYield: u32 = 1 << 18;
  pub const ContainsAwait: u32 = 1 << 19;
  pub const ContainsHoistedDeclarationOrCompletion: u32 = 1 << 20;
  pub const ContainsDynamicImport: u32 = 1 << 21;
  pub const ContainsClassFields: u32 = 1 << 22;
  pub const ContainsPossibleTopLevelAwait: u32 = 1 << 23;

  // Please leave this as 1 << 29.
  // It is the maximum bit we can set before we outgrow the size of a v8 small integer (SMI) on an x86 system.
  // It is a good reminder of how much room we have left
  pub const HasComputedFlags: u32 = 1 << 29; // Transform flags have been computed.

  // Assertions
  // - Bitmasks that are used to assert facts about the syntax of a node and its subtree.
  pub const AssertTypeScript: u32 = ContainsTypeScript;
  pub const AssertJsx: u32 = ContainsJsx;
  pub const AssertESNext: u32 = ContainsESNext;
  pub const AssertES2020: u32 = ContainsES2020;
  pub const AssertES2019: u32 = ContainsES2019;
  pub const AssertES2018: u32 = ContainsES2018;
  pub const AssertES2017: u32 = ContainsES2017;
  pub const AssertES2016: u32 = ContainsES2016;
  pub const AssertES2015: u32 = ContainsES2015;
  pub const AssertGenerator: u32 = ContainsGenerator;
  pub const AssertDestructuringAssignment: u32 = ContainsDestructuringAssignment;

  // Scope Exclusions
  // - Bitmasks that exclude flags from propagating out of a specific context
  //   into the subtree flags of their container.
  pub const OuterExpressionExcludes: u32 = HasComputedFlags;
  pub const PropertyAccessExcludes: u32 = OuterExpressionExcludes;
  pub const NodeExcludes: u32 = PropertyAccessExcludes;
  pub const ArrowFunctionExcludes: u32 = NodeExcludes | ContainsTypeScriptClassSyntax | ContainsBlockScopedBinding | ContainsYield | ContainsAwait | ContainsHoistedDeclarationOrCompletion | ContainsBindingPattern | ContainsObjectRestOrSpread | ContainsPossibleTopLevelAwait;
  pub const FunctionExcludes: u32 = NodeExcludes | ContainsTypeScriptClassSyntax | ContainsLexicalThis | ContainsBlockScopedBinding | ContainsYield | ContainsAwait | ContainsHoistedDeclarationOrCompletion | ContainsBindingPattern | ContainsObjectRestOrSpread | ContainsPossibleTopLevelAwait;
  pub const ConstructorExcludes: u32 = NodeExcludes | ContainsLexicalThis | ContainsBlockScopedBinding | ContainsYield | ContainsAwait | ContainsHoistedDeclarationOrCompletion | ContainsBindingPattern | ContainsObjectRestOrSpread | ContainsPossibleTopLevelAwait;
  pub const MethodOrAccessorExcludes: u32 = NodeExcludes | ContainsLexicalThis | ContainsBlockScopedBinding | ContainsYield | ContainsAwait | ContainsHoistedDeclarationOrCompletion | ContainsBindingPattern | ContainsObjectRestOrSpread;
  pub const PropertyExcludes: u32 = NodeExcludes | ContainsLexicalThis;
  pub const ClassExcludes: u32 = NodeExcludes | ContainsTypeScriptClassSyntax | ContainsComputedPropertyName;
  pub const ModuleExcludes: u32 = NodeExcludes | ContainsTypeScriptClassSyntax | ContainsLexicalThis | ContainsBlockScopedBinding | ContainsHoistedDeclarationOrCompletion | ContainsPossibleTopLevelAwait;
  pub const TypeExcludes: u32 = !ContainsTypeScript;
  pub const ObjectLiteralExcludes: u32 = NodeExcludes | ContainsTypeScriptClassSyntax | ContainsComputedPropertyName | ContainsObjectRestOrSpread;
  pub const ArrayLiteralOrCallOrNewExcludes: u32 = NodeExcludes | ContainsRestOrSpread;
  pub const VariableDeclarationListExcludes: u32 = NodeExcludes | ContainsBindingPattern | ContainsObjectRestOrSpread;
  pub const ParameterExcludes: u32 = NodeExcludes;
  pub const CatchClauseExcludes: u32 = NodeExcludes | ContainsObjectRestOrSpread;
  pub const BindingPatternExcludes: u32 = NodeExcludes | ContainsRestOrSpread;

  // Propagating flags
  // - Bitmasks for flags that should propagate from a child
  pub const PropertyNamePropagatingFlags: u32 = ContainsLexicalThis;

  // Masks
  // - Additional bitmasks
}

#[rustfmt::skip]
pub mod EmitFlags {
  pub type EmitFlag = u32;
  pub const None: u32 = 0;
  pub const SingleLine: u32 = 1 << 0;                    // The contents of this node should be emitted on a single line.
  pub const AdviseOnEmitNode: u32 = 1 << 1;              // The printer should invoke the onEmitNode callback when printing this node.
  pub const NoSubstitution: u32 = 1 << 2;                // Disables further substitution of an expression.
  pub const CapturesThis: u32 = 1 << 3;                  // The function captures a lexical `this`
  pub const NoLeadingSourceMap: u32 = 1 << 4;            // Do not emit a leading source map location for this node.
  pub const NoTrailingSourceMap: u32 = 1 << 5;           // Do not emit a trailing source map location for this node.
  pub const NoSourceMap: u32 = NoLeadingSourceMap | NoTrailingSourceMap; // Do not emit a source map location for this node.
  pub const NoNestedSourceMaps: u32 = 1 << 6;            // Do not emit source map locations for children of this node.
  pub const NoTokenLeadingSourceMaps: u32 = 1 << 7;      // Do not emit leading source map location for token nodes.
  pub const NoTokenTrailingSourceMaps: u32 = 1 << 8;     // Do not emit trailing source map location for token nodes.
  pub const NoTokenSourceMaps: u32 = NoTokenLeadingSourceMaps | NoTokenTrailingSourceMaps; // Do not emit source map locations for tokens of this node.
  pub const NoLeadingComments: u32 = 1 << 9;             // Do not emit leading comments for this node.
  pub const NoTrailingComments: u32 = 1 << 10;           // Do not emit trailing comments for this node.
  pub const NoComments: u32 = NoLeadingComments | NoTrailingComments; // Do not emit comments for this node.
  pub const NoNestedComments: u32 = 1 << 11;
  pub const HelperName: u32 = 1 << 12;                   // The Identifier refers to an *unscoped* emit helper (one that is emitted at the top of the file)
  pub const ExportName: u32 = 1 << 13;                   // Ensure an export prefix is added for an identifier that points to an exported declaration with a local name (see SymbolFlags.ExportHasLocal).
  pub const LocalName: u32 = 1 << 14;                    // Ensure an export prefix is not added for an identifier that points to an exported declaration.
  pub const InternalName: u32 = 1 << 15;                 // The name is internal to an ES5 class body function.
  pub const Indented: u32 = 1 << 16;                     // Adds an explicit extra indentation level for class and function bodies when printing (used to match old emitter).
  pub const NoIndentation: u32 = 1 << 17;                // Do not indent the node.
  pub const AsyncFunctionBody: u32 = 1 << 18;
  pub const ReuseTempVariableScope: u32 = 1 << 19;       // Reuse the existing temp variable scope during emit.
  pub const CustomPrologue: u32 = 1 << 20;               // Treat the statement as if it were a prologue directive (NOTE: Prologue directives are *not* transformed).
  pub const NoHoisting: u32 = 1 << 21;                   // Do not hoist this declaration in --module system
  pub const HasEndOfDeclarationMarker: u32 = 1 << 22;    // Declaration has an associated NotEmittedStatement to mark the end of the declaration
  pub const Iterator: u32 = 1 << 23;                     // The expression to a `yield*` should be treated as an Iterator when down-leveling, not an Iterable.
  pub const NoAsciiEscaping: u32 = 1 << 24;              // When synthesizing nodes that lack an original node or textSourceNode, we want to write the text on the node with ASCII escaping substitutions.
  /*@internal*/
  pub const TypeScriptClassWrapper: u32 = 1 << 25; // The node is an IIFE class wrapper created by the ts transform.
  /*@internal*/
  pub const NeverApplyImportHelper: u32 = 1 << 26; // Indicates the node should never be wrapped with an import star helper (because, for example, it imports tslib itself)
  /*@internal*/ 
  pub const IgnoreSourceNewlines: u32 = 1 << 27;   // Overrides `printerOptions.preserveSourceNewlines` to print this node (and all descendants) with default whitespace.
}
