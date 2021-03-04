use tscore_common::ScriptKind;
use tscore_common::ScriptTarget;

mod context;
pub mod diagnostic;
pub mod lexer;
pub mod parser;

pub use parser::*;

pub fn parse_source_file<T: SyntaxCursor>(
  file_name: String,
  source: String,
  lang_version: ScriptTarget,
  syntax_cursor: Option<T>,
  set_parent_nodes: bool,
  script_kind: Option<ScriptKind>,
) /*  -> SourceFile */
{
  let script_kind = ScriptKind::ensure_script_kind(&file_name, script_kind);
  if script_kind == ScriptKind::JSON {
    // const result = parseJsonText(fileName, sourceText, languageVersion, syntaxCursor, setParentNodes);
    // convertToObjectWorker(result, result.statements[0]?.expression, result.parseDiagnostics, /*returnValue*/ false, /*knownRootOptions*/ undefined, /*jsonConversionNotifier*/ undefined);
    // result.referencedFiles = emptyArray;
    // result.typeReferenceDirectives = emptyArray;
    // result.libReferenceDirectives = emptyArray;
    // result.amdDependencies = emptyArray;
    // result.hasNoDefaultLib = false;
    // result.pragmas = emptyMap as ReadonlyPragmaMap;
    // return result;
  }

  let mut parser = Parser::<T>::default();

  parser.initialize(file_name, source, lang_version, syntax_cursor, script_kind);

  parser.parse_source_file(lang_version, set_parent_nodes, script_kind);

  parser.reset();
}
