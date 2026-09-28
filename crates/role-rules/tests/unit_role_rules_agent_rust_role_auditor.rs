// Unit tests for the Rust agent role auditor — AES405 sub-checks.
use role_rules_lint_arwaky::capabilities_agent_rust_role_auditor::AgentRustRoleAuditor;
use shared::common::Severity;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::IAgentRoleProtocol;

use shared::filesystem::taxonomy_filesystem_vo::{Language, ParseMetadata, RustMetadata};
use std::path::PathBuf;

fn auditor() -> AgentRustRoleAuditor {
    AgentRustRoleAuditor::new()
}

fn make_file(path: &str, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: false,
        parse_metadata: None,
    }
}

fn make_file_with_rust_meta(path: &str, meta: RustMetadata) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 100,
        content: String::new(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Rust(meta)),
    }
}

// ── Construction ──

#[test]
fn construction_succeeds() {
    let _ = auditor();
}

// ── Layer routing ──

#[test]
fn non_agent_layer_skipped() {
    let f = make_file("src/capabilities_foo.rs", "pub struct Foo {}");
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "non-agent layer must not produce violations");
}

#[test]
fn agent_layer_with_agent_prefix_routes() {
    let f = make_file("src/capabilities_foo.rs", "pub struct Foo {}");
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "agent(capabilities)", &mut v);
    assert!(
        !v.is_empty(),
        "agent(capabilities) should route to agent auditor"
    );
}

// ── P2 implementor / P3 type budget (fallback line scan) ──

#[test]
fn fallback_rust_no_implementor_flagged() {
    let content = "pub struct Foo {}\npub struct Bar {}\n";
    let f = make_file("src/agent_something.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "agent", &mut v);
    assert!(!v.is_empty(), "should flag missing implementor");
    assert_eq!(v[0].code.code(), "AES405");
}

#[test]
fn fallback_rust_too_many_types_flagged() {
    let content = "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n";
    let f = make_file("src/agent_something.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "agent", &mut v);
    assert!(!v.is_empty(), "should flag too many types");
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn fallback_rust_valid_composition_no_violation() {
    let content = "pub struct Foo {}\nimpl IFooAggregate for Foo {}\n";
    let f = make_file("src/agent_something.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "agent", &mut v);
    assert!(v.is_empty(), "valid agent composition should pass");
}

// ── Metadata path ──

#[test]
fn metadata_rust_no_implementor_flagged() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into(), "Bar".into()],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/agent_something.rs", meta);
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "agent", &mut v);
    assert!(
        !v.is_empty(),
        "should flag missing implementor via metadata"
    );
    assert_eq!(v[0].code.code(), "AES405");
}

#[test]
fn metadata_rust_too_many_types_flagged() {
    let meta = RustMetadata {
        struct_definitions: vec!["A".into(), "B".into()],
        enum_definitions: vec!["C".into(), "D".into()],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/agent_something.rs", meta);
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "agent", &mut v);
    assert!(!v.is_empty(), "should flag too many types via metadata");
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn metadata_rust_valid_composition_no_violation() {
    let meta = RustMetadata {
        struct_definitions: vec!["Foo".into()],
        impl_blocks: vec![shared::filesystem::taxonomy_filesystem_vo::RustImplItem {
            trait_name: Some("IFooAggregate".into()),
            trait_path: None,
            implementor_type: "Foo".into(),
            has_generics: false,
        }],
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/agent_something.rs", meta);
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "agent", &mut v);
    assert!(v.is_empty(), "valid metadata composition should pass");
}

// ── P4 block order ──

#[test]
fn p4_inherent_impl_before_aggregate_flagged() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
}

impl Orchestrator {
    pub fn new(checker: Arc<dyn ICheckerProtocol>) -> Self {
        Self { checker }
    }
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_block_order(&f, &mut v);
    assert!(
        !v.is_empty(),
        "inherent impl before aggregate should be flagged"
    );
    assert_eq!(v[0].severity, Severity::HIGH);
}

#[test]
fn p4_aggregate_before_inherent_no_violation() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}

impl Orchestrator {
    pub fn new(checker: Arc<dyn ICheckerProtocol>) -> Self {
        Self { checker }
    }
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_block_order(&f, &mut v);
    assert!(v.is_empty(), "aggregate before inherent should pass");
}

// ── P6 forbidden I/O ──

#[test]
fn p6_filesystem_access_flagged() {
    let content = "\
pub struct Orchestrator {}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {
        let _ = std::fs::read_to_string(\"x\");
    }
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_io_forbidden(&f, &mut v);
    assert!(!v.is_empty(), "std::fs in agent should be flagged");
}

#[test]
fn p6_no_io_no_violation() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {
        self.checker.audit();
    }
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_io_forbidden(&f, &mut v);
    assert!(v.is_empty(), "no I/O should pass");
}

// ── P12 constant placement ──

#[test]
fn p12_file_level_const_flagged() {
    let content = "\
const MAX_RETRIES: usize = 3;

pub struct Orchestrator {}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_constant_placement(&f, &mut v);
    assert!(!v.is_empty(), "file-level const should be flagged");
}

#[test]
fn p12_no_const_no_violation() {
    let content = "\
pub struct Orchestrator {}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_constant_placement(&f, &mut v);
    assert!(v.is_empty(), "no const should pass");
}

// ── P14 subsystem count ──

#[test]
fn p14_single_protocol_field_flagged() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    // feature_protocol_count = 2 (not a single-subsystem feature)
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(!v.is_empty(), "single protocol field should be flagged");
    assert_eq!(v[0].severity, Severity::LOW);
}

#[test]
fn p14_two_protocol_fields_no_violation() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
    reporter: Arc<dyn IReportProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(v.is_empty(), "two protocol fields should pass");
}

#[test]
fn p14_single_subsystem_feature_skipped() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    // feature_protocol_count = 1 → single-subsystem feature, check skipped
    auditor().check_agent_subsystem_count(&f, 1, &mut v);
    assert!(v.is_empty(), "single-subsystem feature should be skipped");
}

// ── P7 computation ──

#[test]
fn p7_sum_flagged() {
    let content = "\
pub struct Orchestrator {}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) -> usize {
        let vals = vec![1, 2, 3];
        vals.iter().sum::<usize>()
    }
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_computation(&f, &mut v);
    assert!(!v.is_empty(), "sum::<usize>() in agent should be flagged");
}

#[test]
fn p7_no_computation_no_violation() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {
        self.checker.audit();
    }
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_computation(&f, &mut v);
    assert!(v.is_empty(), "no computation should pass");
}

// ── P13 stateless ──

#[test]
fn p13_mut_self_flagged() {
    let content = "\
pub struct Orchestrator {
    count: usize,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&mut self) {
        self.count += 1;
    }
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_stateless(&f, &mut v);
    assert!(!v.is_empty(), "&mut self in agent should be flagged");
}

#[test]
fn p13_immutable_self_no_violation() {
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {
        self.checker.audit();
    }
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_stateless(&f, &mut v);
    assert!(v.is_empty(), "&self should pass");
}

// ── P11 free functions ──

#[test]
fn p11_free_fn_flagged() {
    let content = "\
pub struct Orchestrator {}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}

fn helper(x: usize) -> usize {
    x + 1
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_free_fn(&f, &mut v);
    assert!(!v.is_empty(), "free fn in agent should be flagged");
}

#[test]
fn p11_no_free_fn_no_violation() {
    let content = "\
pub struct Orchestrator {}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_free_fn(&f, &mut v);
    assert!(v.is_empty(), "no free fn should pass");
}

// ── P14: single-subsystem skip requires exactly one seam ──

#[test]
fn p14_multi_seam_single_protocol_feature_still_flagged() {
    // The feature declares one protocol trait, but the agent injects four
    // seams of it. Four coordinate-able subsystems is not a single-subsystem
    // feature, so the skip must not fire.
    let content = "\
pub struct Orchestrator {
    addition: Box<dyn ICalculatorProtocol>,
    subtraction: Box<dyn ICalculatorProtocol>,
    multiplication: Box<dyn ICalculatorProtocol>,
    division: Box<dyn ICalculatorProtocol>,
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 1, &mut v);
    assert!(
        v.is_empty(),
        "four seams should pass regardless of feature count"
    );
}

#[test]
fn p14_last_field_without_comma_counted() {
    // The final field in a Rust struct body has no trailing comma.
    let content = "\
pub struct Orchestrator {
    checker: Arc<dyn ICheckerProtocol>,
    reporter: Arc<dyn IReportProtocol>
}

impl IRunnerAggregate for Orchestrator {
    fn execute(&self) {}
}
";
    let f = make_file("src/agent_thing.rs", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(v.is_empty(), "last field without comma should be counted");
}
