// PURPOSE: structure-rules — AES701 shared purity, AES702 feature health,
// AES703 surface purity, AES704 test-suite coverage, AES705 member-root placement

pub mod agent_structure_orchestrator;
pub use agent_structure_orchestrator::StructureOrchestrator;
pub mod capabilities_shared_purity_auditor;
pub use capabilities_shared_purity_auditor::SharedPurityAuditor;
pub mod capabilities_feature_health_auditor;
pub use capabilities_feature_health_auditor::FeatureHealthAuditor;
pub mod capabilities_surface_purity_auditor;
pub use capabilities_surface_purity_auditor::SurfacePurityAuditor;
pub mod capabilities_member_root_placement_auditor;
pub use capabilities_member_root_placement_auditor::MemberRootPlacementAuditor;
pub mod capabilities_test_suite_coverage_auditor;
pub use capabilities_test_suite_coverage_auditor::TestSuiteCoverageAuditor;
pub mod root_structure_rules_container;
pub use root_structure_rules_container::RootStructureRulesContainer;
