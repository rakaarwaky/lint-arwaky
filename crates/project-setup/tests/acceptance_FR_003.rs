// Acceptance test — FR-003: Language detection identifies project languages.
use project_setup_lint_arwaky::root_project_setup_container::SetupContainer;
use shared::project_setup::SetupManagementAggregate;

fn make_container() -> SetupContainer {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupContainer::new(fs)
}

#[test]
fn fr003_detect_language_returns_valid_language() {
    let container = make_container();
    let agg = container.aggregate();
    let lang = agg.detect_language();
    let valid = ["rust", "python", "javascript"];
    assert!(
        valid.contains(&lang.value()),
        "FR-003: detected language '{}' should be one of {:?}",
        lang.value(),
        valid
    );
}

#[test]
fn fr003_detect_languages_returns_non_empty() {
    let container = make_container();
    let agg = container.aggregate();
    let langs = agg.detect_languages();
    assert!(
        !langs.is_empty(),
        "FR-003: detect_languages should return at least one language"
    );
}

#[test]
fn fr003_get_config_template_for_each_language() {
    let container = make_container();
    let agg = container.aggregate();
    for lang in &["rust", "python", "javascript"] {
        let template = agg.get_config_template(lang);
        assert!(
            !template.is_empty(),
            "FR-003: config template for '{}' should not be empty",
            lang
        );
    }
}
