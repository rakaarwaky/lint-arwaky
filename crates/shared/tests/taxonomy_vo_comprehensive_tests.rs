// PURPOSE: Comprehensive tests for shared crate VOs with highest coverage gaps
// Covers taxonomy_common_vo, taxonomy_path_vo, taxonomy_paths_vo, taxonomy_lint_vo,
// taxonomy_parser_error, taxonomy_adapter_error, taxonomy_common_error,
// taxonomy_symbol_renamer_utility, taxonomy_file_collector_helper, taxonomy_naming_list_vo

use shared_lint_arwaky::auto_fix::taxonomy_symbol_renamer_utility::SymbolRenamer;
use shared_lint_arwaky::cli_commands::taxonomy_severity_vo::Severity;
use shared_lint_arwaky::common::taxonomy_adapter_error::{
    AdapterError, ScanError, ValidationError,
};
use shared_lint_arwaky::common::taxonomy_adapter_name_vo::AdapterName;
use shared_lint_arwaky::common::taxonomy_common_error::{ExitCode, FieldName};
use shared_lint_arwaky::common::taxonomy_common_vo::{
    BooleanVO, ColumnNumber, Count, DataFlowList, ErrorMessage, JobIdList, LineContentList,
    LineNumber, PatternList, ResponseDataList, Score, Timestamp,
};
use shared_lint_arwaky::common::taxonomy_job_id_vo::JobId;
use shared_lint_arwaky::common::taxonomy_layer_vo::LineContentVO;
use shared_lint_arwaky::common::taxonomy_lint_vo::{
    CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint,
};
use shared_lint_arwaky::common::taxonomy_name_vo::SymbolName;
use shared_lint_arwaky::common::taxonomy_naming_list_vo::{
    primitive_type_list, CallChainList, ImportNameList, PrimitiveTypeList, SymbolNameList,
};
use shared_lint_arwaky::common::taxonomy_parser_error::{SourceParserError, SyntaxErrorVO};
use shared_lint_arwaky::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared_lint_arwaky::common::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
use shared_lint_arwaky::common::taxonomy_response_data_vo::ResponseData;
use shared_lint_arwaky::common::taxonomy_source_vo::ContentString;
use shared_lint_arwaky::common::taxonomy_suggestion_vo::DescriptionVO;

// ============================================================================
// taxonomy_common_vo: BooleanVO, ColumnNumber, Count, DataFlowList, JobIdList,
//                     LineContentList, LineNumber, PatternList, ResponseDataList,
//                     Score, Timestamp, ErrorMessage
// ============================================================================

#[test]
fn boolean_vo_new_value_display() {
    let b = BooleanVO::new(true);
    assert!(b.value());
    assert_eq!(b.to_string(), "true");
    let b2 = BooleanVO::from(false);
    assert!(!b2.value);
    let b3: BooleanVO = Default::default();
    assert!(!b3.value);
}

#[test]
fn column_number_new_value_display() {
    let c = ColumnNumber::new(42);
    assert_eq!(c.value(), 42);
    assert_eq!(c.to_string(), "42");
    let c2 = ColumnNumber::from(10i64);
    assert_eq!(c2.value, 10);
}

#[test]
fn count_new_value_display() {
    let c = Count::new(5);
    assert_eq!(c.value(), 5);
    assert_eq!(c.to_string(), "5");
    let c2 = Count::from(3i64);
    assert_eq!(c2.value, 3);
}

#[test]
fn data_flow_list_methods() {
    let mut list = DataFlowList::new(vec![]);
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    let msg = ErrorMessage::new("test flow");
    list.push(msg.clone());
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
    assert_eq!(list.values()[0].value(), "test flow");
}

#[test]
fn job_id_list_methods() {
    let mut list = JobIdList::new(vec![]);
    assert!(list.is_empty());
    let id = JobId::new("job1".to_string());
    list.push(id);
    assert_eq!(list.len(), 1);
    assert!(!list.values().is_empty());
}

#[test]
fn line_content_list_methods() {
    let mut list = LineContentList::new(vec![]);
    assert!(list.is_empty());
    let vo = LineContentVO::new("line1".to_string());
    list.push(vo);
    assert_eq!(list.len(), 1);
}

#[test]
fn line_number_new_value_display() {
    let l = LineNumber::new(10);
    assert_eq!(l.value(), 10);
    assert_eq!(l.to_string(), "10");
    let l2 = LineNumber::from(20i64);
    assert_eq!(l2.value, 20);
}

#[test]
fn pattern_list_various_constructors() {
    let p = PatternList::new("*.rs");
    assert_eq!(p.len(), 1);
    let p2 = PatternList::new(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(p2.len(), 2);
    let p3 = PatternList::new(vec!["x", "y", "z"]);
    assert_eq!(p3.len(), 3);
    let src = vec!["p".to_string()];
    let p4 = PatternList::new(&src);
    assert_eq!(p4.len(), 1);
    let mut p5: PatternList = Default::default();
    assert!(p5.is_empty());
    p5.push("new_pat".to_string());
    assert_eq!(p5.len(), 1);
}

#[test]
fn response_data_list_methods() {
    let mut list = ResponseDataList::new(vec![]);
    assert!(list.is_empty());
    let rd = ResponseData::new();
    list.push(rd);
    assert_eq!(list.len(), 1);
}

#[test]
fn score_new_value_perfect_passing_deduct_display() {
    let s = Score::new(95.5);
    assert_eq!(s.value(), 95.5);
    assert!(!s.is_perfect());
    let s2 = Score::new(100.0);
    assert!(s2.is_perfect());
    let threshold = Score::new(80.0);
    assert!(s.is_passing(&threshold));
    let low = Score::new(50.0);
    assert!(!low.is_passing(&threshold));
    let deducted = s.deduct(&Severity::CRITICAL);
    assert!(deducted.value < s.value());
    assert_eq!(s.to_string(), "95.5");
    let from_f64: Score = (85.0f64).into();
    assert_eq!(from_f64.value, 85.0);
}

#[test]
fn timestamp_new_now_value_display() {
    let t = Timestamp::new("2024-01-01T00:00:00Z");
    assert_eq!(t.value(), "2024-01-01T00:00:00Z");
    assert_eq!(t.to_string(), "2024-01-01T00:00:00Z");
    let now = Timestamp::now();
    assert!(now.value().contains('T'));
    let from_str: Timestamp = "fixed".into();
    assert_eq!(from_str.value, "fixed");
    let from_string: Timestamp = String::from("also_fixed").into();
    assert_eq!(from_string.value, "also_fixed");
}

#[test]
fn error_message_new_value_display() {
    let e = ErrorMessage::new("something went wrong");
    assert_eq!(e.value(), "something went wrong");
    assert_eq!(e.to_string(), "something went wrong");
    let e2: ErrorMessage = "from_str".into();
    assert_eq!(e2.value, "from_str");
    let e3: ErrorMessage = String::from("from_string").into();
    assert_eq!(e3.value, "from_string");
}

// ============================================================================
// taxonomy_path_vo: FilePath, DirectoryPath
// ============================================================================

#[test]
fn file_path_new_validation() {
    let fp = FilePath::new("src/main.rs").unwrap();
    assert_eq!(fp.value(), "src/main.rs");
    assert!(FilePath::new("").is_err());
    assert!(FilePath::new("   ").is_err());
    let fp2 = FilePath::new("src\\main.rs").unwrap();
    assert_eq!(fp2.value, "src/main.rs");
    let fp3 = FilePath::new("src/").unwrap();
    assert_eq!(fp3.value, "src");
    let fp4 = FilePath::new("///").unwrap();
    assert_eq!(fp4.value, "/");
}

#[test]
fn file_path_extension_basename_barrel_entry() {
    let fp = FilePath::new("src/main.rs").unwrap();
    assert_eq!(fp.extension(), "rs");
    assert!(fp.has_extension("rs"));
    assert!(fp.has_extension("RS"));
    assert!(!fp.has_extension("py"));
    assert_eq!(fp.basename(), "main.rs");
    let py = FilePath::new("mymod/__init__.py").unwrap();
    assert!(py.is_barrel_file());
    assert!(py.is_entry_point());
    let mod_rs = FilePath::new("module/mod.rs").unwrap();
    assert!(mod_rs.is_barrel_file());
    let index = FilePath::new("src/index.ts").unwrap();
    assert!(index.is_barrel_file());
    let lib = FilePath::new("src/lib.rs").unwrap();
    assert!(lib.is_entry_point());
    assert!(!lib.is_barrel_file());
    let hidden = FilePath::new(".bashrc").unwrap();
    assert_eq!(hidden.extension(), "");
    let no_ext = FilePath::new("Makefile").unwrap();
    assert_eq!(no_ext.extension(), "");
}

#[test]
fn file_path_deref_display_hash() {
    use std::collections::HashSet;
    let fp = FilePath::new("foo/bar.rs").unwrap();
    assert_eq!(fp.len(), 10);
    assert_eq!(format!("{}", fp), "foo/bar.rs");
    let mut set = HashSet::new();
    set.insert(fp.clone());
    assert!(set.contains(&fp));
}

#[test]
fn directory_path_new_validation() {
    let dp = DirectoryPath::new("/home/user/project").unwrap();
    assert_eq!(dp.value(), "/home/user/project");
    assert!(DirectoryPath::new("").is_err());
    assert!(DirectoryPath::new("   ").is_err());
    let dp2 = DirectoryPath::new("/home/user/").unwrap();
    assert_eq!(dp2.value, "/home/user");
    let dp3 = DirectoryPath::new("C:\\Users\\me").unwrap();
    assert_eq!(dp3.value, "C:/Users/me");
}

#[test]
fn directory_path_deref_display() {
    let dp = DirectoryPath::new("/tmp").unwrap();
    assert_eq!(dp.len(), 4);
    assert_eq!(format!("{}", dp), "/tmp");
}

// ============================================================================
// taxonomy_paths_vo: FilePathList, RenamedFile, RenamedFileList
// ============================================================================

#[test]
fn renamed_file_new() {
    let old = FilePath::new("old.rs").unwrap();
    let new = FilePath::new("new.rs").unwrap();
    let rf = RenamedFile::new(old, new);
    assert_eq!(rf.old_path.value, "old.rs");
    assert_eq!(rf.new_path.value, "new.rs");
}

#[test]
fn renamed_file_list_methods() {
    let mut list = RenamedFileList::new(vec![]);
    assert!(list.is_empty());
    let old = FilePath::new("a.rs").unwrap();
    let new = FilePath::new("b.rs").unwrap();
    list.push(RenamedFile::new(old, new));
    assert_eq!(list.len(), 1);
}

#[test]
fn file_path_list_new_and_methods() {
    let f1 = FilePath::new("a.rs").unwrap();
    let f2 = FilePath::new("b.rs").unwrap();
    let mut list = FilePathList::new(vec![f1, f2]);
    assert_eq!(list.len(), 2);
    assert!(!list.is_empty());
    let f3 = FilePath::new("c.rs").unwrap();
    list.push(f3);
    assert_eq!(list.len(), 3);
}

// ============================================================================
// taxonomy_lint_vo: ScopeRef, Location, LocationList, ViolationConstraint,
//                    CommandArgs, ScopeBounds
// ============================================================================

#[test]
fn scope_ref_new_has_range_display() {
    let sr = ScopeRef::new("my_func");
    assert_eq!(sr.name.value, "my_func");
    assert!(!sr.has_range());
    let sr2 = ScopeRef {
        name: DescriptionVO::new("f"),
        kind: DescriptionVO::new("function"),
        file: Some(FilePath::new("file.rs").unwrap()),
        start_line: Some(LineNumber::new(10)),
        end_line: Some(LineNumber::new(20)),
    };
    assert!(sr2.has_range());
    let display = format!("{}", sr2);
    assert!(display.contains("function"));
    assert!(display.contains("f"));
    assert!(display.contains("file.rs"));
}

#[test]
fn location_new_display() {
    let loc = Location::new();
    assert_eq!(format!("{}", loc), "unknown");
    let loc2 = Location {
        file: Some(FilePath::new("f.rs").unwrap()),
        line: Some(LineNumber::new(5)),
        column: Some(ColumnNumber::new(10)),
        description: DescriptionVO::new("desc here"),
    };
    let s = format!("{}", loc2);
    assert!(s.contains("f.rs"));
    assert!(s.contains("5:10"));
    assert!(s.contains("desc here"));
}

#[test]
fn location_list_methods() {
    let mut ll = LocationList::new();
    assert!(ll.is_empty());
    ll.push(Location::new());
    assert_eq!(ll.len(), 1);
}

#[test]
fn violation_constraint_new_display() {
    let vc = ViolationConstraint::new("AES301");
    assert_eq!(vc.rule.value, "AES301");
    assert_eq!(format!("{}", vc), "AES301");
}

#[test]
fn command_args_display() {
    let ca = CommandArgs::new();
    assert_eq!(format!("{}", ca), "");
    let ca2 = CommandArgs {
        args: vec![ContentString::new("arg1"), ContentString::new("arg2")],
    };
    assert_eq!(format!("{}", ca2), "arg1 arg2");
}

#[test]
fn scope_bounds_default() {
    let sb = ScopeBounds {
        start: None,
        end: None,
    };
    assert!(sb.start.is_none());
}

// ============================================================================
// taxonomy_parser_error: SourceParserError, SyntaxErrorVO
// ============================================================================

#[test]
fn source_parser_error_new_display() {
    let path = FilePath::new("test.rs").unwrap();
    let err = SourceParserError::new(path, ErrorMessage::new("parse failed"));
    let s = format!("{}", err);
    assert!(s.contains("test.rs"));
    assert!(s.contains("parse failed"));
}

#[test]
fn syntax_error_vo_new_display() {
    let path = FilePath::new("syntax.rs").unwrap();
    let err = SyntaxErrorVO::new(path, ErrorMessage::new("unexpected token"));
    let s = format!("{}", err);
    assert!(s.contains("Syntax Error"));
    assert!(s.contains("syntax.rs"));
    assert!(s.contains("unexpected token"));
}

// ============================================================================
// taxonomy_adapter_error: AdapterError, ScanError, ValidationError
// ============================================================================

#[test]
fn adapter_error_new_display() {
    let name = AdapterName::raw("ruff");
    let err = AdapterError::new(name, ErrorMessage::new("tool not found"));
    let s = format!("{}", err);
    assert!(s.contains("ruff"));
    assert!(s.contains("tool not found"));
}

#[test]
fn scan_error_new_display() {
    let path = FilePath::new("src/lib.rs").unwrap();
    let err = ScanError::new(path, ErrorMessage::new("scan timeout"));
    let s = format!("{}", err);
    assert!(s.contains("src/lib.rs"));
    assert!(s.contains("scan timeout"));
}

#[test]
fn validation_error_new_display() {
    let err = ValidationError::new(
        FieldName::new("threshold"),
        ErrorMessage::new("must be >= 0"),
    );
    let s = format!("{}", err);
    assert!(s.contains("threshold"));
    assert!(s.contains("must be >= 0"));
}

// ============================================================================
// taxonomy_common_error: ExitCode
// ============================================================================

#[test]
fn exit_code_new_value_display_from() {
    let ec = ExitCode::new(1);
    assert_eq!(ec.value(), 1);
    assert_eq!(ec.to_string(), "1");
    let ec2 = ExitCode::from(255i64);
    assert_eq!(ec2.value(), 255);
}

// ============================================================================
// taxonomy_symbol_renamer_utility: SymbolRenamer
// ============================================================================

#[test]
fn symbol_renamer_nonexistent_file() {
    let count = SymbolRenamer::rename_in_file("/nonexistent/path/file.rs", "old_sym", "new_sym");
    assert_eq!(count, 0);
}

#[test]
fn symbol_renamer_symbol_not_found_in_nonexistent() {
    assert!(!SymbolRenamer::symbol_exists(
        "/nonexistent/path/file.rs",
        "sym"
    ));
}

// ============================================================================
// taxonomy_naming_list_vo: SymbolNameList, ImportNameList, PrimitiveTypeList, CallChainList
// ============================================================================

#[test]
fn symbol_name_list_default_and_methods() {
    let mut list: SymbolNameList = Default::default();
    assert!(list.is_empty());
    let name = SymbolName::new("my_var".to_string());
    list.push(name);
    assert_eq!(list.len(), 1);
}

#[test]
fn import_name_list_default_and_methods() {
    let list: ImportNameList = Default::default();
    assert!(list.is_empty());
}

#[test]
fn primitive_type_list_new_contains() {
    let list: PrimitiveTypeList = Default::default();
    assert!(list.is_empty());
    let plist = primitive_type_list();
    assert!(!plist.is_empty());
    assert!(plist.contains("str"));
    assert!(plist.contains("int"));
    assert!(!plist.contains("foobar"));
}

#[test]
fn call_chain_list_default_and_methods() {
    let list: CallChainList = Default::default();
    assert!(list.is_empty());
}
