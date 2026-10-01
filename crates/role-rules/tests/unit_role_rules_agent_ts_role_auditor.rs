// Unit tests for the TypeScript agent role auditor — AES405 sub-checks.
use role_rules_lint_arwaky::capabilities_agent_ts_role_auditor::AgentTsRoleAuditor;
use shared_common::Severity;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::IAgentRoleProtocol;

use shared_filesystem::taxonomy_filesystem_vo::Language;
use std::path::PathBuf;

fn auditor() -> AgentTsRoleAuditor {
    AgentTsRoleAuditor::new()
}

fn make_file(path: &str, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "ts".to_string(),
        language: Language::TypeScript,
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
    let f = make_file("src/capabilities_foo.ts", "export class Foo {}\n");
    let mut v = Vec::new();
    auditor().check_agent_routing(&f, "capabilities", &mut v);
    assert!(v.is_empty(), "non-agent layer must not produce violations");
}

// ── P2 implementor ──

#[test]
fn ts_no_implements_flagged() {
    let content = "export class Orchestrator {\n  execute() {}\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_implementor(&f, &mut v);
    assert!(!v.is_empty(), "no aggregate implements should be flagged");
}

#[test]
fn ts_with_implements_no_violation() {
    let content =
        "export class Orchestrator implements IOrchestratorAggregate {\n  execute() {}\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_implementor(&f, &mut v);
    assert!(v.is_empty(), "aggregate implements should pass");
}

// ── P3 type budget ──

#[test]
fn ts_too_many_types_flagged() {
    let content = "
export interface A {}
export interface B {}
export interface C {}
export interface D {}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_type_budget(&f, &mut v);
    assert!(!v.is_empty(), "4 interface declarations should be flagged");
    assert_eq!(v[0].severity, Severity::HIGH);
}

// ── P16 Any annotation ──

#[test]
fn ts_any_annotation_flagged() {
    let content = "export class Orchestrator {\n  process(v: any) {}\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_any_annotation(&f, &mut v);
    assert!(!v.is_empty(), "any annotation should be flagged");
}

#[test]
fn ts_no_any_no_violation() {
    let content = "export class Orchestrator {\n  process(v: string) {}\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_any_annotation(&f, &mut v);
    assert!(v.is_empty(), "typed param should pass");
}

// ── P6 forbidden I/O ──

#[test]
fn ts_fs_import_flagged() {
    let content = "import fs from 'fs';\n\nexport class Orchestrator {\n  execute() {}\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_io_forbidden(&f, &mut v);
    assert!(!v.is_empty(), "fs import in agent should be flagged");
}

#[test]
fn ts_no_io_no_violation() {
    let content = "export class Orchestrator {\n  constructor(private scanner: IScannerProtocol) {}\n  execute() { return this.scanner.scan(); }\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_io_forbidden(&f, &mut v);
    assert!(v.is_empty(), "no I/O should pass");
}

// ── P12 constant placement ──

#[test]
fn ts_top_level_const_flagged() {
    let content = "const MAX: number = 10;\n\nexport class Orchestrator {\n  execute() {}\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_constant_placement(&f, &mut v);
    assert!(!v.is_empty(), "top-level const should be flagged");
}

// ── P14 subsystem count ──

#[test]
fn p14_single_field_flagged() {
    let content = "
export class Orchestrator {
  private readonly checker: ICheckerProtocol;
  constructor(checker: ICheckerProtocol) {
    this.checker = checker;
  }
  execute() {}
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(!v.is_empty(), "single protocol field should be flagged");
    assert_eq!(v[0].severity, Severity::LOW);
}

#[test]
fn p14_two_fields_no_violation() {
    let content = "
export class Orchestrator {
  private readonly checker: ICheckerProtocol;
  private readonly reporter: IReportProtocol;
  constructor(checker: ICheckerProtocol, reporter: IReportProtocol) {
    this.checker = checker;
    this.reporter = reporter;
  }
  execute() {}
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(v.is_empty(), "two protocol fields should pass");
}

#[test]
fn p14_single_subsystem_feature_skipped() {
    let content = "
export class Orchestrator {
  private readonly checker: ICheckerProtocol;
  constructor(checker: ICheckerProtocol) {
    this.checker = checker;
  }
  execute() {}
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 1, &mut v);
    assert!(v.is_empty(), "single-subsystem feature should be skipped");
}

// ── P7 computation ──

#[test]
fn ts_reduce_flagged() {
    let content = "export class Orchestrator {\n  total(vals: number[]) { return vals.reduce((a,b)=>a+b, 0); }\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_computation(&f, &mut v);
    assert!(!v.is_empty(), ".reduce() in agent should be flagged");
}

// ── P21 abstract class ──

#[test]
fn ts_abstract_class_flagged() {
    let content = "abstract class BaseOrchestrator {\n  abstract execute(): void;\n}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_abstract_method(&f, &mut v);
    assert!(!v.is_empty(), "abstract class in agent should be flagged");
}

// ── P13 stateless ──

#[test]
fn ts_this_assignment_outside_ctor_flagged() {
    let content = "\
export class Orchestrator {\n\
  private count: number = 0;\n\
  constructor() {\n\
    this.count = 0;\n\
  }\n\
  execute() {\n\
    this.count = 1;\n\
  }\n\
}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_stateless(&f, &mut v);
    assert!(
        !v.is_empty(),
        "this.count = 1 outside ctor should be flagged"
    );
}

#[test]
fn ts_no_assignment_no_violation() {
    let content = "\
export class Orchestrator {\n\
  private readonly scanner: IScannerProtocol;\n\
  constructor(scanner: IScannerProtocol) {\n\
    this.scanner = scanner;\n\
  }\n\
  execute() { return this.scanner.scan(); }\n\
}\n";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_stateless(&f, &mut v);
    assert!(v.is_empty(), "assignment in ctor should pass");
}

// ── P11 free functions ──

#[test]
fn ts_module_level_function_flagged() {
    let content = "
function helper(x: number): number {
  return x + 1;
}

export class Orchestrator {
  execute() {}
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_free_fn(&f, &mut v);
    assert!(!v.is_empty(), "module-level function should be flagged");
}

#[test]
fn ts_no_module_level_fn_no_violation() {
    let content = "
export class Orchestrator {
  execute() {}
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_free_fn(&f, &mut v);
    assert!(v.is_empty(), "no module-level functions should pass");
}

// ── P14: TS interface and field injection ──

#[test]
fn p14_interface_fields_counted() {
    // A `Deps` interface with multiple protocol fields is how the good
    // packages fixture wires its subsystems — the interface members are
    // injection sites even though the class holds the interface as a field.
    let content = "\
export interface OrchestratorDeps {
  addition: ICalculatorProtocol;
  subtraction: ICalculatorProtocol;
  multiplication: ICalculatorProtocol;
  division: ICalculatorProtocol;
}

export class Orchestrator {
  private deps: OrchestratorDeps;
  constructor(deps: OrchestratorDeps) { this.deps = deps; }
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(v.is_empty(), "four interface protocol fields should pass");
}

#[test]
fn p14_interface_single_field_flagged() {
    let content = "\
export interface OrchestratorDeps {
  checker: ICalculatorProtocol;
}

export class Orchestrator {
  private deps: OrchestratorDeps;
  constructor(deps: OrchestratorDeps) { this.deps = deps; }
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(
        !v.is_empty(),
        "one interface protocol field should be flagged"
    );
}

#[test]
fn p14_wrapped_constructor_shorthand_counted() {
    // A constructor shorthand with the params spread across several lines
    // is the shape the good TS fixture uses; `read_wrapped_params` joins the
    // continuation lines so each `private readonly dep: IProtocol` is counted.
    let content = "\
export class Orchestrator {
  constructor(
    private readonly addition: ICalculatorProtocol,
    private readonly subtraction: ICalculatorProtocol,
  ) {}
}
";
    let f = make_file("src/agent_thing.ts", content);
    let mut v = Vec::new();
    auditor().check_agent_subsystem_count(&f, 2, &mut v);
    assert!(
        v.is_empty(),
        "two wrapped constructor shorthand fields should pass"
    );
}
