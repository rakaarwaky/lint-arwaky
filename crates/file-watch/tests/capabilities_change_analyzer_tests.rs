use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;

// ---------------------------------------------------------------------------
// is_lintable — which file extensions should trigger a re-lint
// ---------------------------------------------------------------------------

#[test]
fn lintable_rust_file() {
    assert!(ChangeAnalyzer::is_lintable("src/main.rs"));
}

#[test]
fn lintable_python_file() {
    assert!(ChangeAnalyzer::is_lintable("app.py"));
}

#[test]
fn lintable_js_file() {
    assert!(ChangeAnalyzer::is_lintable("index.js"));
}

#[test]
fn lintable_ts_file() {
    assert!(ChangeAnalyzer::is_lintable("component.ts"));
}

#[test]
fn lintable_tsx_file() {
    assert!(ChangeAnalyzer::is_lintable("component.tsx"));
}

#[test]
fn lintable_jsx_file() {
    assert!(ChangeAnalyzer::is_lintable("component.jsx"));
}

#[test]
fn lintable_yaml_file() {
    assert!(ChangeAnalyzer::is_lintable("config.yaml"));
}

#[test]
fn lintable_yml_file() {
    assert!(ChangeAnalyzer::is_lintable("config.yml"));
}

#[test]
fn lintable_json_file() {
    assert!(ChangeAnalyzer::is_lintable("package.json"));
}

#[test]
fn lintable_css_file() {
    assert!(ChangeAnalyzer::is_lintable("styles.css"));
}

#[test]
fn lintable_md_file() {
    assert!(ChangeAnalyzer::is_lintable("README.md"));
}

#[test]
fn lintable_toml_file() {
    assert!(ChangeAnalyzer::is_lintable("Cargo.toml"));
}

#[test]
fn not_lintable_other_extension() {
    assert!(!ChangeAnalyzer::is_lintable("image.png"));
    assert!(!ChangeAnalyzer::is_lintable("data.bin"));
    assert!(!ChangeAnalyzer::is_lintable("font.woff2"));
    assert!(!ChangeAnalyzer::is_lintable("document.pdf"));
}

#[test]
fn not_lintable_without_extension() {
    assert!(!ChangeAnalyzer::is_lintable("Makefile"));
    assert!(!ChangeAnalyzer::is_lintable("LICENSE"));
}

#[test]
fn lintable_with_full_path() {
    assert!(ChangeAnalyzer::is_lintable("/home/user/project/src/lib.rs"));
    assert!(ChangeAnalyzer::is_lintable("crates/shared/src/lib.rs"));
}

#[test]
fn lintable_dotfiles_in_hidden_dirs() {
    assert!(ChangeAnalyzer::is_lintable(".config/settings.py"));
    assert!(ChangeAnalyzer::is_lintable(".github/workflows/deploy.rs"));
}
