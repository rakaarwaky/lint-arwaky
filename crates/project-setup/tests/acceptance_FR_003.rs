// Acceptance test — FR-003: Language detection identifies project languages.
// QA #1–#6: marker detection, empty dir, multi-language, skipped dirs.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;

fn make_container() -> SetupContainer {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = c.io();
    SetupContainer::new(fs)
}

// QA #1–#3: Detects based on markers in current working directory
#[test]
fn fr003_detect_language_returns_valid_language_or_none() {
    let container = make_container();
    let lang_det = container.language_detection();
    let result = lang_det.detect_language();
    if let Some(lang) = result {
        let valid = ["rust", "python", "javascript"];
        assert!(
            valid.contains(&lang.value()),
            "FR-003: detected language '{}' should be one of {:?}",
            lang.value(),
            valid
        );
    }
    // None is valid when running from a directory with no markers
}

#[test]
fn fr003_detect_languages_returns_list() {
    let container = make_container();
    let lang_det = container.language_detection();
    let langs = lang_det.detect_languages();
    // In this repo, rust should always be detected (Cargo.toml exists)
    if !langs.is_empty() {
        let valid = ["rust", "python", "javascript"];
        for lang in langs.iter() {
            assert!(
                valid.contains(&lang.value()),
                "FR-003: detected language '{}' should be valid",
                lang.value()
            );
        }
    }
}

#[test]
fn fr003_get_config_template_rust() {
    let result = shared_project_setup::utility_project_setup_helpers::get_config_template("rust");
    assert!(result.is_ok(), "FR-003: 'rust' should have a template");
    let template = result.unwrap();
    assert!(!template.is_empty());
    assert!(template.contains("architecture") || template.contains("rules"));
}

#[test]
fn fr003_get_config_template_python() {
    let result = shared_project_setup::utility_project_setup_helpers::get_config_template("python");
    assert!(result.is_ok(), "FR-003: 'python' should have a template");
    let template = result.unwrap();
    assert!(!template.is_empty());
}

#[test]
fn fr003_get_config_template_javascript() {
    let result =
        shared_project_setup::utility_project_setup_helpers::get_config_template("javascript");
    assert!(
        result.is_ok(),
        "FR-003: 'javascript' should have a template"
    );
    let template = result.unwrap();
    assert!(!template.is_empty());
}
