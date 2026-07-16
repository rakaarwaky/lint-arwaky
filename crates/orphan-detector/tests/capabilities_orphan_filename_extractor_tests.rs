use orphan_detector_lint_arwaky::capabilities_orphan_filename_extractor::OrphanFilenameExtractor;
use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;

fn make_extractor() -> OrphanFilenameExtractor {
    OrphanFilenameExtractor::new()
}

fn fp(s: &str) -> shared::common::taxonomy_path_vo::FilePath {
    shared::common::taxonomy_path_vo::FilePath::new(s.to_string()).unwrap()
}

#[test]
fn basename_from_path_with_dir() {
    let ex = make_extractor();
    assert_eq!(ex.file_basename(&fp("crates/shared/src/lib.rs")).value, "lib.rs");
}

#[test]
fn basename_no_dir() {
    let ex = make_extractor();
    assert_eq!(ex.file_basename(&fp("lib.rs")).value, "lib.rs");
}

#[test]
fn basename_root_file() {
    let ex = make_extractor();
    assert_eq!(ex.file_basename(&fp("/root/file.py")).value, "file.py");
}

#[test]
fn stem_removes_rs_extension() {
    let ex = make_extractor();
    assert_eq!(ex.file_stem(&fp("checker.rs")).value, "checker");
}

#[test]
fn stem_removes_py_extension() {
    let ex = make_extractor();
    assert_eq!(ex.file_stem(&fp("checker.py")).value, "checker");
}

#[test]
fn stem_removes_tsx_extension() {
    let ex = make_extractor();
    assert_eq!(ex.file_stem(&fp("component.tsx")).value, "component");
}

#[test]
fn stem_with_full_path() {
    let ex = make_extractor();
    assert_eq!(ex.file_stem(&fp("crates/shared/src/lib.rs")).value, "lib");
}

#[test]
fn stem_keeps_mid_dots() {
    let ex = make_extractor();
    assert_eq!(ex.file_stem(&fp("my.test.file.rs")).value, "my.test.file");
}

#[test]
fn stem_empty_basename() {
    let ex = make_extractor();
    assert_eq!(ex.file_stem(&fp("")).value, "");
}

#[test]
fn suffix_gets_last_part() {
    let ex = make_extractor();
    assert_eq!(ex.file_suffix(&fp("capabilities_checker.rs")).value, "checker");
}

#[test]
fn suffix_no_underscore_empty() {
    let ex = make_extractor();
    assert_eq!(ex.file_suffix(&fp("checker.rs")).value, "checker");
}

#[test]
fn suffix_with_full_path() {
    let ex = make_extractor();
    assert_eq!(ex.file_suffix(&fp("/path/to/surface_command.rs")).value, "command");
}

#[test]
fn suffix_py_file() {
    let ex = make_extractor();
    assert_eq!(ex.file_suffix(&fp("infrastructure_adapter.py")).value, "adapter");
}

#[test]
fn suffix_single_underscore_prefix() {
    let ex = make_extractor();
    assert_eq!(ex.file_suffix(&fp("_helper.rs")).value, "helper");
}
