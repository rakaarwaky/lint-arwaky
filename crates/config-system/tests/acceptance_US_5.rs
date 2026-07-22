// US-5 — Config Security
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::utility_config_io::{
    read_text_within_canonical_root, MAX_CONFIG_FILE_SIZE,
};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn us5_symlink_outside_root_is_rejected() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("project");
    fs::create_dir_all(&root).unwrap();
    let outside = tmp.path().join("secret.txt");
    fs::write(&outside, "sensitive data").unwrap();
    let link = root.join("config.yaml");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, &link).unwrap();
    #[cfg(not(unix))]
    {
        return;
    }
    let canonical_root = fs::canonicalize(&root).unwrap();
    let result = read_text_within_canonical_root(&link, &canonical_root).await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().kind(),
        std::io::ErrorKind::PermissionDenied
    );
}

#[tokio::test]
async fn us5_oversized_config_is_rejected() {
    let tmp = TempDir::new().unwrap();
    let large_content = "x".repeat((MAX_CONFIG_FILE_SIZE + 1) as usize);
    let large_file = tmp.path().join("large.yaml");
    fs::write(&large_file, &large_content).unwrap();
    let canonical_root = fs::canonicalize(tmp.path()).unwrap();
    let result = read_text_within_canonical_root(&large_file, &canonical_root).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
}

#[tokio::test]
async fn us5_valid_file_within_root_is_read() {
    let tmp = TempDir::new().unwrap();
    let config_file = tmp.path().join("config.yaml");
    fs::write(&config_file, "architecture:\n  enabled: true\n").unwrap();
    let canonical_root = fs::canonicalize(tmp.path()).unwrap();
    let result = read_text_within_canonical_root(&config_file, &canonical_root).await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("architecture"));
}

#[test]
fn us5_config_language_prevents_path_injection() {
    use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
    use std::str::FromStr;
    assert!(ConfigLanguage::from_str("rust").is_ok());
    assert!(ConfigLanguage::from_str("python").is_ok());
    assert!(ConfigLanguage::from_str("typescript").is_ok());
    assert!(ConfigLanguage::from_str("../../etc/passwd").is_err());
    assert!(ConfigLanguage::from_str("rust; rm -rf /").is_err());
}
