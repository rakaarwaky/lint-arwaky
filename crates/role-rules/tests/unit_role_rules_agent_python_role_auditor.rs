// Unit tests for the Python agent role auditor — AES405 sub-checks.
use role_rules_lint_arwaky::capabilities_agent_python_role_auditor::AgentPythonRoleAuditor;
use shared_common::Severity;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::IAgentRoleProtocol;

use shared_filesystem::taxonomy_filesystem_vo::Language;
use std::path::PathBuf;

fn auditor() -> AgentPythonRoleAuditor {
    AgentPythonRoleAuditor::new()
}

fn make_file(path: &str, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "py".to_string(),
        language: Language::Python,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: false,
        parse_metadata: None,
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
    let f = make_file("src/capabilities_foo.py", "class Foo:\n    pass\n");
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "non-agent layer must not produce violations");
}

// ── P2 implementor ──

#[test]
fn python_no_parent_flagged() {
    let content = "class Dispatcher:\n    pass\n";
    let f = make_file("src/agent_dispatcher.py", content);
    let mut v = Vec::new();
    auditor().check_agent_implementor(&f, &mut v);
    assert!(!v.is_empty(), "no aggregate base should be flagged");
}

#[test]
fn python_with_parent_no_violation() {
    let content = "class Dispatcher(IDispatcherAggregate):\n    pass\n";
    let f = make_file("src/agent_dispatcher.py", content);
    let mut v = Vec::new();
    auditor().check_agent_implementor(&f, &mut v);
    assert!(v.is_empty(), "aggregate base should pass");
}

// ── P3 type budget ──

#[test]
fn python_too_many_classes_flagged() {
    let content =
        "class A:\n    pass\nclass B:\n    pass\nclass C:\n    pass\nclass D:\n    pass\n";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_type_budget(&f, &mut v);
    assert!(!v.is_empty(), "4 classes should be flagged");
    assert_eq!(v[0].severity, Severity::HIGH);
}

// ── P16 Any annotation ──

#[test]
fn python_any_annotation_flagged() {
    let content = "def process(value: any) -> Any:\n    return value\n";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_any_annotation(&f, &mut v);
    assert!(!v.is_empty(), "any/Any annotation should be flagged");
}

#[test]
fn python_no_any_no_violation() {
    let content = "def process(value: int) -> str:\n    return str(value)\n";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_any_annotation(&f, &mut v);
    assert!(v.is_empty(), "typed signature should pass");
}

// ── P6 forbidden I/O ──

#[test]
fn python_file_io_flagged() {
    let content = "class Orchestrator:\n    def execute(self):\n        with open('x') as f:\n            pass\n";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_io_forbidden(&f, &mut v);
    assert!(!v.is_empty(), "open() in agent should be flagged");
}

#[test]
fn python_no_io_no_violation() {
    let content = "\
class Orchestrator:
    def __init__(self, scanner):
        self._scanner = scanner

    def execute(self):
        return self._scanner.scan()
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_io_forbidden(&f, &mut v);
    assert!(v.is_empty(), "no I/O should pass");
}

// ── P12 constant placement ──

#[test]
fn python_module_constant_flagged() {
    let content = "\
CORE_VERSION = \"2.1.0\"

class Orchestrator:
    def execute(self):
        pass
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_constant_placement(&f, &mut v);
    assert!(!v.is_empty(), "module-level constant should be flagged");
}

#[test]
fn python_no_module_constant_no_violation() {
    let content = "\
class Orchestrator:
    VERSION = \"1.0\"

    def execute(self):
        pass
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_constant_placement(&f, &mut v);
    assert!(v.is_empty(), "class attribute is not a module constant");
}

// ── P14 subsystem count ──

#[test]
fn p14_single_protocol_param_flagged() {
    let content = "\
class Orchestrator:
    def __init__(self, checker: ICheckerProtocol):
        self._checker = checker
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(!v.is_empty(), "single protocol param should be flagged");
    assert_eq!(v[0].severity, Severity::LOW);
}

#[test]
fn p14_two_protocol_params_no_violation() {
    let content = "\
class Orchestrator:
    def __init__(self, checker: ICheckerProtocol, reporter: IReportProtocol):
        self._checker = checker
        self._reporter = reporter
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(v.is_empty(), "two protocol params should pass");
}

#[test]
fn p14_single_subsystem_feature_skipped() {
    let content = "\
class Orchestrator:
    def __init__(self, checker: ICheckerProtocol):
        self._checker = checker
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 1, &mut v);
    assert!(v.is_empty(), "single-subsystem feature should be skipped");
}

// ── P7 computation ──

#[test]
fn python_sum_flagged() {
    let content = "\
class Orchestrator:
    def execute(self, values):
        return sum(values)
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_computation(&f, &mut v);
    assert!(!v.is_empty(), "sum() in agent should be flagged");
}

// ── P21 abstract method ──

#[test]
fn python_abstractmethod_flagged() {
    let content = "\
class Orchestrator:
    @abstractmethod
    def execute(self):
        pass
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_abstract_method(&f, &mut v);
    assert!(!v.is_empty(), "@abstractmethod in agent should be flagged");
}

#[test]
fn python_no_abstractmethod_no_violation() {
    let content = "\
class Orchestrator:
    def execute(self):
        pass
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_abstract_method(&f, &mut v);
    assert!(v.is_empty(), "no @abstractmethod should pass");
}

// ── P13 stateless ──

#[test]
fn python_state_assignment_outside_init_flagged() {
    let content = "\
class Orchestrator:
    def __init__(self, scanner):
        self._scanner = scanner
        self._count = 0

    def execute(self):
        self._count = 1
        return self._scanner.scan()
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_stateless(&f, &mut v);
    assert!(
        !v.is_empty(),
        "self._count = 1 outside init should be flagged"
    );
}

#[test]
fn python_append_to_buffer_no_violation() {
    let content = "\
class Orchestrator:
    def __init__(self, scanner):
        self._scanner = scanner
        self._history = []

    def execute(self, item):
        self._history.append(item)
        return self._scanner.scan(item)
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_stateless(&f, &mut v);
    assert!(v.is_empty(), "appending to a results buffer should pass");
}

#[test]
fn python_assignment_in_init_no_violation() {
    let content = "\
class Orchestrator:
    def __init__(self, scanner):
        self._scanner = scanner
        self._count = 0
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_stateless(&f, &mut v);
    assert!(v.is_empty(), "assignment in __init__ should pass");
}

// ── P11 free functions ──

#[test]
fn python_module_level_fn_flagged() {
    let content = "\
def helper(value):
    return value + 1

class Orchestrator:
    def execute(self):
        pass
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_free_fn(&f, &mut v);
    assert!(!v.is_empty(), "module-level def should be flagged");
}

#[test]
fn python_no_module_level_fn_no_violation() {
    let content = "\
class Orchestrator:
    def execute(self):
        pass
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_free_fn(&f, &mut v);
    assert!(v.is_empty(), "only class methods should pass");
}

// ── P14: wrapped `__init__` params ──

#[test]
fn p14_wrapped_init_params_counted() {
    // The `Deps` class annotates its protocols across a wrapped signature,
    // which is the shape the good workspaces use. Each param must contribute
    // its annotation, not the whole `name: Type` text.
    let content = "\
class OrchestratorDeps:
    def __init__(
        self,
        addition: ICalculatorProtocol,
        subtraction: ICalculatorProtocol,
        multiplication: ICalculatorProtocol,
        division: ICalculatorProtocol,
    ):
        self.addition = addition
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(v.is_empty(), "four wrapped protocol params should pass");
}

#[test]
fn p14_wrapped_init_single_param_flagged() {
    let content = "\
class OrchestratorDeps:
    def __init__(
        self,
        addition: ICalculatorProtocol,
    ):
        self.addition = addition
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(
        !v.is_empty(),
        "one wrapped protocol param should be flagged"
    );
}

#[test]
fn p14_default_value_param_counted_by_annotation() {
    // `addition: ICalculatorProtocol = None` has a default; the annotation
    // before the `=` is what identifies the protocol.
    let content = "\
class Orchestrator:
    def __init__(
        self,
        addition: ICalculatorProtocol = None,
        subtraction: ICalculatorProtocol = None,
    ):
        self.addition = addition
        self.subtraction = subtraction
";
    let f = make_file("src/agent_thing.py", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(v.is_empty(), "defaulted protocol params should be counted");
}
