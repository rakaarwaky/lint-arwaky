// Contract tests — verify dispatcher modules compile and are accessible.
#[test]
fn check_action_module_exists() {
    let _ = std::any::type_name::<
        dispatcher_lint_arwaky::orchestrator_check_pipeline::ScanOptions,
    >();
}

#[test]
fn output_component_module_exists() {
    let _ = std::any::type_name::<shared_common::ViolationItem>();
}
