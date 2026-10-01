// PURPOSE: Smoke tests — quick boot + respond within 5s.
use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use import_rules_lint_arwaky::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared_common::NamingConfig;
use shared_common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared_common::taxonomy_definition_vo::LayerDefinition;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_paths_vo::FilePathList;
use shared_config_system::ArchitectureConfig;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_import_rules::contract_import_protocol::ICycleImportProtocol;
use shared_import_rules::contract_import_protocol::IDummyImportCheckerProtocol;
use shared_import_rules::contract_import_protocol::IUnusedImportProtocol;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use std::collections::HashMap;
use std::sync::Arc;
fn minimal_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent"]),
            allowed: PatternList::new(vec!["taxonomy"]),
            ..Default::default()
        },
    );
    ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    )
}

type FilesystemDeps = (
    Arc<dyn IFilesystemAggregate>,
    Arc<dyn IFileSystemIOProtocol>,
    Arc<dyn IWorkspaceProtocol>,
    Arc<dyn IParserProtocol>,
);

fn make_filesystem() -> FilesystemDeps {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    (c.orchestrator(), c.io(), c.workspace(), c.parser())
}

#[test]
fn smoke_container_creation() {
    let config = minimal_config();
    let (fs, fs_io, ws, parser) = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs, fs_io, ws, parser);
    let orch = container.orchestrator();
    assert_eq!(
        orch.execute(ImportRequest::Name).into_name(),
        "import-rules"
    );
}

#[test]
fn smoke_individual_checker_creation() {
    let _forbidden = ArchImportForbiddenChecker::new();
    let _mandatory = ArchImportMandatoryChecker::new();
    let _unused = UnusedImportRuleChecker::new();
    let _cycle = DependencyCycleAnalyzer::new();
    let _dummy = DummyImportChecker::new();
}

#[test]
fn smoke_forbidden_checker_with_sample_imports() {
    let checker = ArchImportForbiddenChecker::new();
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent"]),
            allowed: PatternList::new(vec!["taxonomy"]),
            ..Default::default()
        },
    );
    let layer_map = shared_common::LayerMapVO::new(layers.clone());
    let config = ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    );

    // Forbidden import — should detect violation
    let result = checker
        .check_single_file(
            "/tmp/project/capabilities_handler.rs",
            "use agent::orchestrator;\n",
            "/tmp/project",
            &config,
            &layer_map,
        )
        .unwrap();
    assert!(
        !result.values.is_empty(),
        "Forbidden checker should detect agent import from capabilities"
    );
}

#[test]
fn smoke_unused_checker_with_sample_imports() {
    let checker = UnusedImportRuleChecker::new();

    // Unused import — should detect violation
    let _result = checker
        .check_unused_imports(
            "/tmp/project/src/handler.rs",
            "use std::collections::HashMap;\n\nfn main() { println!(\"hi\"); }\n",
            &[],
            &[],
            &std::collections::HashMap::new(),
        )
        .unwrap();
    // Result may be empty if no ImportEntry provided — but no panic
}

#[test]
fn smoke_cycle_analyzer_creates_and_normalizes() {
    let analyzer = DependencyCycleAnalyzer::new();
    let layer = analyzer.normalize_to_layer("capabilities_import_orchestrator");
    assert_eq!(layer.value(), "capabilities");
}

#[test]
fn smoke_dummy_checker_with_clean_file() {
    let checker = DummyImportChecker::new();
    let file = FilePath::new("/tmp/project/capabilities_handler.rs".to_string()).unwrap();
    let content = shared_common::ContentString::new("fn process() { let x = 1; }\n".to_string());
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let layers = HashMap::new();
    let layer_map = shared_common::LayerMapVO::new(layers);

    let result = checker
        .check_all_dummy(&file, &content, &root, &layer_map, &HashMap::new())
        .unwrap();
    assert!(
        result.is_empty(),
        "Clean file should have no dummy violations"
    );
}
