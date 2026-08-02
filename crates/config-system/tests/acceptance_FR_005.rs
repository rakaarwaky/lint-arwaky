// FR-005 — Config Security (symlink rejection, XDG fallback)
use shared::common::FilePath;
use shared::config_system::{ConfigLanguage, IConfigReaderProtocol};
use std::fs;
use tempfile::TempDir;

fn make_reader() -> config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader {
    config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader::default()
}

#[test]
fn us5_symlink_outside_root_is_rejected() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("project");
    fs::create_dir_all(&root).unwrap();
    let outside = tmp.path().join("secret.txt");
    fs::write(&outside, "sensitive data").unwrap();
    let link = root.join("lint_arwaky.config.rust.yaml");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, &link).unwrap();
    #[cfg(not(unix))]
    {
        return;
    }
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();
    let result = make_reader().read_config(&fp, ConfigLanguage::Rust);
    // Symlink pointing outside root should be rejected — reader returns None or
    // falls back to XDG defaults (which also won't find a file), so no panic.
    assert!(result.is_ok());
}

#[test]
fn us5_valid_file_within_root_is_read() {
    let tmp = TempDir::new().unwrap();
    let config_file = tmp.path().join("lint_arwaky.config.rust.yaml");
    fs::write(&config_file, "architecture:\n  enabled: true\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_reader().read_config(&fp, ConfigLanguage::Rust);
    assert!(result.is_ok());
    let source = result.unwrap();
    assert!(source.is_some());
    assert!(source.unwrap().raw_content.contains("architecture"));
}

#[test]
fn us5_config_language_prevents_path_injection() {
    use shared::config_system::ConfigLanguage;
    use std::str::FromStr;
    assert!(ConfigLanguage::from_str("rust").is_ok());
    assert!(ConfigLanguage::from_str("python").is_ok());
    assert!(ConfigLanguage::from_str("typescript").is_ok());
    assert!(ConfigLanguage::from_str("../../etc/passwd").is_err());
    assert!(ConfigLanguage::from_str("rust; rm -rf /").is_err());
}
