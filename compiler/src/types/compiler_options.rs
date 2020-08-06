use std::collections::HashMap;

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
  pub(crate) all: bool,
  pub allow_js: bool,
  pub(crate) allow_non_ts_extensions: bool,
  pub allow_synthetic_default_imports: bool,
  pub allow_umd_global_access: bool,
  pub allow_unreachable_code: bool,
  pub allow_unused_labels: bool,
  pub always_strict: bool, // Always combine with strict propert,
  pub base_url: Option<String>,
  /** An error if set - this should only go through the -b pipeline and not actually be observed */
  pub(crate) build: bool,
  pub charset: Option<String>,
  pub check_js: bool,
  pub(crate) config_file_path: Option<String>,
  /** configFile is set as non enumerable property so as to avoid checking of json source files */
  pub(crate) config_file: Option<TsConfigSourceFile>,
  pub declaration: bool,
  pub declaration_map: bool,
  pub emit_declaration_only: bool,
  pub declaration_dir: Option<String>,
  pub(crate) diagnostics: bool,
  pub(crate) extended_diagnostics: bool,
  pub disable_size_limit: bool,
  pub disable_source_of_project_reference_redirect: bool,
  pub disable_solution_searching: bool,
  pub disable_referenced_project_load: bool,
  pub downlevel_iteration: bool,
  pub emit_bom: bool,
  pub emit_decorator_metadata: bool,
  pub experimental_decorators: bool,
  pub force_consistent_casing_in_file_names: bool,
  pub(crate) generate_cpu_profile: Option<String>,
  pub(crate) help: bool,
  pub import_helpers: bool,
  pub imports_not_used_as_values: Option<ImportsNotUsedAsValues>,
  pub(crate) init: bool,
  pub inline_source_map: bool,
  pub inline_sources: bool,
  pub isolated_modules: bool,
  pub jsx: Option<JsxEmit>,
  pub keyof_strings_only: bool,
  pub lib: Vec<String>,
  pub(crate) list_emitted_files: bool,
  pub(crate) list_files: bool,
  pub(crate) list_files_only: bool,
  pub locale: Option<String>,
  pub map_root: Option<String>,
  pub max_node_module_js_depth: Option<u32>,
  pub module: Option<ModuleKind>,
  pub module_resolution: Option<ModuleResolutionKind>,
  pub new_line: Option<NewLineKind>,
  pub no_emit: bool,
  pub(crate) no_emit_for_js_files: bool,
  pub no_emit_helpers: bool,
  pub no_emit_on_error: bool,
  pub no_error_truncation: bool,
  pub no_fallthrough_cases_in_switch: bool,
  pub no_implicit_any: bool, // Always combine with strict property
  pub no_implicit_returns: bool,
  pub no_implicit_this: bool, // Always combine with strict property
  pub no_strict_generic_checks: bool,
  pub no_unused_locals: bool,
  pub no_unused_parameters: bool,
  pub no_implicit_use_strict: bool,
  pub assume_changes_only_affect_direct_dependencies: bool,
  pub no_lib: bool,
  pub no_resolve: bool,
  pub out: Option<String>,
  pub out_dir: Option<String>,
  pub out_file: Option<String>,
  pub paths: HashMap<String, Vec<String>>,
  pub(crate) plugins: Vec<String>,
  pub preserve_const_enums: bool,
  pub preserve_symlinks: bool,
  pub(crate) preserve_watch_output: bool,
  pub project: Option<String>,
  pub(crate) pretty: bool,
  pub react_namespace: Option<String>,
  pub jsx_factory: Option<String>,
  pub jsx_fragment_factory: Option<String>,
  pub composite: bool,
  pub incremental: bool,
  pub ts_build_info_file: Option<String>,
  pub remove_comments: bool,
  pub root_dir: Option<String>,
  pub root_dirs: Vec<String>,
  pub skip_lib_check: bool,
  pub skip_default_lib_check: bool,
  pub source_map: bool,
  pub source_root: Option<String>,
  pub strict: bool,
  pub strict_function_types: bool, // Always combine with strict property
  pub strict_bind_call_apply: bool, // Always combine with strict property
  pub strict_null_checks: bool,    // Always combine with strict property
  pub strict_property_initialization: bool, // Always combine with strict property
  pub strip_internal: bool,
  pub suppress_excess_property_errors: bool,
  pub suppress_implicit_any_index_errors: bool,
  pub(crate) suppress_output_path_check: bool,
  pub target: Option<ScriptTarget>, // TODO: GH#18217 frequently asserted as defined
  pub trace_resolution: bool,
  pub resolve_json_module: bool,
  pub types: Vec<String>,
  /** Paths used to compute primary types search locations */
  pub type_roots: Vec<String>,
  pub(crate) version: bool,
  pub(crate) watch: bool,
  pub es_module_interop: bool,
  pub(crate) show_config: bool,
  pub use_define_for_class_fields: bool,
}