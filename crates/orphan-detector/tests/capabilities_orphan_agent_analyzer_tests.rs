use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::extract_aggregate_traits;

#[test]
fn extract_aggregate_rust_impl_trait() {
    let content = "impl IMyAggregate for MyStruct { fn run(&self) {} }";
    let traits = extract_aggregate_traits(content);
    assert_eq!(traits, vec!["IMyAggregate"]);
}

#[test]
fn extract_aggregate_rust_box_dyn() {
    let content = "let agg: Box<dyn IDevAggregate> = ...";
    let traits = extract_aggregate_traits(content);
    assert_eq!(traits, vec!["IDevAggregate"]);
}

#[test]
fn extract_aggregate_rust_arc_dyn() {
    let content = "let agg: Arc<dyn IImportAggregate> = ...";
    let traits = extract_aggregate_traits(content);
    assert_eq!(traits, vec!["IImportAggregate"]);
}

#[test]
fn extract_aggregate_python_class() {
    let content = "class MyProcessor(IDevAggregate):";
    let traits = extract_aggregate_traits(content);
    assert_eq!(traits, vec!["IDevAggregate"]);
}

#[test]
fn extract_aggregate_js_class_implements() {
    let content = "class MyProcessor implements ILintAggregate";
    let traits = extract_aggregate_traits(content);
    assert_eq!(traits, vec!["ILintAggregate"]);
}

#[test]
fn extract_aggregate_no_aggregate_trait() {
    let content = "impl IHelper for Foo { fn help(&self) {} }";
    let traits = extract_aggregate_traits(content);
    assert!(
        traits.is_empty(),
        "non-aggregate traits should not be extracted"
    );
}

#[test]
fn extract_aggregate_empty_content() {
    let traits = extract_aggregate_traits("");
    assert!(traits.is_empty());
}

#[test]
fn extract_aggregate_multiple_traits_deduped() {
    let content = "\
impl IFirstAggregate for Foo {}
impl IFirstAggregate for Bar {}
";
    let traits = extract_aggregate_traits(content);
    assert_eq!(traits, vec!["IFirstAggregate"]);
}

#[test]
fn extract_aggregate_multiple_different_traits() {
    let content = "\
impl IFirstAggregate for Foo {}
impl ISecondAggregate for Bar {}
";
    let traits = extract_aggregate_traits(content);
    assert_eq!(traits.len(), 2);
    assert!(traits.contains(&"IFirstAggregate".to_string()));
    assert!(traits.contains(&"ISecondAggregate".to_string()));
}
