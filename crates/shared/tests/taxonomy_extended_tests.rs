// PURPOSE: Extended tests for high-gap files not covered by taxonomy_vo_comprehensive_tests
// Covers: taxonomy_duration_vo, taxonomy_command_catalog_vo, taxonomy_protocol_vo,
// taxonomy_violation_code_analysis_vo, taxonomy_import_source_vo,
// infrastructure_file_collector_provider

use shared_lint_arwaky::cli_commands::taxonomy_command_catalog_vo::*;
use shared_lint_arwaky::cli_commands::taxonomy_protocol_vo::*;
use shared_lint_arwaky::code_analysis::taxonomy_import_source_vo::*;
use shared_lint_arwaky::code_analysis::taxonomy_violation_code_analysis_vo::*;
use shared_lint_arwaky::common::taxonomy_common_vo::*;
use shared_lint_arwaky::common::taxonomy_duration_vo::*;
use shared_lint_arwaky::common::taxonomy_message_vo::LintMessage;

// ============================================================================
// taxonomy_duration_vo: Timeout
// ============================================================================

#[test]
fn timeout_new_clamps_to_minimum() {
    let t = Timeout::new(0.0005); // below 0.001 minimum
    assert!(t.value() >= 0.001, "should clamp to minimum 0.001");
    assert_eq!(t.to_string(), "0.001s");
    let t2 = Timeout::new(5.0);
    assert_eq!(t2.value(), 5.0);
    assert_eq!(t2.to_string(), "5s");
    let t3 = Timeout::from(1.5f64);
    assert_eq!(t3.value, 1.5);
}

#[test]
fn timeout_edge_cases() {
    let t = Timeout::new(-1.0);
    assert_eq!(t.value(), 0.001, "negative values clamped to min");
    let t2 = Timeout::new(0.0);
    assert_eq!(t2.value(), 0.001, "zero clamped to min");
}

// ============================================================================
// taxonomy_command_catalog_vo: CommandCatalogVO
// ============================================================================

#[test]
fn command_catalog_contains_expected_commands() {
    let catalog = command_catalog();
    assert!(catalog.contains_key(&"check".into()), "should have check");
    assert!(catalog.contains_key(&"scan".into()), "should have scan");
    assert!(catalog.contains_key(&"fix".into()), "should have fix");
    assert!(catalog.contains_key(&"ci".into()), "should have ci");
    assert!(
        catalog.contains_key(&"version".into()),
        "should have version"
    );
    assert!(
        catalog.contains_key(&"adapters".into()),
        "should have adapters"
    );
    assert!(
        catalog.contains_key(&"security".into()),
        "should have security"
    );
}

#[test]
fn command_catalog_metadata_non_empty() {
    let catalog = command_catalog();
    for (name, meta) in &catalog {
        assert!(
            !meta.description.value.is_empty(),
            "{} missing description",
            name
        );
        assert!(!meta.example.value.is_empty(), "{} missing example", name);
    }
}

#[test]
fn command_catalog_count() {
    let catalog = command_catalog();
    assert!(
        catalog.len() >= 12,
        "should have at least 12 commands, got {}",
        catalog.len()
    );
}

// ============================================================================
// taxonomy_protocol_vo: TransportProtocol, TransportEndpoint
// ============================================================================

#[test]
fn transport_protocol_display() {
    assert_eq!(format!("{}", TransportProtocol::HTTP), "HTTP");
    assert_eq!(format!("{}", TransportProtocol::UnixSocket), "UnixSocket");
    assert_eq!(format!("{}", TransportProtocol::STDAggregate), "Stdio");
}

#[test]
fn transport_protocol_needs_desktop_commander() {
    assert!(TransportProtocol::HTTP.needs_desktop_commander());
    assert!(TransportProtocol::UnixSocket.needs_desktop_commander());
    assert!(!TransportProtocol::STDAggregate.needs_desktop_commander());
}

#[test]
fn transport_endpoint_new_and_display() {
    let ep = TransportEndpoint::new(TransportProtocol::HTTP, "http://localhost:8080".to_string());
    assert_eq!(ep.display_name(), "HTTP(http://localhost:8080)");
    assert_eq!(format!("{}", ep), "HTTP:http://localhost:8080");

    let ep2 = TransportEndpoint::new(TransportProtocol::STDAggregate, String::new());
    assert_eq!(ep2.display_name(), "Stdio(direct)");
}

#[test]
fn transport_endpoint_from_url() {
    let ep = TransportEndpoint::from_url("http://localhost:3000");
    assert_eq!(ep.protocol, TransportProtocol::HTTP);
    assert_eq!(ep.address, "http://localhost:3000");

    let ep2 = TransportEndpoint::from_url("stdio");
    assert_eq!(ep2.protocol, TransportProtocol::STDAggregate);

    let ep3 = TransportEndpoint::from_url("/var/run/socket.sock");
    assert_eq!(ep3.protocol, TransportProtocol::UnixSocket);

    let ep4 = TransportEndpoint::from_url("./local.sock");
    assert_eq!(ep4.protocol, TransportProtocol::UnixSocket);

    let ep5 = TransportEndpoint::from_url("unknown");
    assert_eq!(ep5.protocol, TransportProtocol::STDAggregate);
}

#[test]
fn transport_endpoint_default() {
    let ep = TransportEndpoint::default();
    assert_eq!(ep.protocol, TransportProtocol::STDAggregate);
    assert!(ep.address.is_empty());
}

// ============================================================================
// taxonomy_violation_code_analysis_vo: Language, AesCodeAnalysisViolation
// ============================================================================

#[test]
fn language_from_adapter_name() {
    assert_eq!(Language::from_adapter_name("clippy"), Language::Rust);
    assert_eq!(Language::from_adapter_name("rust"), Language::Rust);
    assert_eq!(Language::from_adapter_name("eslint"), Language::JavaScript);
    assert_eq!(Language::from_adapter_name("ruff"), Language::Python);
    assert_eq!(Language::from_adapter_name("unknown"), Language::Rust);
}

#[test]
fn language_struct_keyword() {
    assert_eq!(Language::Rust.struct_keyword(), "struct");
    assert_eq!(Language::JavaScript.struct_keyword(), "class/interface");
    assert_eq!(Language::Python.struct_keyword(), "class/Protocol");
}

#[test]
fn language_type_keyword() {
    assert_eq!(Language::Rust.type_kw(), "type");
    assert_eq!(Language::JavaScript.type_kw(), "interface/type");
    assert_eq!(Language::Python.type_kw(), "Protocol/type");
}

#[test]
fn language_interface_keyword() {
    assert_eq!(Language::Rust.interface_kw(), "trait");
    assert_eq!(Language::JavaScript.interface_kw(), "interface");
    assert_eq!(Language::Python.interface_kw(), "Protocol");
}

#[test]
fn language_inherits_keyword() {
    assert_eq!(Language::Rust.inherits_kw(), "implements");
    assert_eq!(Language::JavaScript.inherits_kw(), "implements/extends");
    assert_eq!(Language::Python.inherits_kw(), "implements/inherits");
}

#[test]
fn aes_file_too_large_display() {
    let v = AesCodeAnalysisViolation::FileTooLarge { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("AES301"));
    assert!(s.contains("FILE_TOO_LARGE"));

    let v2 = AesCodeAnalysisViolation::FileTooLarge {
        reason: Some(LintMessage::new("custom reason")),
    };
    let s2 = format!("{}", v2);
    assert!(s2.contains("custom reason"));
}

#[test]
fn aes_file_too_short_display() {
    let v = AesCodeAnalysisViolation::FileTooShort { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("AES302"));
}

#[test]
fn aes_bypass_comment_display() {
    let v = AesCodeAnalysisViolation::BypassComment { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("AES304"));
    assert!(s.contains("BYPASS_COMMENT"));
}

#[test]
fn aes_unwrap_expect_display() {
    let v = AesCodeAnalysisViolation::UnwrapExpect { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("UNWRAP_EXPECT"));
}

#[test]
fn aes_panic_display() {
    let v = AesCodeAnalysisViolation::Panic { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("PANIC"));
}

#[test]
fn aes_todo_display() {
    let v = AesCodeAnalysisViolation::Todo { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("TODO"));
}

#[test]
fn aes_unimplemented_display() {
    let v = AesCodeAnalysisViolation::Unimplemented { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("UNIMPLEMENTED"));
}

#[test]
fn aes_mandatory_class_definition_display() {
    let v = AesCodeAnalysisViolation::MandatoryClassDefinition { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("AES303"));
    assert!(s.contains("MANDATORY_DEFINITION"));
}

#[test]
fn aes_dead_inheritance_display() {
    let v = AesCodeAnalysisViolation::DeadInheritance { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("AES305"));
    assert!(s.contains("DEAD_INHERITANCE"));
}

#[test]
fn aes_code_duplication_display() {
    let v = AesCodeAnalysisViolation::CodeDuplication { reason: None };
    let s = format!("{}", v);
    assert!(s.contains("CODE_DUPLICATION"));
}

#[test]
fn aes_violation_into_string() {
    let v = AesCodeAnalysisViolation::FileTooLarge { reason: None };
    let s: String = v.into();
    assert!(s.contains("AES301"));
}

// ============================================================================
// taxonomy_import_source_vo: ImportInfo, PrimitiveViolation, ImportInfoList, PrimitiveViolationList
// ============================================================================

#[test]
fn import_info_new() {
    let info = ImportInfo::new(LineNumber::new(10), "std::fs".to_string());
    assert_eq!(info.line.value, 10);
    assert_eq!(info.module, "std::fs");
    assert!(info.name.is_none());
}

#[test]
fn primitive_violation_new() {
    let pv = PrimitiveViolation::new(
        LineNumber::new(5),
        ColumnNumber::new(3),
        "String".to_string(),
    );
    assert_eq!(pv.line.value, 5);
    assert_eq!(pv.column.value, 3);
    assert_eq!(pv.type_name, "String");
}

#[test]
fn import_info_list_methods() {
    let mut list = ImportInfoList::new();
    assert!(list.is_empty());
    let info = ImportInfo::new(LineNumber::new(1), "mod".to_string());
    list.push(info);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
}

#[test]
fn primitive_violation_list_methods() {
    let mut list = PrimitiveViolationList::new();
    assert!(list.is_empty());
    let pv = PrimitiveViolation::new(LineNumber::new(1), ColumnNumber::new(1), "int".to_string());
    list.push(pv);
    assert_eq!(list.len(), 1);
}

// ============================================================================
// infrastructure_file_collector_provider: FileCollectorProvider, collect_all_source_files
// ============================================================================

#[test]
fn collect_all_source_files_nonexistent_dir() {
    let files =
        code_analysis::collect_all_source_files(std::path::Path::new("/nonexistent_path_xyz_999"));
    assert!(files.is_empty(), "nonexistent dir should return empty");
}

#[test]
fn file_collector_provider_scan_nonexistent() {
    use shared_lint_arwaky::common::contract_scanner_provider_port::IScannerProviderPort;
    use shared_lint_arwaky::common::taxonomy_path_vo::DirectoryPath;
    let provider = code_analysis::FileCollectorProvider::new();
    let path = DirectoryPath::new("/nonexistent_xyz").unwrap();
    let result = provider.scan_directory(&path).unwrap();
    assert!(result.is_empty());
}

#[test]
fn file_collector_provider_get_ignored_files() {
    use shared_lint_arwaky::common::contract_scanner_provider_port::IScannerProviderPort;
    let provider = code_analysis::FileCollectorProvider::new();
    let ignored = provider.get_ignored_files();
    assert!(ignored.values.is_empty());
}

#[test]
fn file_collector_provider_scan_temp_dir_with_source_files() {
    use shared_lint_arwaky::common::contract_scanner_provider_port::IScannerProviderPort;
    use shared_lint_arwaky::common::taxonomy_path_vo::DirectoryPath;

    let dir = std::env::temp_dir().join("lint_arwaky_ext_test");
    let _ = std::fs::create_dir_all(&dir);
    std::fs::write(dir.join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.join("lib.py"), "def foo(): pass").unwrap();
    std::fs::write(dir.join("readme.txt"), "not a source file").unwrap();

    let provider = code_analysis::FileCollectorProvider::new();
    let path = DirectoryPath::new(dir.to_string_lossy().to_string()).unwrap();
    let result = provider.scan_directory(&path).unwrap();
    assert_eq!(result.len(), 2, "should find 2 source files (.rs, .py)");

    // cleanup
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn file_collector_provider_scans_recursively() {
    use shared_lint_arwaky::common::contract_scanner_provider_port::IScannerProviderPort;
    use shared_lint_arwaky::common::taxonomy_path_vo::DirectoryPath;

    let dir = std::env::temp_dir().join("lint_arwaky_rec_test");
    let _ = std::fs::create_dir_all(&dir.join("nested/deep"));
    std::fs::write(dir.join("root.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.join("nested/child.ts"), "let x = 1").unwrap();
    std::fs::write(dir.join("nested/deep/deep.js"), "console.log('hi')").unwrap();

    let provider = code_analysis::FileCollectorProvider::new();
    let path = DirectoryPath::new(dir.to_string_lossy().to_string()).unwrap();
    let result = provider.scan_directory(&path).unwrap();
    assert_eq!(result.len(), 3, "should find 3 source files across subdirs");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn file_collector_provider_skips_tests_dir() {
    use shared_lint_arwaky::common::contract_scanner_provider_port::IScannerProviderPort;
    use shared_lint_arwaky::common::taxonomy_path_vo::DirectoryPath;

    let dir = std::env::temp_dir().join("lint_arwaky_tests_skip");
    let _ = std::fs::create_dir_all(&dir.join("tests"));
    std::fs::write(dir.join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.join("tests/test_mod.rs"), "fn test() {}").unwrap();

    let provider = code_analysis::FileCollectorProvider::new();
    let path = DirectoryPath::new(dir.to_string_lossy().to_string()).unwrap();
    let result = provider.scan_directory(&path).unwrap();
    assert_eq!(result.len(), 1, "should skip tests/ dir, only find main.rs");

    let _ = std::fs::remove_dir_all(&dir);
}

// ============================================================================
// collect_all_source_files utility
// ============================================================================

#[test]
fn collect_all_source_files_empty_dir() {
    let dir = std::env::temp_dir().join("lint_arwaky_empty_coll");
    let _ = std::fs::create_dir_all(&dir);
    let files = code_analysis::collect_all_source_files(&dir);
    assert!(files.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn collect_all_source_files_with_rs_files() {
    let dir = std::env::temp_dir().join("lint_arwaky_rs_coll");
    let _ = std::fs::create_dir_all(&dir);
    std::fs::write(dir.join("a.rs"), "fn a() {}").unwrap();
    std::fs::write(dir.join("b.py"), "def b(): pass").unwrap();
    let files = code_analysis::collect_all_source_files(&dir);
    assert_eq!(files.len(), 2, "should find both .rs and .py files");
    let _ = std::fs::remove_dir_all(&dir);
}
