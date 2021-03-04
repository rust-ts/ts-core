pub struct TsConfigSourceFile {
  pub extended_source_files: Vec<String>,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
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

impl Default for ScriptTarget {
  fn default() -> ScriptTarget {
    ScriptTarget::ESNext
  }
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum LanguageVariant {
  Standard,
  JSX,
}

impl Default for LanguageVariant {
  fn default() -> LanguageVariant {
    LanguageVariant::Standard
  }
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
