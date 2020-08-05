pub const NONE: u32 = 0;
pub const Let: u32 = 1 << 0; // Variable declaration
pub const Const: u32 = 1 << 1; // Variable declaration
pub const NestedNamespace: u32 = 1 << 2; // Namespace declaration
pub const Synthesized: u32 = 1 << 3; // Node was synthesized during transformation
pub const Namespace: u32 = 1 << 4; // Namespace declaration
pub const OptionalChain: u32 = 1 << 5; // Chained MemberExpression rooted to a pseudo-OptionalExpression
pub const ExportContext: u32 = 1 << 6; // Export context (initialized by binding)
pub const ContainsThis: u32 = 1 << 7; // Interface contains references to "this"
pub const HasImplicitReturn: u32 = 1 << 8; // If function implicitly returns on one of codepaths (initialized by binding)
pub const HasExplicitReturn: u32 = 1 << 9; // If function has explicit reachable return on one of codepaths (initialized by binding)
pub const GlobalAugmentation: u32 = 1 << 10; // Set if module declaration is an augmentation for the global scope
pub const HasAsyncFunctions: u32 = 1 << 11; // If the file has async functions (initialized by binding)
pub const DisallowInContext: u32 = 1 << 12; // If node was parsed in a context where 'in-expressions' are not allowed
pub const YieldContext: u32 = 1 << 13; // If node was parsed in the 'yield' context created when parsing a generator
pub const DecoratorContext: u32 = 1 << 14; // If node was parsed as part of a decorator
pub const AwaitContext: u32 = 1 << 15; // If node was parsed in the 'await' context created when parsing an async function
pub const ThisNodeHasError: u32 = 1 << 16; // If the parser encountered an error when parsing the code that created this node
pub const JavaScriptFile: u32 = 1 << 17; // If node was parsed in a JavaScript
pub const ThisNodeOrAnySubNodesHasError: u32 = 1 << 18; // If this node or any of its children had an error
pub const HasAggregatedChildData: u32 = 1 << 19; // If we've computed data from children and cached it in this node

// These flags will be set when the parser encounters a dynamic import expression or 'import.meta' to avoid
// walking the tree if the flags are not set. However, these flags are just a approximation
// (hence why it's named "PossiblyContainsDynamicImport") because once set, the flags never get cleared.
// During editing, if a dynamic import is removed, incremental parsing will *NOT* clear this flag.
// This means that the tree will always be traversed during module resolution, or when looking for external module indicators.
// However, the removal operation should not occur often and in the case of the
// removal, it is likely that users will add the import anyway.
// The advantage of this approach is its simplicity. For the case of batch compilation,
// we guarantee that users won't have to pay the price of walking the tree if a dynamic import isn't used.
/* @internal */
pub const PossiblyContainsDynamicImport: u32 = 1 << 20;
/* @internal */
pub const PossiblyContainsImportMeta: u32 = 1 << 21;

pub const JSDoc: u32 = 1 << 22; // If node was parsed inside jsdoc
                                /* @internal */
pub const Ambient: u32 = 1 << 23; // If node was inside an ambient context -- a declaration file; or inside something with the `declare` modifier.
                                  /* @internal */
pub const InWithStatement: u32 = 1 << 24; // If any ancestor of node was the `statement` of a WithStatement (not the `expression`)
pub const JsonFile: u32 = 1 << 25; // If node was parsed in a Json
                                   /* @internal */
pub const TypeCached: u32 = 1 << 26; // If a type was cached for node at any point
                                     /* @internal */
pub const Deprecated: u32 = 1 << 27; // If has '@deprecated' JSDoc tag

pub const BlockScoped: u32 = Let | Const;

pub const ReachabilityCheckFlags: u32 = HasImplicitReturn | HasExplicitReturn;
pub const ReachabilityAndEmitFlags: u32 = ReachabilityCheckFlags | HasAsyncFunctions;

// Parsing context flags
pub const ContextFlags: u32 = DisallowInContext
  | YieldContext
  | DecoratorContext
  | AwaitContext
  | JavaScriptFile
  | InWithStatement
  | Ambient;

// Exclude these flags when parsing a Type
pub const TypeExcludesFlags: u32 = YieldContext | AwaitContext;

// Represents all flags that are potentially set once and
// never cleared on SourceFiles which get re-used in between incremental parses.
// See the comment above on `PossiblyContainsDynamicImport` and `PossiblyContainsImportMeta`.
/* @internal */
pub const PermanentlySetIncrementalFlags: u32 =
  PossiblyContainsDynamicImport | PossiblyContainsImportMeta;
