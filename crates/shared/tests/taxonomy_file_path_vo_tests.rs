use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;

#[test]
fn creates_from_valid_path() {
    let fp = FilePath::new("src/main.rs".into()).unwrap();
    assert_eq!(fp.value, "src/main.rs");
}

#[test]
fn creates_from_root_path() {
    let fp = FilePath::new("/home/user/project/src/lib.rs".into()).unwrap();
    assert!(fp.value.contains("lib.rs"));
}

#[test]
fn creates_from_empty_path() {
    let fp = FilePath::new("".into());
    assert!(fp.is_err());
}

#[test]
fn display_shows_path_value() {
    let fp = FilePath::new("src/main.rs".into()).unwrap();
    assert_eq!(format!("{}", fp), "src/main.rs");
}

#[test]
fn clone_is_equal() {
    let a = FilePath::new("src/main.rs".into()).unwrap();
    let b = a.clone();
    assert_eq!(a.value, b.value);
}

#[test]
fn default_is_empty() {
    let fp = FilePath::default();
    assert_eq!(fp.value, "");
}

#[test]
fn file_path_with_special_chars() {
    let fp = FilePath::new("my project/src/utils@2.0/file.test.rs".into()).unwrap();
    assert_eq!(fp.value, "my project/src/utils@2.0/file.test.rs");
}

#[test]
fn file_path_newtype_pattern() {
    // FilePath is used as a newtype wrapper — test the field access
    let fp = FilePath::new("config.yaml".into()).unwrap();
    assert_eq!(fp.value, "config.yaml");
}
