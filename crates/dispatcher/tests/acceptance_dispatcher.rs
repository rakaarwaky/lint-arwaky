// Acceptance tests — dispatcher surface actions produce valid output.
use shared::common::taxonomy_path_vo::FilePath;

#[test]
fn acceptance_check_action_on_current_project() {
    let path = FilePath::new(".").unwrap();
    let result = dispatcher_lint_arwaky::surface_check_action::CheckAction::execute(&path);
    // Result should be a valid lint result list
    for violation in &result.values {
        assert!(
            !violation.aes_code.is_empty(),
            "Each violation should have an AES code"
        );
    }
}

#[test]
fn acceptance_output_component_formats_results() {
    let results = shared::common::taxonomy_lint_result_vo::LintResultList::new(Vec::new());
    let output =
        dispatcher_lint_arwaky::surface_output_component::OutputComponent::format(&results);
    assert!(!output.is_empty(), "Output should not be empty");
}
