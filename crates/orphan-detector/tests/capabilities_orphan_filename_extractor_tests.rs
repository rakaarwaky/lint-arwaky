use orphan_detector_lint_arwaky::capabilities_orphan_filename_extractor::{
    file_basename, file_stem, file_suffix,
};

#[test]
fn basename_from_path_with_dir() {
    assert_eq!(file_basename("crates/shared/src/lib.rs"), "lib.rs");
}

#[test]
fn basename_no_dir() {
    assert_eq!(file_basename("lib.rs"), "lib.rs");
}

#[test]
fn basename_root_file() {
    assert_eq!(file_basename("/root/file.py"), "file.py");
}

#[test]
fn stem_removes_rs_extension() {
    assert_eq!(file_stem("checker.rs"), "checker");
}

#[test]
fn stem_removes_py_extension() {
    assert_eq!(file_stem("checker.py"), "checker");
}

#[test]
fn stem_removes_tsx_extension() {
    assert_eq!(file_stem("component.tsx"), "component");
}

#[test]
fn stem_with_full_path() {
    assert_eq!(file_stem("crates/shared/src/lib.rs"), "lib");
}

#[test]
fn stem_keeps_mid_dots() {
    assert_eq!(file_stem("my.test.file.rs"), "my.test.file");
}

#[test]
fn stem_empty_basename() {
    assert_eq!(file_stem(""), "");
}

#[test]
fn suffix_gets_last_part() {
    assert_eq!(file_suffix("capabilities_checker.rs"), "checker");
}

#[test]
fn suffix_no_underscore_empty() {
    assert_eq!(file_suffix("checker.rs"), "checker");
}

#[test]
fn suffix_with_full_path() {
    assert_eq!(file_suffix("/path/to/surface_command.rs"), "command");
}

#[test]
fn suffix_py_file() {
    assert_eq!(file_suffix("infrastructure_adapter.py"), "adapter");
}

#[test]
fn suffix_single_underscore_prefix() {
    assert_eq!(file_suffix("_helper.rs"), "helper");
}
