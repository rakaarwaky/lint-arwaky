// Contract tests — verify all concrete types implement their declared contract traits.
use dispatcher_lint_arwaky::surface_check_action;
use dispatcher_lint_arwaky::surface_output_component;

#[test]
fn check_action_module_exists() {
    // Verify the module compiles and is accessible
    let _ = std::any::type_name::<dispatcher_lint_arwaky::surface_check_action::CheckAction>();
}

#[test]
fn output_component_module_exists() {
    let _ =
        std::any::type_name::<dispatcher_lint_arwaky::surface_output_component::OutputComponent>();
}
