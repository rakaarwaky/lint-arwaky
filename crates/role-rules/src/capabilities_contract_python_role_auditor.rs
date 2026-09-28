// PURPOSE: python contract role auditor — AES402 sub-checks for Python contract files.
//
// Handles the Python contract seam: a `class I<Name>Protocol(ABC)` whose
// methods are `@abstractmethod def` stubs ending in `...` or `pass`, and a
// `class I<Name>Aggregate(ABC)` with exactly one `execute` method.
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor by
// `file.language` and calls the trait entry point; all five AES402 sub-checks
// then run here. The language-independent pieces (contract-file recognition,
// the LintResult shape, the I/O exemption) live in
// utility_contract_role_checker.rs so the three auditors share one copy.

use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_signature_parser::{
    extract_python_method_signatures, python_signature_uses_forbidden_primitive,
};
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::contract_role_protocol::IContractRoleProtocol;

use super::utility_contract_role_checker as utility;

// === Block 1: Type Definition ===

pub struct ContractPythonRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl IContractRoleProtocol for ContractPythonRoleAuditor {
    fn check_contract_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if !utility::is_contract_layer(layer) {
            return;
        }
        self._contract_primitive(file, violations);
        self.check_contract_default_body(file, violations);
        self.check_contract_aggregate_method_count(file, violations);
        self.check_contract_dispatch_bag(file, violations);
        self.check_contract_untyped_return(file, violations);
    }

    fn check_contract_default_body(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._default_body_python(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_contract_aggregate_method_count(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    ) {
        self._aggregate_method_count_python(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_contract_dispatch_bag(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._dispatch_bag_python(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_contract_untyped_return(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._untyped_return_python(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_protocol(&self, file: &FileEntry) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_routing(file, "contract", &mut violations);
        violations
    }

    fn check_aggregate(&self, file: &FileEntry) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_routing(file, "contract", &mut violations);
        violations
    }
}

// === Block 3: Helpers ===

impl Default for ContractPythonRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractPythonRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    // ===============================================================
    // Contract primitive — no primitive types in a method signature
    // ===============================================================

    /// Extract `def` signatures declared inside every Python class in the file
    /// and report the forbidden primitive types each one carries.
    fn _contract_primitive(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path = file.path.to_string_lossy().to_string();
        if !utility::is_contract_file(&path) || utility::is_io_exemption(&path) {
            return;
        }
        let content = &file.content;
        for (line_no, sig) in extract_python_method_signatures(content) {
            let forbidden = python_signature_uses_forbidden_primitive(&sig);
            if forbidden.is_empty() {
                continue;
            }
            let why = format!(
                "Forbidden primitive types in signature: {}. \
                 A contract signature crosses a layer boundary, so it must speak \
                 in taxonomy VOs or constants, not Python primitives.",
                forbidden.join(", ")
            );
            violations.push(utility::build_violation(
                &path,
                line_no as u32,
                utility::rule_code(),
                Severity::HIGH,
                &why,
                "Replace the primitive types with the matching Value Objects (VO) or constants from the taxonomy layer.",
                "Contract or method signature uses primitive types instead of taxonomy VO or constant.",
            ));
        }
    }

    // ===============================================================
    // Default body — an abstract method must be a stub, not a body
    // ===============================================================

    /// Report every `def` inside a contract class whose body is more than the
    /// `...` or `pass` stub. A body inside a contract class makes the method
    /// silently return the stub value at every call site.
    fn _default_body_python(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut class_indent: Option<usize> = None;
        let mut class_name = String::new();

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            let indent = l.len() - l.trim_start().len();

            if let Some(name) = class_header(t)
                && class_indent.is_none_or(|ci| indent <= ci)
            {
                class_name = name;
                class_indent = Some(indent);
                continue;
            }
            let Some(ci) = class_indent else {
                continue;
            };
            if !t.is_empty() && indent < ci {
                // An outer dedent that is not the class's own line closes the
                // class body (e.g. `__all__ = [...]` after the class).
                class_indent = None;
                continue;
            }
            if !t.starts_with("def ") && !t.starts_with("async def ") {
                continue;
            }
            let method = def_name(t);
            if is_stub_body(&lines, i, indent) {
                continue;
            }
            violations.push(utility::build_violation(
                path,
                i as u32 + 1,
                utility::rule_code(),
                Severity::HIGH,
                &format!(
                    "`{method}` in class `{class_name}` has an implementation body. \
                     A contract method is a declaration: it promises the shape of a call, \
                     and a default body hands every implementor a silent no-op."
                ),
                "Replace the body with the `...` stub (keeping `@abstractmethod`) so each implementor supplies the behaviour.",
                "Contract method has a default body.",
            ));
        }
    }

    // ===============================================================
    // Aggregate method count — exactly one method on the aggregate
    // ===============================================================

    /// Report a contract class whose name ends in `Aggregate` that declares
    /// anything other than a single `execute` method. More than one entry
    /// point makes the aggregate a capability seam, which belongs in
    /// `_protocol`.
    fn _aggregate_method_count_python(
        &self,
        content: &str,
        path: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if !utility::is_contract_file(path) {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut in_aggregate = false;
        let mut class_indent = 0usize;
        let mut class_name = String::new();
        let mut header_line = 0usize;
        let mut methods: Vec<String> = Vec::new();

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            let indent = l.len() - l.trim_start().len();

            if let Some(name) = class_header(t)
                && name.ends_with("Aggregate")
            {
                in_aggregate = true;
                class_name = name;
                header_line = i;
                class_indent = indent;
                methods.clear();
                continue;
            }
            if !in_aggregate {
                continue;
            }
            // An outer dedent that is not the class's own line closes the
            // class body (e.g. `__all__ = [...]` after the class).
            if !t.is_empty() && indent < class_indent {
                if methods.len() > 1 {
                    report_python_aggregate(
                        path,
                        &class_name,
                        header_line + 1,
                        &methods,
                        violations,
                    );
                }
                in_aggregate = false;
                methods.clear();
                continue;
            }
            if t.starts_with("def ") || t.starts_with("async def ") {
                methods.push(def_name(t));
            }
        }
        // Flush the last class — if the file ends inside an aggregate without
        // any dedent, we still need to report on the methods we collected.
        if in_aggregate && methods.len() > 1 {
            report_python_aggregate(path, &class_name, header_line + 1, &methods, violations);
        }
    }

    // ===============================================================
    // Dispatch bag — an operation string is a dispatcher inside a contract
    // ===============================================================

    /// Report a contract method that takes an operation name plus a bag of
    /// loose values, i.e. `def execute(self, op: str, ...)`. The capability
    /// then branches on `op` itself, so the contract no longer names what it
    /// does.
    fn _dispatch_bag_python(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        for (i, l) in content.lines().enumerate() {
            let t = l.trim();
            if !t.starts_with("def ") && !t.starts_with("async def ") {
                continue;
            }
            let Some(params) = def_parameter_list(t) else {
                continue;
            };
            if !carries_operation_name(&params) {
                continue;
            }
            let method = def_name(t);
            violations.push(utility::build_violation(
                path,
                i as u32 + 1,
                utility::rule_code(),
                Severity::MEDIUM,
                &format!(
                    "`{method}` takes an operation name alongside its other parameters. \
                     A capability that switches on a caller-supplied operation string is a \
                     dispatcher, and the contract no longer names what it does."
                ),
                "Split the operation into its own method with a VO-typed parameter, so the signature states the intent.",
                "Contract seam takes an operation-name dispatch parameter.",
            ));
        }
    }

    // ===============================================================
    // Untyped return — a response enum of bare primitives
    // ===============================================================

    /// Report a multi-member `Enum` subclass in the contract file where every
    /// member holds only primitives. A response type with no VO field carries
    /// no domain meaning back across the layer boundary.
    fn _untyped_return_python(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut in_enum = false;
        let mut class_indent = 0usize;
        let mut enum_name = String::new();
        let mut header_line = 0usize;
        let mut members = 0usize;
        let mut carries_vo = false;

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            let indent = l.len() - l.trim_start().len();

            if let Some(name) = enum_class_header(t) {
                in_enum = true;
                enum_name = name;
                header_line = i;
                class_indent = indent;
                members = 0;
                carries_vo = false;
                continue;
            }
            if !in_enum {
                continue;
            }
            if !t.is_empty() && indent < class_indent {
                report_python_enum(
                    path,
                    &enum_name,
                    header_line + 1,
                    members,
                    carries_vo,
                    violations,
                );
                in_enum = false;
                continue;
            }
            if is_enum_member(t) {
                members += 1;
                if assignment_carries_vo(t) {
                    carries_vo = true;
                }
            }
        }
        // Flush the last enum — a file that ends inside the class body has no
        // dedent line to close it.
        if in_enum {
            report_python_enum(
                path,
                &enum_name,
                header_line + 1,
                members,
                carries_vo,
                violations,
            );
        }
    }
}

// === Free Functions ===

/// The class name from a `class Name(...):` header line.
fn class_header(line: &str) -> Option<String> {
    let rest = line.strip_prefix("class ")?;
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() {
        return None;
    }
    Some(name)
}

/// The enum name from a `class Name(Enum):` header line.
fn enum_class_header(line: &str) -> Option<String> {
    let rest = line.strip_prefix("class ")?;
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() {
        return None;
    }
    let bases = rest[name.len()..].to_lowercase();
    if !(bases.contains("enum") || bases.contains("intenum") || bases.contains("streenum")) {
        return None;
    }
    Some(name)
}

/// The name of a `def name(...)` declaration.
fn def_name(line: &str) -> String {
    line.strip_prefix("async def ")
        .or_else(|| line.strip_prefix("def "))
        .unwrap_or("")
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

/// The text between the `def` name's `(` and its balancing `)`.
fn def_parameter_list(line: &str) -> Option<String> {
    let open = line.find('(')?;
    let mut depth = 0i32;
    for (offset, ch) in line[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(line[open + 1..open + offset].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// True when the parameter list carries a caller-supplied operation name.
fn carries_operation_name(params: &str) -> bool {
    params.split(',').any(|p| {
        let lhs = p.split(':').next().unwrap_or("").trim();
        let ty = p.split(':').nth(1).unwrap_or("").trim().to_lowercase();
        matches!(
            lhs,
            "op" | "operation" | "command" | "action" | "kind" | "mode"
        ) && (ty.starts_with("str") || ty.starts_with("string"))
    })
}

/// True when the lines under a `def` header are the `...` or `pass` stub.
///
/// A docstring between the header and the stub counts as the stub: a contract
/// method that documents itself and then raises nothing is a declaration.
fn is_stub_body(lines: &[&str], header_idx: usize, header_indent: usize) -> bool {
    // A one-line `def ...: ...` is itself the stub.
    if lines[header_idx].trim_end().ends_with(": ...")
        || lines[header_idx].trim_end().ends_with(": pass")
    {
        return true;
    }
    let mut idx = header_idx + 1;
    loop {
        let Some(line) = lines.get(idx) else {
            return true;
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            idx += 1;
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent <= header_indent {
            // Dedented out of the method body — a bare `def ...:` with no
            // implementation is still a declaration, not a body.
            return true;
        }
        // A docstring followed (or not) by the stub is documentation, not
        // behaviour. Skip it, then look for the `...` / `pass` terminator.
        if is_docstring_start(trimmed) {
            idx += docstring_span(lines, idx, indent);
            continue;
        }
        return trimmed == "..." || trimmed == "pass" || trimmed.starts_with("...");
    }
}

/// True when `line` opens a string-literal docstring.
fn is_docstring_start(line: &str) -> bool {
    line.starts_with("\"\"\"")
        || line.starts_with("'''")
        || line.starts_with("r\"\"\"")
        || line.starts_with("b\"\"\"")
        || line.starts_with("u\"\"\"")
}

/// How many lines the docstring opened at `idx` spans, counted from `idx`.
fn docstring_span(lines: &[&str], idx: usize, indent: usize) -> usize {
    let first = lines[idx].trim();
    // A single-line docstring (three quotes on both ends) is one line.
    if first.len() > 6 && first.ends_with("\"\"\"") && first.rfind("\"\"\"") != Some(0) {
        return 1;
    }
    let closing = if first.starts_with("'''") || first.contains("'''") {
        "'''"
    } else {
        "\"\"\""
    };
    for (offset, l) in lines.iter().enumerate().skip(idx + 1) {
        let t = l.trim();
        if t.ends_with(closing) {
            return offset - idx + 1;
        }
        // An outdent below the docstring's indent closes the string early.
        if !t.is_empty() && l.len() - l.trim_start().len() < indent {
            return 1;
        }
    }
    1
}

/// True when a line declares an enum member, i.e. `NAME = ...`.
fn is_enum_member(line: &str) -> bool {
    !line.is_empty()
        && !line.starts_with('#')
        && !line.starts_with("\"")
        && !line.starts_with("'")
        && line.contains('=')
        && line
            .split('=')
            .next()
            .is_some_and(|name| is_identifier(name.trim()))
}

/// True when a member's value or tuple carries a non-primitive type name.
fn assignment_carries_vo(line: &str) -> bool {
    let Some((lhs, rhs)) = line.split_once('=') else {
        return false;
    };
    // `NAME: ResultVO = (...)` — the annotation names the payload type.
    if let Some((_, annotation)) = lhs.split_once(':') {
        return is_vo_name(annotation.trim());
    }
    // `NAME = <value>`. A quoted or bare literal is a primitive payload, not
    // a type name, so a string or number never reads as a VO.
    let value = rhs.trim();
    if value.starts_with('"') || value.starts_with('\'') || value.starts_with('0') {
        return false;
    }
    if let Ok(number) = value.parse::<f64>()
        && number.is_finite()
    {
        return false;
    }
    // A tuple payload: check each element for a VO type name.
    if value.starts_with('(') {
        return value
            .trim_matches(|c| c == '(' || c == ')' || c == ',')
            .split(',')
            .filter(|el| !el.trim().is_empty())
            .any(is_vo_name);
    }
    is_vo_name(value)
}

/// True when `text` names a domain type rather than a Python primitive.
fn is_vo_name(text: &str) -> bool {
    // The Python primitives a payload may hold. Anything else is a domain
    // type, so the enum is a real typed response.
    const PY_PRIMITIVES: [&str; 8] = [
        "str", "int", "float", "bool", "bytes", "list", "dict", "None",
    ];
    let ty = text.trim().trim_end_matches(',');
    if ty.is_empty() {
        return false;
    }
    !PY_PRIMITIVES.contains(&ty)
}

/// True when `text` is a bare Python identifier.
fn is_identifier(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .next()
            .is_some_and(|c| c.is_alphabetic() || c == '_')
        && text.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Report an aggregate class that declares more than one method.
fn report_python_aggregate(
    path: &str,
    class_name: &str,
    line: usize,
    methods: &[String],
    violations: &mut Vec<LintResult>,
) {
    if methods.len() <= 1 {
        return;
    }
    violations.push(utility::build_violation(
        path,
        line as u32,
        utility::rule_code(),
        Severity::HIGH,
        &format!(
            "Aggregate class `{class_name}` declares {} methods: [{}]. \
             An aggregate is the single entry point over a feature; a second \
             method turns it into a capability seam.",
            methods.len(),
            methods.join(", ")
        ),
        "Keep exactly one `def execute(...)` on the aggregate and move the other methods into a `contract_<domain>_protocol.py` capability seam.",
        "Aggregate class declares more than one method.",
    ));
}

/// Report a multi-member enum whose members hold only primitives.
fn report_python_enum(
    path: &str,
    enum_name: &str,
    line: usize,
    members: usize,
    carries_vo: bool,
    violations: &mut Vec<LintResult>,
) {
    if members < 2 || carries_vo {
        return;
    }
    violations.push(utility::build_violation(
        path,
        line as u32,
        utility::rule_code(),
        Severity::MEDIUM,
        &format!(
            "Response enum `{enum_name}` has {members} members and every payload is a primitive. \
             A response that returns only primitives carries no domain meaning across the \
             layer boundary, so the caller cannot tell the members apart."
        ),
        "Give the enum a VO-typed payload field, or return the existing typed response VO for this seam.",
        "Response enum returns only primitive fields.",
    ));
}
