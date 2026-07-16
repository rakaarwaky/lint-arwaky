// Comprehensive tests for high-gap shared crate files to drive coverage toward 50%.
// Targets: taxonomy_unused_helper, violation_import_vo, violation_orphan_vo,
// file_entry_vo, violation_role_vo, orphan_utility, doctor_vo, state_vo,
// layer_names_vo, setup_contract_vo, naming_violation_vo, action_flags_vo,
// orphan_contract_vo, stats_vo

use shared_lint_arwaky::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared_lint_arwaky::common::taxonomy_common_error::ErrorMessage;
use shared_lint_arwaky::common::taxonomy_common_vo::Count;
use shared_lint_arwaky::common::taxonomy_common_vo::Score;

use shared_lint_arwaky::common::taxonomy_layer_vo::LayerNameVO;
use shared_lint_arwaky::common::taxonomy_message_vo::LintMessage;
use shared_lint_arwaky::common::taxonomy_name_vo::SymbolName;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_paths_vo::FilePathList;
use shared_lint_arwaky::common::taxonomy_suggestion_vo::DescriptionVO;
use shared_lint_arwaky::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use shared_lint_arwaky::orphan_detector::taxonomy_orphan_contract_vo::OrphanEntryPatternListVO;
use shared_lint_arwaky::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;
use shared_lint_arwaky::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared_lint_arwaky::project_setup::taxonomy_doctor_vo::DoctorResultVO;
use shared_lint_arwaky::project_setup::taxonomy_setup_contract_vo::McpBinaryNameVO;
use shared_lint_arwaky::project_setup::taxonomy_setup_contract_vo::ProjectLanguageVO;
use shared_lint_arwaky::project_setup::taxonomy_setup_contract_vo::SetupError;
use shared_lint_arwaky::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::all_core_layers;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::core_layer_names;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_agent;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_capabilities;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_contract;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_global;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_infrastructure;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_root;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_surfaces;
use shared_lint_arwaky::role_rules::taxonomy_layer_names_vo::layer_taxonomy;
use shared_lint_arwaky::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared_lint_arwaky::tui::taxonomy_action_flags_vo::ActionFlags;
use shared_lint_arwaky::tui::taxonomy_file_entry_vo::AesLayer;
use shared_lint_arwaky::tui::taxonomy_file_entry_vo::FileEntry;
use shared_lint_arwaky::tui::taxonomy_state_vo::AppState;
use shared_lint_arwaky::tui::taxonomy_state_vo::PanelFocus;
use shared_lint_arwaky::tui::taxonomy_state_vo::PreviewMode;
use std::collections::HashMap;

// ============================================================================
// taxonomy_orphan_contract_vo.rs (18 uncovered lines)
// ============================================================================

#[test]
fn test_orphan_file_list_vo_basic() {
    let vo = OrphanFileListVO::new(vec!["a.rs".to_string(), "b.rs".to_string()]);
    assert_eq!(vo.len(), 2);
    assert!(!vo.is_empty());
    let items: Vec<&String> = vo.iter().collect();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_orphan_file_list_vo_empty() {
    let vo = OrphanFileListVO::default();
    assert_eq!(vo.len(), 0);
    assert!(vo.is_empty());
}

#[test]
fn test_orphan_entry_pattern_list_vo_basic() {
    let vo = OrphanEntryPatternListVO::new(vec!["main.rs".to_string()]);
    assert_eq!(vo.len(), 1);
    assert!(!vo.is_empty());
}

#[test]
fn test_orphan_entry_pattern_list_vo_empty() {
    let vo = OrphanEntryPatternListVO::default();
    assert!(vo.is_empty());
}

// ============================================================================
// taxonomy_violation_orphan_vo.rs (53 uncovered)
// ============================================================================

#[test]
fn test_aes_orphan_violation_taxonomy_orphan_display() {
    let v = AesOrphanViolation::TaxonomyOrphan {
        stem: "my_vo".to_string(),
        category: "vo",
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES501"));
    assert!(msg.contains("TAXONOMY_ORPHAN"));
    assert!(msg.contains("my_vo"));
}

#[test]
fn test_aes_orphan_violation_taxonomy_orphan_with_reason() {
    let v = AesOrphanViolation::TaxonomyOrphan {
        stem: "my_vo".to_string(),
        category: "utility",
        reason: Some(LintMessage::new("custom reason")),
    };
    let msg = v.to_string();
    assert!(msg.contains("custom reason"));
}

#[test]
fn test_aes_orphan_violation_contract_orphan_port() {
    let v = AesOrphanViolation::ContractOrphan {
        suffix: "port".to_string(),
        trait_name: "MyPort".to_string(),
        target_layer: "infrastructure",
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES502"));
    assert!(msg.contains("CONTRACT_ORPHAN"));
    assert!(msg.contains("MyPort"));
    assert!(msg.contains("infrastructure"));
}

#[test]
fn test_aes_orphan_violation_contract_orphan_protocol() {
    let v = AesOrphanViolation::ContractOrphan {
        suffix: "protocol".to_string(),
        trait_name: "MyProtocol".to_string(),
        target_layer: "capabilities",
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES502"));
    assert!(msg.contains("CONTRACT_ORPHAN"));
}

#[test]
fn test_aes_orphan_violation_contract_orphan_aggregate() {
    let v = AesOrphanViolation::ContractOrphan {
        suffix: "aggregate".to_string(),
        trait_name: "MyAgg".to_string(),
        target_layer: "surface",
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("surface_"));
}

#[test]
fn test_aes_orphan_violation_capabilities_orphan() {
    let v = AesOrphanViolation::CapabilitiesOrphan {
        stem: "my_checker".to_string(),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES503"));
    assert!(msg.contains("CAPABILITIES_ORPHAN"));
}

#[test]
fn test_aes_orphan_violation_infrastructure_orphan() {
    let v = AesOrphanViolation::InfrastructureOrphan {
        stem: "my_adapter".to_string(),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES504"));
    assert!(msg.contains("INFRASTRUCTURE_ORPHAN"));
}

#[test]
fn test_aes_orphan_violation_agent_orphan() {
    let v = AesOrphanViolation::AgentOrphan {
        agg_name: "MyOrchestrator".to_string(),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES505"));
    assert!(msg.contains("AGENT_ORPHAN"));
}

#[test]
fn test_aes_orphan_violation_surface_orphan_smart() {
    let v = AesOrphanViolation::SurfaceOrphan {
        category: "smart",
        stem: "my_command".to_string(),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES506"));
    assert!(msg.contains("SURFACE_ORPHAN"));
    assert!(msg.contains("entry point"));
}

#[test]
fn test_aes_orphan_violation_surface_orphan_with_reason() {
    let v = AesOrphanViolation::SurfaceOrphan {
        category: "passive",
        stem: "my_view".to_string(),
        reason: Some(LintMessage::new("not imported anywhere")),
    };
    let msg = v.to_string();
    assert!(msg.contains("not imported anywhere"));
}

#[test]
fn test_aes_orphan_violation_from_string() {
    let v = AesOrphanViolation::TaxonomyOrphan {
        stem: "test".to_string(),
        category: "vo",
        reason: None,
    };
    let s: String = v.into();
    assert!(s.contains("AES501"));
}

// ============================================================================
// taxonomy_violation_import_vo.rs (58 uncovered)
// ============================================================================

#[test]
fn test_aes_import_violation_forbidden_import_display() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::ForbiddenImport {
        source_layer: LayerNameVO::new("surfaces(command)"),
        forbidden_layer: LayerNameVO::new("infrastructure"),
        allowed: vec![LayerNameVO::new("capabilities")],
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES201"));
    assert!(msg.contains("FORBIDDEN_IMPORT"));
    assert!(msg.contains("surfaces(command)"));
    assert!(msg.contains("infrastructure"));
}

#[test]
fn test_aes_import_violation_forbidden_import_no_allowed() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::ForbiddenImport {
        source_layer: LayerNameVO::new("taxonomy(vo)"),
        forbidden_layer: LayerNameVO::new("agent"),
        allowed: vec![],
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("fully isolated"));
    assert!(msg.contains("none"));
}

#[test]
fn test_aes_import_violation_forbidden_import_with_reason() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::ForbiddenImport {
        source_layer: LayerNameVO::new("capabilities"),
        forbidden_layer: LayerNameVO::new("infrastructure"),
        allowed: vec![],
        reason: Some(LintMessage::new("custom reason text")),
    };
    let msg = v.to_string();
    assert!(msg.contains("custom reason text"));
}

#[test]
fn test_aes_import_violation_missing_import_display() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::MissingImport {
        source_layer: LayerNameVO::new("capabilities"),
        required: SymbolName::new("IUserProtocol"),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES202"));
    assert!(msg.contains("MANDATORY_IMPORT"));
    assert!(msg.contains("IUserProtocol"));
}

#[test]
fn test_aes_import_violation_missing_import_with_reason() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::MissingImport {
        source_layer: LayerNameVO::new("infrastructure"),
        required: SymbolName::new("IUserPort"),
        reason: Some(LintMessage::new("must implement port")),
    };
    let msg = v.to_string();
    assert!(msg.contains("must implement port"));
}

#[test]
fn test_aes_import_violation_unused_import_display() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::FixUnusedImport {
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES203"));
    assert!(msg.contains("UNUSED_IMPORT"));
}

#[test]
fn test_aes_import_violation_unused_import_with_reason() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::FixUnusedImport {
        reason: Some(LintMessage::new("import not used")),
    };
    let msg = v.to_string();
    assert!(msg.contains("import not used"));
}

#[test]
fn test_aes_import_violation_intent_violation() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::ImportIntentViolation {
        source_layer: LayerNameVO::new("surfaces(command)"),
        import_type: SymbolName::new("DBConnection"),
        intent: SymbolName::new("data_access"),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES204"));
    assert!(msg.contains("IMPORT_INTENT"));
}

#[test]
fn test_aes_import_violation_circular_import() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::CircularImport {
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES205"));
    assert!(msg.contains("CIRCULAR_IMPORT"));
}

#[test]
fn test_aes_import_violation_circular_import_with_reason() {
    let v = shared_lint_arwaky::import_rules::taxonomy_violation_import_vo::AesImportViolation::CircularImport {
        reason: Some(LintMessage::new("a -> b -> a cycle")),
    };
    let msg = v.to_string();
    assert!(msg.contains("a -> b -> a cycle"));
}

// ============================================================================
// taxonomy_violation_role_vo.rs (31 uncovered)
// ============================================================================

#[test]
fn test_aes_role_violation_constant_purity() {
    let v = AesRoleViolation::ConstantPurity { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("AES401"));
    assert!(msg.contains("TAXONOMY_ROLE"));
    assert!(msg.contains("Constant file contains non-constant"));
}

#[test]
fn test_aes_role_violation_constant_purity_with_reason() {
    let v = AesRoleViolation::ConstantPurity {
        reason: Some(LintMessage::new("mutable variable found")),
    };
    let msg = v.to_string();
    assert!(msg.contains("mutable variable found"));
}

#[test]
fn test_aes_role_violation_primitive_usage() {
    let v = AesRoleViolation::PrimitiveUsage {
        primitive: SymbolName::new("String"),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES401"));
    assert!(msg.contains("String"));
    assert!(msg.contains("primitive"));
}

#[test]
fn test_aes_role_violation_contract_primitive() {
    let v = AesRoleViolation::ContractPrimitive { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("AES402"));
    assert!(msg.contains("CONTRACT_PRIMITIVE"));
}

#[test]
fn test_aes_role_violation_capability_routing() {
    let v = AesRoleViolation::CapabilityRouting {
        struct_name: SymbolName::new("UserChecker"),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES403"));
    assert!(msg.contains("UserChecker"));
}

#[test]
fn test_aes_role_violation_capability_no_protocol() {
    let v = AesRoleViolation::CapabilityNoProtocol { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("AES403"));
    assert!(msg.contains("no protocol"));
}

#[test]
fn test_aes_role_violation_single_bottleneck() {
    let v = AesRoleViolation::SingleBottleneck { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("single bottleneck"));
}

#[test]
fn test_aes_role_violation_infrastructure_no_port() {
    let v = AesRoleViolation::InfrastructureNoPort { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("AES404"));
    assert!(msg.contains("INFRASTRUCTURE_ROLE"));
}

#[test]
fn test_aes_role_violation_stateless_execution() {
    let v = AesRoleViolation::StatelessExecution { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("AES405"));
    assert!(msg.contains("AGENT_ROLE"));
    assert!(msg.contains("stateless"));
}

#[test]
fn test_aes_role_violation_high_level_policy() {
    let v = AesRoleViolation::HighLevelPolicy { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("infrastructure adapters"));
}

#[test]
fn test_aes_role_violation_coordinates_multiple() {
    let v = AesRoleViolation::CoordinatesMultiple { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("too few subsystems"));
}

#[test]
fn test_aes_role_violation_no_domain_logic() {
    let v = AesRoleViolation::NoDomainLogic { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("domain logic"));
}

#[test]
fn test_aes_role_violation_lazy_eager_init() {
    let v = AesRoleViolation::LazyEagerInit { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("initialization"));
}

#[test]
fn test_aes_role_violation_must_implement_contract() {
    let v = AesRoleViolation::MustImplementContract { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("ServiceContainerAggregate"));
}

#[test]
fn test_aes_role_violation_any_type() {
    let v = AesRoleViolation::AnyType { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("any") || msg.contains("Any"));
    assert!(msg.contains("Forbidden"));
}

#[test]
fn test_aes_role_violation_agent_file_size_limit() {
    let v = AesRoleViolation::AgentFileSizeLimit { max_lines: 500 };
    let msg = v.to_string();
    assert!(msg.contains("500"));
    assert!(msg.contains("exceeds"));
}

#[test]
fn test_aes_role_violation_passive_violation() {
    let v = AesRoleViolation::PassiveViolation { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("AES406"));
    assert!(msg.contains("SURFACE_ROLE"));
}

#[test]
fn test_aes_role_violation_surface_role_violation() {
    let v = AesRoleViolation::SurfaceRoleViolation { reason: None };
    let msg = v.to_string();
    assert!(msg.contains("boundary"));
}

#[test]
fn test_labeled_role_violation_display_rust() {
    let v = AesRoleViolation::ConstantPurity { reason: None };
    let labeled = v.with_language(Language::Rust);
    let msg = labeled.to_string();
    assert!(msg.contains("AES401"));
}

#[test]
fn test_labeled_role_violation_display_python() {
    let v = AesRoleViolation::ContractPrimitive { reason: None };
    let labeled = v.with_language(Language::Python);
    let msg = labeled.to_string();
    assert!(msg.contains("AES402"));
    assert!(msg.contains("Protocol") || msg.contains("type"));
}

// ============================================================================
// taxonomy_naming_violation_vo.rs (18 uncovered)
// ============================================================================

#[test]
fn test_naming_violation_naming_convention() {
    let v = NamingViolation::NamingConvention {
        min_words: 2,
        separator: "_".to_string(),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES101"));
    assert!(msg.contains("NAMING_CONVENTION"));
    assert!(msg.contains("2"));
}

#[test]
fn test_naming_violation_naming_convention_with_reason() {
    let v = NamingViolation::NamingConvention {
        min_words: 3,
        separator: "-".to_string(),
        reason: Some(LintMessage::new("custom reason")),
    };
    let msg = v.to_string();
    assert!(msg.contains("custom reason"));
}

#[test]
fn test_naming_violation_unknown_prefix() {
    let v = NamingViolation::UnknownPrefix {
        prefix: "xxx".to_string(),
        allowed: vec!["taxonomy".to_string(), "contract".to_string()],
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES102"));
    assert!(msg.contains("UNKNOWN_PREFIX"));
    assert!(msg.contains("xxx"));
}

#[test]
fn test_naming_violation_unknown_prefix_with_reason() {
    let v = NamingViolation::UnknownPrefix {
        prefix: "bad".to_string(),
        allowed: vec!["good".to_string()],
        reason: Some(LintMessage::new("custom prefix reason")),
    };
    let msg = v.to_string();
    assert!(msg.contains("custom prefix reason"));
}

#[test]
fn test_naming_violation_suffix_forbidden() {
    let v = NamingViolation::SuffixForbidden {
        layer_name: "surfaces".to_string(),
        forbidden_suffix: "adapter".to_string(),
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES102"));
    assert!(msg.contains("SUFFIX_FORBIDDEN"));
    assert!(msg.contains("adapter"));
}

#[test]
fn test_naming_violation_suffix_mismatch() {
    let v = NamingViolation::SuffixMismatch {
        layer_name: "infrastructure".to_string(),
        used_suffix: "utility".to_string(),
        allowed: vec!["adapter".to_string(), "provider".to_string()],
        reason: None,
    };
    let msg = v.to_string();
    assert!(msg.contains("AES102"));
    assert!(msg.contains("SUFFIX_MISMATCH"));
}

// ============================================================================
// taxonomy_file_entry_vo.rs (49 uncovered)
// ============================================================================

#[test]
fn test_aes_layer_from_filename_taxonomy() {
    assert_eq!(
        AesLayer::from_filename("taxonomy_user_vo.rs"),
        AesLayer::Taxonomy
    );
}

#[test]
fn test_aes_layer_from_filename_contract() {
    assert_eq!(
        AesLayer::from_filename("contract_user_port.rs"),
        AesLayer::Contract
    );
}

#[test]
fn test_aes_layer_from_filename_capabilities() {
    assert_eq!(
        AesLayer::from_filename("capabilities_user_checker.rs"),
        AesLayer::Capabilities
    );
}

#[test]
fn test_aes_layer_from_filename_infrastructure() {
    assert_eq!(
        AesLayer::from_filename("infrastructure_db_adapter.rs"),
        AesLayer::Infrastructure
    );
}

#[test]
fn test_aes_layer_from_filename_agent() {
    assert_eq!(
        AesLayer::from_filename("agent_user_orchestrator.rs"),
        AesLayer::Agent
    );
}

#[test]
fn test_aes_layer_from_filename_surfaces() {
    assert_eq!(
        AesLayer::from_filename("surface_user_command.rs"),
        AesLayer::Surfaces
    );
}

#[test]
fn test_aes_layer_from_filename_root() {
    assert_eq!(
        AesLayer::from_filename("root_main_entry.rs"),
        AesLayer::Root
    );
}

#[test]
fn test_aes_layer_from_filename_none() {
    assert_eq!(AesLayer::from_filename("main.rs"), AesLayer::None);
}

#[test]
fn test_aes_layer_badge_labels() {
    assert_eq!(AesLayer::Taxonomy.badge_label(), "[tax]");
    assert_eq!(AesLayer::Contract.badge_label(), "[con]");
    assert_eq!(AesLayer::Capabilities.badge_label(), "[cap]");
    assert_eq!(AesLayer::Infrastructure.badge_label(), "[inf]");
    assert_eq!(AesLayer::Agent.badge_label(), "[agt]");
    assert_eq!(AesLayer::Surfaces.badge_label(), "[sur]");
    assert_eq!(AesLayer::Root.badge_label(), "[root]");
    assert_eq!(AesLayer::None.badge_label(), "[---]");
}

#[test]
fn test_aes_layer_color_index() {
    assert_eq!(AesLayer::Taxonomy.color_index(), 14);
    assert_eq!(AesLayer::Contract.color_index(), 12);
    assert_eq!(AesLayer::None.color_index(), 8);
}

#[test]
fn test_file_entry_from_path() {
    let entry = FileEntry {
        name: "taxonomy_user_vo.rs".into(),
        full_path: "taxonomy_user_vo.rs".into(),
        is_dir: false,
        layer: AesLayer::Taxonomy,
        violation_count: 0,
        extension: "rs".into(),
        size_bytes: 0,
    };
    assert_eq!(entry.name, "taxonomy_user_vo.rs");
    assert_eq!(entry.layer, AesLayer::Taxonomy);
    assert_eq!(entry.extension, "rs");
}

#[test]
fn test_file_entry_from_path_directory() {
    let entry = FileEntry {
        name: "src".into(),
        full_path: "src".into(),
        is_dir: true,
        layer: AesLayer::None,
        violation_count: 0,
        extension: String::new(),
        size_bytes: 0,
    };
    assert!(entry.is_dir);
    assert_eq!(entry.layer, AesLayer::None);
}

#[test]
fn test_file_entry_display_name() {
    let entry = FileEntry {
        name: "test.rs".into(),
        full_path: "test.rs".into(),
        is_dir: false,
        layer: AesLayer::None,
        violation_count: 0,
        extension: "rs".into(),
        size_bytes: 0,
    };
    assert_eq!(entry.display_name(), "test.rs");
}

#[test]
fn test_file_entry_display_name_dir() {
    let entry = FileEntry {
        name: "mydir".to_string(),
        full_path: "mydir".to_string(),
        is_dir: true,
        layer: AesLayer::None,
        violation_count: 0,
        extension: String::new(),
        size_bytes: 0,
    };
    assert_eq!(entry.display_name(), "mydir/");
}

// ============================================================================
// taxonomy_state_vo.rs (20 uncovered)
// ============================================================================

#[test]
fn test_app_state_new() {
    let state = AppState::new("/root".to_string());
    assert_eq!(state.project_root, "/root");
    assert_eq!(state.current_dir, "/root");
    assert!(state.entries.is_empty());
    assert_eq!(state.panel_focus, PanelFocus::FileList);
    assert_eq!(state.preview_mode, PreviewMode::FileContent);
    assert_eq!(state.status_message, "Ready");
}

#[test]
fn test_app_state_select_next() {
    let mut state = AppState::new("/root".to_string());
    // Empty entries — select_next should be a no-op
    state.select_next();
    assert_eq!(state.selected_index, 0);

    // With entries in search mode but no query — uses non-search path
    let entry = FileEntry {
        name: "a.rs".into(),
        full_path: "a.rs".into(),
        is_dir: false,
        layer: AesLayer::None,
        violation_count: 0,
        extension: "rs".into(),
        size_bytes: 0,
    };
    state.entries.push(entry);
    state.search_mode = true;
    state.search_query = String::new(); // empty query → non-search path
    state.select_next();
    // With 1 entry, selected_index stays at 0 (can't advance past last)
    assert_eq!(state.selected_index, 0);
}

#[test]
fn test_app_state_select_prev() {
    let mut state = AppState::new("/root".to_string());
    state.selected_index = 1;
    state.select_prev();
    assert_eq!(state.selected_index, 0);
    state.select_prev(); // already at 0
    assert_eq!(state.selected_index, 0);
}

#[test]
fn test_app_state_select_first() {
    let mut state = AppState::new("/root".to_string());
    state.selected_index = 5;
    state.select_first();
    assert_eq!(state.selected_index, 0);
}

#[test]
fn test_app_state_select_last() {
    let mut state = AppState::new("/root".to_string());
    let entry = FileEntry {
        name: "a.rs".into(),
        full_path: "a.rs".into(),
        is_dir: false,
        layer: AesLayer::None,
        violation_count: 0,
        extension: "rs".into(),
        size_bytes: 0,
    };
    state.entries.push(entry);
    state.select_last();
    assert_eq!(state.selected_index, 0);
}

#[test]
fn test_app_state_selected_entry_none() {
    let state = AppState::new("/root".to_string());
    assert!(state.selected_entry().is_none());
}

#[test]
fn test_app_state_selected_path_fallback() {
    let state = AppState::new("/root".to_string());
    assert_eq!(state.selected_path(), "/root");
}

#[test]
fn test_app_state_cycle_focus() {
    let mut state = AppState::new("/root".to_string());
    assert_eq!(state.panel_focus, PanelFocus::FileList);
    state.cycle_focus_forward();
    assert_eq!(state.panel_focus, PanelFocus::Preview);
    state.cycle_focus_forward();
    assert_eq!(state.panel_focus, PanelFocus::Tree);
    state.cycle_focus_forward();
    assert_eq!(state.panel_focus, PanelFocus::FileList);
    state.cycle_focus_backward();
    assert_eq!(state.panel_focus, PanelFocus::Tree);
}

#[test]
fn test_app_state_set_status() {
    let mut state = AppState::new("/root".to_string());
    state.set_status("Scanning...");
    assert_eq!(state.status_message, "Scanning...");
}

#[test]
fn test_app_state_adjust_scroll() {
    let mut state = AppState::new("/root".to_string());
    state.selected_index = 10;
    state.adjust_scroll(5);
    assert_eq!(state.scroll_offset, 6); // 10 - 5 + 1
    state.scroll_offset = 10;
    state.selected_index = 2;
    state.adjust_scroll(5);
    assert_eq!(state.scroll_offset, 2); // clamped to selected_index
}

#[test]
fn test_app_state_compute_filtered_indices_search() {
    let mut state = AppState::new("/root".to_string());
    state.search_mode = true;
    state.search_query = "test".to_string();
    let entry1 = FileEntry {
        name: "test_file.rs".into(),
        full_path: "test_file.rs".into(),
        is_dir: false,
        layer: AesLayer::None,
        violation_count: 0,
        extension: "rs".into(),
        size_bytes: 0,
    };
    let entry2 = FileEntry {
        name: "other.rs".into(),
        full_path: "other.rs".into(),
        is_dir: false,
        layer: AesLayer::None,
        violation_count: 0,
        extension: "rs".into(),
        size_bytes: 0,
    };
    state.entries.push(entry1);
    state.entries.push(entry2);
    state.compute_filtered_indices();
    assert_eq!(state.filtered_indices.len(), 1);
    assert_eq!(state.filtered_indices[0], 0);
}

#[test]
fn test_app_state_compute_filtered_indices_no_search() {
    let mut state = AppState::new("/root".to_string());
    state.search_mode = false;
    state.compute_filtered_indices();
    assert!(state.filtered_indices.is_empty());
}

// ============================================================================
// taxonomy_action_flags_vo.rs (18 uncovered)
// ============================================================================

#[test]
fn test_action_flags_default() {
    let flags = ActionFlags::default();
    assert!(!flags.git_diff);
    assert!(!flags.dry_run);
    assert_eq!(flags.threshold, 80);
    assert!(!flags.global_config);
    assert!(!flags.use_sudo);
    assert_eq!(flags.mcp_client, "claude");
}

#[test]
fn test_action_flags_toggle_git_diff() {
    let mut flags = ActionFlags::default();
    flags.toggle_git_diff();
    assert!(flags.git_diff);
    flags.toggle_git_diff();
    assert!(!flags.git_diff);
}

#[test]
fn test_action_flags_toggle_dry_run() {
    let mut flags = ActionFlags::default();
    flags.toggle_dry_run();
    assert!(flags.dry_run);
}

#[test]
fn test_action_flags_toggle_global() {
    let mut flags = ActionFlags::default();
    flags.toggle_global();
    assert!(flags.global_config);
}

#[test]
fn test_action_flags_toggle_sudo() {
    let mut flags = ActionFlags::default();
    flags.toggle_sudo();
    assert!(flags.use_sudo);
}

#[test]
fn test_action_flags_set_threshold() {
    let mut flags = ActionFlags::default();
    flags.set_threshold(50);
    assert_eq!(flags.threshold, 50);
}

#[test]
fn test_action_flags_set_mcp_client() {
    let mut flags = ActionFlags::default();
    flags.set_mcp_client("codebuff");
    assert_eq!(flags.mcp_client, "codebuff");
}

// ============================================================================
// taxonomy_doctor_vo.rs (21 uncovered)
// ============================================================================

#[test]
fn test_doctor_result_vo_new() {
    let result = DoctorResultVO::new(
        DescriptionVO::new("3.9.0"),
        shared_lint_arwaky::common::taxonomy_message_vo::ComplianceStatus::new(true),
        FilePathList::new(vec![FilePath::new("config.yaml").unwrap()]),
        HashMap::new(),
        vec![ErrorMessage::new("issue 1")],
        shared_lint_arwaky::common::taxonomy_message_vo::ComplianceStatus::new(true),
    );
    assert_eq!(result.python_version.value, "3.9.0");
    assert!(result.issues.len() == 1);
}

#[test]
fn test_doctor_result_vo_display() {
    let result = DoctorResultVO::new(
        DescriptionVO::new("3.10.0"),
        shared_lint_arwaky::common::taxonomy_message_vo::ComplianceStatus::new(true),
        FilePathList::default(),
        HashMap::new(),
        vec![],
        shared_lint_arwaky::common::taxonomy_message_vo::ComplianceStatus::new(true),
    );
    let s = result.to_string();
    assert!(s.contains("DoctorResult"));
    assert!(s.contains("3.10.0"));
}

// ============================================================================
// taxonomy_setup_contract_vo.rs (19 uncovered)
// ============================================================================

#[test]
fn test_mcp_binary_name_vo() {
    let name = McpBinaryNameVO::new("lint-arwaky-cli");
    assert_eq!(name.value(), "lint-arwaky-cli");
}

#[test]
fn test_project_language_vo() {
    let lang = ProjectLanguageVO::new("rust");
    assert_eq!(lang.value(), "rust");
}

#[test]
fn test_setup_error_display() {
    let err = SetupError::io("IO error");
    assert_eq!(err.to_string(), "IO error");
    let err2 = SetupError::invalid_state("invalid");
    assert_eq!(err2.to_string(), "invalid");
    let err3 = SetupError::other("other error");
    assert_eq!(err3.to_string(), "other error");
}

#[test]
fn test_setup_error_is_error() {
    let err = SetupError::other("test");
    let _: &dyn std::error::Error = &err;
}

// ============================================================================
// taxonomy_stats_vo.rs (18 uncovered)
// ============================================================================

#[test]
fn test_maintenance_stats_vo_new() {
    let stats = MaintenanceStatsVO::new(
        FilePath::new(".").unwrap(),
        Count::new(42),
        Count::new(10),
        Score::new(0.238),
        Count::new(5),
    );
    assert_eq!(stats.total_files.value, 42);
    assert_eq!(stats.test_files.value, 10);
}

#[test]
fn test_maintenance_stats_vo_display() {
    let stats = MaintenanceStatsVO::new(
        FilePath::new(".").unwrap(),
        Count::new(100),
        Count::new(25),
        Score::new(0.25),
        Count::new(10),
    );
    let s = stats.to_string();
    assert!(s.contains("MaintenanceStats"));
    assert!(s.contains("100"));
    assert!(s.contains("25"));
}

// ============================================================================
// taxonomy_layer_names_vo.rs (19 uncovered)
// ============================================================================

#[test]
fn test_layer_names_functions() {
    assert_eq!(layer_agent().value, "agent");
    assert_eq!(layer_capabilities().value, "capabilities");
    assert_eq!(layer_taxonomy().value, "taxonomy");
    assert_eq!(layer_contract().value, "contract");
    assert_eq!(layer_infrastructure().value, "infrastructure");
    assert_eq!(layer_surfaces().value, "surfaces");
    assert_eq!(layer_root().value, "root");
    assert_eq!(layer_global().value, "global");
}

#[test]
fn test_all_core_layers() {
    let layers = all_core_layers();
    assert_eq!(layers.len(), 7);
    let names: Vec<&str> = layers.iter().map(|l| l.value.as_str()).collect();
    assert!(names.contains(&"agent"));
    assert!(names.contains(&"root"));
}

#[test]
fn test_core_layer_names() {
    let names = core_layer_names();
    assert!(names.contains("agent"));
    assert!(names.contains("taxonomy"));
    assert!(names.contains("surfaces"));
    assert!(!names.contains("global"));
}
