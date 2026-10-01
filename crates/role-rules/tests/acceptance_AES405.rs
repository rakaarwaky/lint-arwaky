// Acceptance test AES405 — Agent composition.
// Agent files must have >= 1 aggregate implementor and max 3 types.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use std::path::PathBuf;

fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::TypeScript | Language::JavaScript => "ts",
            _ => "txt",
        }
        .to_string(),
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

fn run_audit(files: Vec<FileEntry>) -> Vec<shared_common::LintResult> {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    orch.execute(RoleRequest::audit(&files)).into_violations()
}

// ── No implementor → AgentNoImplementor ──

#[test]
fn aes405_no_implementor_detected() {
    let file = make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "pub struct Dispatcher {}\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "agent without implementor should trigger AES405"
    );
}

// ── Too many types → AgentTooManyTypes ──

#[test]
fn aes405_too_many_types_detected() {
    let file = make_file(
        "src/agent_orchestrator.rs",
        Language::Rust,
        "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "agent with 4 types should trigger AES405"
    );
    assert_eq!(aes405[0].severity, shared_common::Severity::HIGH);
}

// ── Valid agent with implementor → no violation ──
//
// A fully valid AES405 agent: aggregate impl first, then the constructor, and
// it injects the two protocol seams an orchestrator is expected to coordinate.

const VALID_RUST_AGENT: &str = "\
pub struct Dispatcher {
    scanner: Arc<dyn IScannerProtocol>,
    reporter: Arc<dyn IReporterProtocol>,
}

impl IDispatcherAggregate for Dispatcher {
    fn execute(&self) {
        self.scanner.scan();
    }
}

impl Dispatcher {
    pub fn new(scanner: Arc<dyn IScannerProtocol>, reporter: Arc<dyn IReporterProtocol>) -> Self {
        Self { scanner, reporter }
    }
}
";

#[test]
fn aes405_valid_agent_no_violation() {
    let file = make_file("src/agent_dispatcher.rs", Language::Rust, VALID_RUST_AGENT);
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        aes405.is_empty(),
        "valid agent with implementor should not trigger AES405, got: {:?}",
        aes405
            .iter()
            .map(|r| r.message.value.as_str())
            .collect::<Vec<_>>()
    );
}

// ── Python: no parent class → AgentNoImplementor ──

#[test]
fn aes405_python_no_parent_detected() {
    let file = make_file(
        "src/agent_dispatcher.py",
        Language::Python,
        "class Dispatcher:\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "python agent without parent should trigger AES405"
    );
}

// ── Python: with parent → no violation ──

#[test]
fn aes405_python_with_parent_no_violation() {
    // A fully valid AES405 Python agent: two injected protocol seams, no I/O,
    // no free functions, no state outside the constructor, no computation.
    let content = "\
class Dispatcher(IDispatcherAggregate):
    def __init__(self, scanner: IScannerProtocol, reporter: IReporterProtocol):
        self._scanner = scanner
        self._reporter = reporter

    def execute(self, request):
        return self._scanner.scan(request)
";
    let file = make_file("src/agent_dispatcher.py", Language::Python, content);
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        aes405.is_empty(),
        "python agent with parent should not trigger AES405, got: {:?}",
        aes405
            .iter()
            .map(|r| r.message.value.as_str())
            .collect::<Vec<_>>()
    );
}

// ── Non-agent file is not checked ──

#[test]
fn aes405_non_agent_file_ignored() {
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        "pub struct Feature {}\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        aes405.is_empty(),
        "non-agent file should not trigger AES405"
    );
}

// ── Contract-protocol implementation → AgentProtocolImplementation ──
//
// An agent is the feature's composition root: it implements the feature
// aggregate and *injects* protocol seams. Implementing a protocol itself makes
// the orchestration layer duplicate a capability's work, so every
// `impl I*Protocol for <AgentType>` is a violation.

fn aes405_messages(files: Vec<FileEntry>) -> Vec<String> {
    run_audit(files)
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .map(|r| r.message.value.to_string())
        .collect()
}

#[test]
fn aes405_contract_protocol_impl_detected() {
    let file = make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "\
pub struct Dispatcher {
    scanner: Arc<dyn IScannerProtocol>,
}

impl IDispatcherAggregate for Dispatcher {
    fn execute(&self) {
        self.scanner.scan();
    }
}

impl IScannerProtocol for Dispatcher {
    fn scan(&self) {}
}

impl Dispatcher {
    pub fn new(scanner: Arc<dyn IScannerProtocol>) -> Self {
        Self { scanner }
    }
}
",
    );
    let messages = aes405_messages(vec![file]);
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "agent implementing IScannerProtocol should trigger AES405, got: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("IScannerProtocol")),
        "the finding should name the offending protocol, got: {messages:?}"
    );
}

#[test]
fn aes405_several_contract_protocol_impls_all_reported() {
    // Both offending protocols are named, so the author can fix them in one pass.
    let file = make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "\
pub struct Dispatcher {
    scanner: Arc<dyn IScannerProtocol>,
    reporter: Arc<dyn IReporterProtocol>,
}

impl IDispatcherAggregate for Dispatcher {
    fn execute(&self) {
        self.scanner.scan();
    }
}

impl IScannerProtocol for Dispatcher {
    fn scan(&self) {}
}

impl IReporterProtocol for Dispatcher {
    fn report(&self) {}
}

impl Dispatcher {
    pub fn new(scanner: Arc<dyn IScannerProtocol>, reporter: Arc<dyn IReporterProtocol>) -> Self {
        Self { scanner, reporter }
    }
}
",
    );
    let messages = aes405_messages(vec![file]);
    let protocol_findings: Vec<&String> = messages
        .iter()
        .filter(|m| m.contains("implements a contract protocol"))
        .collect();
    assert_eq!(
        protocol_findings.len(),
        1,
        "one finding per file, listing every offending protocol: {messages:?}"
    );
    let combined = protocol_findings[0].as_str();
    assert!(
        combined.contains("IScannerProtocol") && combined.contains("IReporterProtocol"),
        "both protocols should be named in the single finding: {combined}"
    );
}

#[test]
fn aes405_std_and_aggregate_impls_allowed() {
    // `Default`, `Display`, and the aggregate are not contract protocols, and
    // injected seams are field types rather than impls — none of these flag.
    let file = make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "\
pub struct Dispatcher {
    scanner: Arc<dyn IScannerProtocol>,
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Dispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, \"dispatcher\")
    }
}

impl IDispatcherAggregate for Dispatcher {
    fn execute(&self) {
        self.scanner.scan();
    }
}

impl Dispatcher {
    pub fn new(scanner: Arc<dyn IScannerProtocol>) -> Self {
        Self { scanner }
    }
}
",
    );
    let messages = aes405_messages(vec![file]);
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "std traits and the aggregate impl are not protocol implementations: {messages:?}"
    );
}

#[test]
fn aes405_python_base_protocol_class_detected() {
    let file = make_file(
        "src/agent_dispatcher.py",
        Language::Python,
        "\
class Dispatcher(IDispatcherAggregate):
    def __init__(self, scanner: IScannerProtocol):
        self._scanner = scanner

    def execute(self, request):
        return self._scanner.scan(request)


class Scanner(IScannerProtocol):
    def scan(self, request):
        return request
",
    );
    let messages = aes405_messages(vec![file]);
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "python agent declaring a protocol base class should trigger AES405: {messages:?}"
    );
}

#[test]
fn aes405_typescript_implements_protocol_detected() {
    let file = make_file(
        "src/agent_dispatcher.ts",
        Language::TypeScript,
        "\
class Dispatcher implements IDispatcherAggregate {
    private scanner: IScannerProtocol;

    constructor(scanner: IScannerProtocol) {
        this.scanner = scanner;
    }

    execute(request: Request): Result {
        return this.scanner.scan(request);
    }
}

class Scanner implements IScannerProtocol {
    scan(request: Request): Result {
        return request;
    }
}
",
    );
    let messages = aes405_messages(vec![file]);
    assert!(
        messages
            .iter()
            .any(|m| m.contains("implements a contract protocol")),
        "typescript agent implementing a protocol should trigger AES405: {messages:?}"
    );
}

// ── Block markers beyond Block 3 → AgentBlockMarkers ──
//
// The structure is Block 1 (types and injected deps) -> Block 2 (aggregate
// impl) -> Block 3 (constructors, std traits, helpers). A Block 4 means the file
// has outgrown the 3-block shape and the behaviour belongs elsewhere.

#[test]
fn aes405_block_marker_beyond_three_detected() {
    let file = make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "\
// ─── Block 1: Struct Definitions ───
pub struct Dispatcher {
    scanner: Arc<dyn IScannerProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ───
impl IDispatcherAggregate for Dispatcher {
    fn execute(&self) {
        self.scanner.scan();
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───
impl Dispatcher {
    pub fn new(scanner: Arc<dyn IScannerProtocol>) -> Self {
        Self { scanner }
    }
}

// ─── Block 4: Extra Seams ───
impl Dispatcher {
    fn helper(&self) {}
}
",
    );
    let messages = aes405_messages(vec![file]);
    assert!(
        messages
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "a Block 4 marker should trigger AES405, got: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("Block 4")),
        "the finding should name the offending block, got: {messages:?}"
    );
}

#[test]
fn aes405_exactly_three_blocks_allowed() {
    let messages = aes405_messages(vec![make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "\
// ─── Block 1: Struct Definitions ───
pub struct Dispatcher {
    scanner: Arc<dyn IScannerProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ───
impl IDispatcherAggregate for Dispatcher {
    fn execute(&self) {
        self.scanner.scan();
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───
impl Dispatcher {
    pub fn new(scanner: Arc<dyn IScannerProtocol>) -> Self {
        Self { scanner }
    }
}
",
    )]);
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "three blocks is the valid structure: {messages:?}"
    );
}

#[test]
fn aes405_prose_block_reference_is_not_a_marker() {
    // Only a `Block <n>:` heading opens a block. Prose that merely mentions a
    // block — e.g. a doc comment describing the three-block structure — must not
    // be read as a marker, or every explanatory comment would false-fire.
    let messages = aes405_messages(vec![make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "\
// Block 1 (types + injected deps) -> Block 2 (aggregate impl) -> Block 3 (helpers).
pub struct Dispatcher {
    scanner: Arc<dyn IScannerProtocol>,
}

impl IDispatcherAggregate for Dispatcher {
    fn execute(&self) {
        self.scanner.scan();
    }
}

impl Dispatcher {
    pub fn new(scanner: Arc<dyn IScannerProtocol>) -> Self {
        Self { scanner }
    }
}
",
    )]);
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("block markers beyond Block 3")),
        "prose mentioning blocks is not a block marker: {messages:?}"
    );
}
