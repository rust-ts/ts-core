pub struct TsConfigSourceFile {
  pub extended_source_files: Vec<String>,
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum ImportsNotUsedAsValues {
  Remove,
  Preserve,
  Error,
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum JsxEmit {
  None = 0,
  Preserve = 1,
  React = 2,
  ReactNative = 3,
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
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

impl From<&str> for ScriptKind {
  fn from(file_name: &str) -> Self {
    match &file_name[file_name.rfind('.').unwrap_or(file_name.len() - 1)..] {
      ".js" => ScriptKind::JS,
      ".jsx" => ScriptKind::JSX,
      ".ts" => ScriptKind::TS,
      ".tsx" => ScriptKind::TSX,
      ".json" => ScriptKind::JSON,
      _ => ScriptKind::Unknown,
    }
  }
}

impl ScriptKind {
  pub fn ensure_script_kind(file_name: &str, script_kind: Option<ScriptKind>) -> ScriptKind {
    script_kind.unwrap_or_else(|| match ScriptKind::from(file_name) {
      ScriptKind::Unknown => ScriptKind::TS,
      k => k,
    })
  }
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

impl From<ScriptKind> for LanguageVariant {
  fn from(kind: ScriptKind) -> Self {
    match kind {
      ScriptKind::TSX | ScriptKind::JSX | ScriptKind::JS | ScriptKind::JSON => LanguageVariant::JSX,
      _ => LanguageVariant::Standard,
    }
  }
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum ModuleResolutionKind {
  Classic = 1,
  NodeJs = 2,
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum NewLineKind {
  CarriageReturnLineFeed,
  LineFeed,
}
