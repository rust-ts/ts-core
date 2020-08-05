use std::collections::HashMap;

pub struct TsConfigSourceFile {
  extendedSourceFiles: Vec<String>,
}

#[repr(u8)]
pub enum ImportsNotUsedAsValues {
  Remove,
  Preserve,
  Error,
}

#[repr(u8)]
pub enum JsxEmit {
  None = 0,
  Preserve = 1,
  React = 2,
  ReactNative = 3,
}

#[repr(u8)]
pub enum ScriptKind {
  Unknown = 0,
  JS = 1,
  JSX = 2,
  TS = 3,
  TSX = 4,
  External = 5,
  JSON = 6,
  /**
   * Used on extensions that doesn't define the ScriptKind but the content defines it.
   * Deferred extensions are going to be included in all project contexts.
   */
  Deferred = 7,
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum ScriptTarget {
  ES3 = 0,
  ES5 = 1,
  ES2015 = 2,
  ES2016 = 3,
  ES2017 = 4,
  ES2018 = 5,
  ES2019 = 6,
  ES2020 = 7,
  ESNext = 99,
  JSON = 100,
}

#[repr(u8)]
pub enum LanguageVariant {
  Standard,
  JSX,
}

#[repr(u8)]
pub enum ModuleKind {
  None = 0,
  CommonJS = 1,
  AMD = 2,
  UMD = 3,
  System = 4,

  // NOTE: ES module kinds should be contiguous to more easily check whether a module kind is *any* ES module kind.
  //       Non-ES module kinds should not come between ES2015 (the earliest ES module kind) and ESNext (the last ES
  //       module kind).
  ES2015 = 5,
  ES2020 = 6,
  ESNext = 99,
}

#[repr(u8)]
pub enum ModuleResolutionKind {
  Classic = 1,
  NodeJs = 2,
}

#[repr(u8)]
pub enum NewLineKind {
  CarriageReturnLineFeed,
  LineFeed,
}

pub struct CompilerOptions {
  pub(crate) all: Option<bool>,
  pub allowJs: Option<bool>,
  pub(crate) allowNonTsExtensions: Option<bool>,
  pub allowSyntheticDefaultImports: Option<bool>,
  pub allowUmdGlobalAccess: Option<bool>,
  pub allowUnreachableCode: Option<bool>,
  pub allowUnusedLabels: Option<bool>,
  pub alwaysStrict: Option<bool>, // Always combine with strict propert,
  pub baseUrl: Option<String>,
  /** An error if set - this should only go through the -b pipeline and not actually be observed */
  pub(crate) build: Option<bool>,
  pub charset: Option<String>,
  pub checkJs: Option<bool>,
  pub(crate) configFilePath: Option<String>,
  /** configFile is set as non enumerable property so as to avoid checking of json source files */
  pub(crate) configFile: Option<TsConfigSourceFile>,
  pub declaration: Option<bool>,
  pub declarationMap: Option<bool>,
  pub emitDeclarationOnly: Option<bool>,
  pub declarationDir: Option<String>,
  pub(crate) diagnostics: Option<bool>,
  pub(crate) extendedDiagnostics: Option<bool>,
  pub disableSizeLimit: Option<bool>,
  pub disableSourceOfProjectReferenceRedirect: Option<bool>,
  pub disableSolutionSearching: Option<bool>,
  pub disableReferencedProjectLoad: Option<bool>,
  pub downlevelIteration: Option<bool>,
  pub emitBOM: Option<bool>,
  pub emitDecoratorMetadata: Option<bool>,
  pub experimentalDecorators: Option<bool>,
  pub forceConsistentCasingInFileNames: Option<bool>,
  pub(crate) generateCpuProfile: Option<String>,
  pub(crate) help: Option<bool>,
  pub importHelpers: Option<bool>,
  pub importsNotUsedAsValues: Option<ImportsNotUsedAsValues>,
  pub(crate) init: Option<bool>,
  pub inlineSourceMap: Option<bool>,
  pub inlineSources: Option<bool>,
  pub isolatedModules: Option<bool>,
  pub jsx: Option<JsxEmit>,
  pub keyofStringsOnly: Option<bool>,
  pub lib: Vec<String>,
  pub(crate) listEmittedFiles: Option<bool>,
  pub(crate) listFiles: Option<bool>,
  pub(crate) listFilesOnly: Option<bool>,
  pub locale: Option<String>,
  pub mapRoot: Option<String>,
  pub maxNodeModuleJsDepth: Option<u32>,
  pub module: Option<ModuleKind>,
  pub moduleResolution: Option<ModuleResolutionKind>,
  pub newLine: Option<NewLineKind>,
  pub noEmit: Option<bool>,
  pub(crate) noEmitForJsFiles: Option<bool>,
  pub noEmitHelpers: Option<bool>,
  pub noEmitOnError: Option<bool>,
  pub noErrorTruncation: Option<bool>,
  pub noFallthroughCasesInSwitch: Option<bool>,
  pub noImplicitAny: Option<bool>, // Always combine with strict property
  pub noImplicitReturns: Option<bool>,
  pub noImplicitThis: Option<bool>, // Always combine with strict property
  pub noStrictGenericChecks: Option<bool>,
  pub noUnusedLocals: Option<bool>,
  pub noUnusedParameters: Option<bool>,
  pub noImplicitUseStrict: Option<bool>,
  pub assumeChangesOnlyAffectDirectDependencies: Option<bool>,
  pub noLib: Option<bool>,
  pub noResolve: Option<bool>,
  pub out: Option<String>,
  pub outDir: Option<String>,
  pub outFile: Option<String>,
  pub paths: HashMap<String, Vec<String>>,
  pub(crate) plugins: Vec<String>,
  pub preserveConstEnums: Option<bool>,
  pub preserveSymlinks: Option<bool>,
  pub(crate) preserveWatchOutput: Option<bool>,
  pub project: Option<String>,
  pub(crate) pretty: Option<bool>,
  pub reactNamespace: Option<String>,
  pub jsxFactory: Option<String>,
  pub jsxFragmentFactory: Option<String>,
  pub composite: Option<bool>,
  pub incremental: Option<bool>,
  pub tsBuildInfoFile: Option<String>,
  pub removeComments: Option<bool>,
  pub rootDir: Option<String>,
  pub rootDirs: Vec<String>,
  pub skipLibCheck: Option<bool>,
  pub skipDefaultLibCheck: Option<bool>,
  pub sourceMap: Option<bool>,
  pub sourceRoot: Option<String>,
  pub strict: Option<bool>,
  pub strictFunctionTypes: Option<bool>, // Always combine with strict property
  pub strictBindCallApply: Option<bool>, // Always combine with strict property
  pub strictNullChecks: Option<bool>,    // Always combine with strict property
  pub strictPropertyInitialization: Option<bool>, // Always combine with strict property
  pub stripInternal: Option<bool>,
  pub suppressExcessPropertyErrors: Option<bool>,
  pub suppressImplicitAnyIndexErrors: Option<bool>,
  pub(crate) suppressOutputPathCheck: Option<bool>,
  pub target: Option<ScriptTarget>, // TODO: GH#18217 frequently asserted as defined
  pub traceResolution: Option<bool>,
  pub resolveJsonModule: Option<bool>,
  pub types: Vec<String>,
  /** Paths used to compute primary types search locations */
  pub typeRoots: Vec<String>,
  pub(crate) version: Option<bool>,
  pub(crate) watch: Option<bool>,
  pub esModuleInterop: Option<bool>,
  pub(crate) showConfig: Option<bool>,
  pub useDefineForClassFields: Option<bool>,
}
