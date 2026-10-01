// PURPOSE: typescript contract role auditor — AES402 sub-checks for TypeScript/JS contract files.
//
// Handles the TypeScript contract seam: an `export interface I<Name>Protocol`
// of declarations (no function body), and an `export interface I<Name>Aggregate`
// with exactly one `execute` method.
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor by
// `file.language` and calls the trait entry point; all five AES402 sub-checks
// then run here. The language-independent pieces (contract-file recognition,
// the LintResult shape, the I/O exemption) live in
// utility_contract_role_checker.rs so the three auditors share one copy.

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::contract_role_protocol::IContractRoleProtocol;
use shared_role_rules::utility_signature_parser::{
    extract_typescript_method_signatures, typescript_signature_uses_forbidden_primitive,
};

use shared_role_rules::utility_contract_role_checker as utility;

// === Block 1: Type Definition ===

pub struct ContractTypeScriptRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl IContractRoleProtocol for ContractTypeScriptRoleAuditor {
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
        self._default_body_ts(
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
        self._aggregate_method_count_ts(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_contract_dispatch_bag(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._dispatch_bag_ts(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_contract_untyped_return(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._untyped_return_ts(
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

impl Default for ContractTypeScriptRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractTypeScriptRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    // ===============================================================
    // Contract primitive — no primitive types in a method signature
    // ===============================================================

    /// Extract `method(params): type` signatures declared inside every
    /// TypeScript interface or class block in the file and report the forbidden
    /// primitive types each one carries.
    fn _contract_primitive(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path = file.path.to_string_lossy().to_string();
        if !utility::is_contract_file(&path) || utility::is_io_exemption(&path) {
            return;
        }
        let content = &file.content;
        for (line_no, sig) in extract_typescript_method_signatures(content) {
            let forbidden = typescript_signature_uses_forbidden_primitive(&sig);
            if forbidden.is_empty() {
                continue;
            }
            let why = format!(
                "Forbidden primitive types in signature: {}. \
                 A contract signature crosses a layer boundary, so it must speak \
                 in taxonomy VOs or constants, not TS primitives.",
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
    // Default body — an interface method must not carry a function body
    // ===============================================================

    /// Report an interface/class method that carries an inline function body
    /// (`name() { ... }`). An inline body makes the contract silently return
    /// the stub value at every call site.
    fn _default_body_ts(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut in_block = false;
        let mut depth = 0i32;
        let mut block_name = String::new();

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if is_block_header(t) {
                in_block = true;
                block_name = block_name_from(t);
                depth = brace_delta(t);
                continue;
            }
            if !in_block {
                continue;
            }
            depth += brace_delta(t);
            if depth <= 0 {
                in_block = false;
                continue;
            }
            // `name(args): type { body }` is a default body — report it when
            // the opening brace sits on the same line as the signature header.
            if has_method_body(t) {
                let method = method_name_from(t);
                violations.push(utility::build_violation(
                    path,
                    i as u32 + 1,
                    utility::rule_code(),
                    Severity::HIGH,
                    &format!(
                        "`{method}` in `{block_name}` has an inline body. \
                         An interface method is a declaration: it promises the \
                         shape of a call, and an inline body hands every \
                         implementor a silent no-op."
                    ),
                    "Remove the body from the interface declaration; only `(...): ReturnType;` is allowed.",
                    "Contract interface method has an inline body.",
                ));
            }
        }
    }

    // ===============================================================
    // Aggregate method count — exactly one method on the aggregate
    // ===============================================================

    /// Report an interface whose name ends in `Aggregate` that declares
    /// anything other than a single `execute` method. More than one entry
    /// point makes the aggregate a capability seam, which belongs in
    /// `_protocol`.
    fn _aggregate_method_count_ts(
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
        let mut depth = 0i32;
        let mut block_name = String::new();
        let mut header_line = 0usize;
        let mut methods: Vec<String> = Vec::new();

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if is_block_header(t) && t.contains("Aggregate") {
                in_aggregate = true;
                block_name = block_name_from(t);
                header_line = i;
                depth = brace_delta(t);
                methods.clear();
                continue;
            }
            if !in_aggregate {
                continue;
            }
            depth += brace_delta(t);
            if depth <= 0 {
                if methods.len() > 1 {
                    report_ts_aggregate(path, &block_name, header_line + 1, &methods, violations);
                }
                in_aggregate = false;
                methods.clear();
                continue;
            }
            if is_ts_method(t) {
                methods.push(method_name_from(t));
            }
        }
        // Flush an aggregate that never closed its brace (file ends inside it).
        if in_aggregate && methods.len() > 1 {
            report_ts_aggregate(path, &block_name, header_line + 1, &methods, violations);
        }
    }

    // ===============================================================
    // Dispatch bag — an operation string is a dispatcher inside a contract
    // ===============================================================

    /// Report a contract method that takes an operation name plus a bag of
    /// loose values, i.e. `execute(op: string, ...)`. The capability then
    /// branches on `op` itself, so the contract no longer names what it does.
    fn _dispatch_bag_ts(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        for (i, l) in content.lines().enumerate() {
            let t = l.trim();
            if !t.contains(':') || !t.contains('(') {
                continue;
            }
            let Some(params) = parameter_list(t) else {
                continue;
            };
            if !carries_operation_name(&params) {
                continue;
            }
            let method = method_name_from(t);
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

    /// Report a multi-member TypeScript enum where every member holds only
    /// primitives. A response type with no VO field carries no domain meaning
    /// back across the layer boundary.
    fn _untyped_return_ts(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut in_enum = false;
        let mut depth = 0i32;
        let mut enum_name = String::new();
        let mut header_line = 0usize;
        let mut members = 0usize;
        let mut carries_vo = false;

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if let Some(name) = enum_header(t) {
                in_enum = true;
                enum_name = name;
                header_line = i;
                depth = brace_delta(t);
                members = 0;
                carries_vo = false;
                continue;
            }
            if !in_enum {
                continue;
            }
            depth += brace_delta(t);
            if depth <= 0 {
                report_ts_enum(
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
            if is_enum_member_line(t) {
                members += 1;
                if assignment_carries_vo(t) {
                    carries_vo = true;
                }
            }
        }
        // Flush the last enum — a file that ends before the closing brace (or
        // without one, as in a `.d.ts` ambient enum) still has its members.
        if in_enum {
            report_ts_enum(
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

/// True when `line` opens an interface or class block.
fn is_block_header(line: &str) -> bool {
    line.starts_with("export interface ")
        || line.starts_with("interface ")
        || line.starts_with("export class ")
        || line.starts_with("class ")
}

/// The interface/class name from a `interface Name ... {` line.
fn block_name_from(line: &str) -> String {
    let rest = line
        .strip_prefix("export interface ")
        .or_else(|| line.strip_prefix("interface "))
        .or_else(|| line.strip_prefix("export class "))
        .or_else(|| line.strip_prefix("class "))
        .unwrap_or(line);
    rest.split([' ', '<']).next().unwrap_or("").to_string()
}

/// The method name from a `name(args): type;` / `name(args): type { body }` line.
fn method_name_from(line: &str) -> String {
    // Skip access modifiers.
    let trimmed = line
        .strip_prefix("public ")
        .or_else(|| line.strip_prefix("private "))
        .or_else(|| line.strip_prefix("protected "))
        .or_else(|| line.strip_prefix("static "))
        .or_else(|| line.strip_prefix("readonly "))
        .unwrap_or(line);
    trimmed.split('(').next().unwrap_or("").trim().to_string()
}

/// True when the method header opens its own body on the same line.
///
/// The opening brace must come after the parameter list's closing `)`, so a
/// nested object-typed parameter does not read as a body.
fn has_method_body(line: &str) -> bool {
    if line.starts_with("//") || line.starts_with("*") || line.starts_with("/*") {
        return false;
    }
    let Some(close) = line.rfind(')') else {
        return false;
    };
    line[..close].contains('(') && line[close + 1..].contains('{')
}

/// True when `line` is a TS method declaration inside an interface/class.
fn is_ts_method(line: &str) -> bool {
    !line.starts_with("import ")
        && !line.starts_with("const ")
        && !line.starts_with("let ")
        && !line.starts_with("var ")
        && !line.starts_with("/*")
        && line.contains('(')
        && (line.contains(':') || line.ends_with('{'))
}

/// Extract the `(args)` portion of a signature line.
fn parameter_list(line: &str) -> Option<String> {
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
        ) && (ty.starts_with("string") || ty.starts_with("str"))
    })
}

/// The enum name from an `enum Name {` header line.
fn enum_header(line: &str) -> Option<String> {
    let rest = line
        .strip_prefix("export enum ")
        .or_else(|| line.strip_prefix("enum "))?;
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() || !line.contains('{') {
        return None;
    }
    Some(name)
}

/// True when a line inside an enum block is a member definition.
fn is_enum_member_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty()
        && !trimmed.starts_with("//")
        && !trimmed.starts_with("/*")
        && !trimmed.starts_with('*')
        && !trimmed.starts_with("}")
}

/// True when an enum member's initializer names a non-primitive type.
fn assignment_carries_vo(line: &str) -> bool {
    let trimmed = line.trim();
    let Some((lhs, rhs)) = trimmed.split_once('=') else {
        return false;
    };
    // `NAME: PayloadType = (...)` — the annotation names the payload type.
    if let Some((_, ann)) = lhs.trim().split_once(':') {
        return is_vo_name(ann.trim());
    }
    // `NAME = <value>,` — the trailing comma is separator syntax, so strip it
    // before deciding whether the value is a literal or a type name.
    let value = rhs.trim().trim_end_matches(',').trim();
    if value.starts_with('"') || value.starts_with('\'') || value.starts_with('`') {
        return false;
    }
    if value.parse::<f64>().is_ok() {
        return false;
    }
    is_vo_name(value)
}

/// True when `text` names a domain type rather than a TS primitive.
fn is_vo_name(text: &str) -> bool {
    // The TS primitives a payload may hold. Anything else is a domain type,
    // so the enum is a real typed response.
    const TS_PRIMITIVES: [&str; 6] = ["string", "number", "boolean", "any", "unknown", "undefined"];
    let ty = text.trim().trim_end_matches(',');
    if ty.is_empty() {
        return false;
    }
    !TS_PRIMITIVES.contains(&ty)
}

/// The brace delta of a line: `+1` per `{`, `-1` per `}`.
fn brace_delta(line: &str) -> i32 {
    let (opens, closes) = (line.matches('{').count(), line.matches('}').count());
    i32::try_from(opens).unwrap_or(0) - i32::try_from(closes).unwrap_or(0)
}

/// Report an aggregate interface that declares more than one method.
fn report_ts_aggregate(
    path: &str,
    block_name: &str,
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
            "Aggregate interface `{block_name}` declares {} methods: [{}]. \
             An aggregate is the single entry point over a feature; a second \
             method turns it into a capability seam.",
            methods.len(),
            methods.join(", ")
        ),
        "Keep exactly one `execute(...)` on the aggregate and move the other methods into a `contract_<domain>_protocol.ts` capability seam.",
        "Aggregate interface declares more than one method.",
    ));
}

/// Report a multi-member enum whose members hold only primitives.
fn report_ts_enum(
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
