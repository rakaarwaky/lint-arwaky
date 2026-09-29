// PURPOSE: rust contract role auditor — AES402 sub-checks for Rust contract files.
//
// Handles the Rust contract seam: a `pub trait I<Name>Protocol` of declarations
// (`fn ...;`, never a body) and a `pub trait I<Name>Aggregate` of exactly one
// `fn execute`.
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor by
// `file.language` and calls the trait entry point; all five AES402 sub-checks
// then run here. The language-independent pieces (contract-file recognition,
// the LintResult shape, the I/O exemption) live in
// utility_contract_role_checker.rs so the three auditors share one copy.

use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_signature_parser::{
    extract_trait_method_signatures, signature_uses_forbidden_primitive,
};
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::contract_role_protocol::IContractRoleProtocol;

use shared::role_rules::utility_contract_role_checker as utility;

// === Block 1: Type Definition ===

pub struct ContractRustRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl IContractRoleProtocol for ContractRustRoleAuditor {
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
        self._default_body_rust(
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
        self._aggregate_method_count_rust(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_contract_dispatch_bag(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._dispatch_bag_rust(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_contract_untyped_return(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._untyped_return_rust(
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

impl Default for ContractRustRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractRustRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    // ===============================================================
    // Contract primitive — no primitive types in a trait signature
    // ===============================================================

    /// Extract `fn ...;` signatures declared inside every Rust trait in the
    /// file and report the forbidden primitive types each one carries.
    fn _contract_primitive(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path = file.path.to_string_lossy().to_string();
        if !utility::is_contract_file(&path) || utility::is_io_exemption(&path) {
            return;
        }
        let content = &file.content;
        for (line_no, sig) in extract_trait_method_signatures(content) {
            let forbidden = signature_uses_forbidden_primitive(&sig);
            if forbidden.is_empty() {
                continue;
            }
            let why = format!(
                "Forbidden primitive types in signature: {}. \
                 A contract signature crosses a layer boundary, so it must speak \
                 in taxonomy VOs or constants, not Rust primitives.",
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
    // Default body — a trait method must be a declaration, not a body
    // ===============================================================

    /// Report every `fn` declared inside a trait that carries a `{ ... }` body
    /// on the same line. A body inside a contract trait makes the method
    /// silently unimplemented at every call site.
    fn _default_body_rust(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut in_trait = false;
        let mut trait_depth = 0i32;
        let mut trait_name = String::new();

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if let Some(rest) = trait_header(t) {
                in_trait = true;
                trait_name = rest;
                trait_depth = brace_delta(t);
                continue;
            }
            if !in_trait {
                continue;
            }
            // A `fn` line that opens a brace carries a default body. Checked
            // before the depth update so the body brace is not mistaken for
            // the trait's own closing brace.
            if t.starts_with("fn ") && t.contains('{') {
                let method = method_name(t);
                violations.push(utility::build_violation(
                    path,
                    i as u32 + 1,
                    utility::rule_code(),
                    Severity::HIGH,
                    &format!(
                        "`{method}` in trait `{trait_name}` has a body. A contract method is a \
                         declaration: it promises the shape of a call, and a default body hands \
                         every implementor a silent no-op."
                    ),
                    "Delete the body and end the signature with `;` so each implementor supplies the behaviour.",
                    "Contract trait method has a default body.",
                ));
            }
            trait_depth += brace_delta(t);
            if trait_depth <= 0 {
                in_trait = false;
            }
        }
    }

    // ===============================================================
    // Aggregate method count — exactly one method on the aggregate
    // ===============================================================

    /// Report a trait whose name ends in `Aggregate` that declares anything
    /// other than a single `execute` method. More than one entry point makes
    /// the aggregate a capability seam, which belongs in `_protocol`.
    fn _aggregate_method_count_rust(
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
        let mut trait_name = String::new();
        let mut header_line = 0usize;
        let mut methods: Vec<(usize, String)> = Vec::new();

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if let Some(rest) = trait_header(t)
                && rest.ends_with("Aggregate")
            {
                in_aggregate = true;
                trait_name = rest;
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
                    let names: Vec<&str> = methods.iter().map(|(_, n)| n.as_str()).collect();
                    violations.push(utility::build_violation(
                        path,
                        (header_line + 1) as u32,
                        utility::rule_code(),
                        Severity::HIGH,
                        &format!(
                            "Aggregate trait `{trait_name}` declares {} methods: [{}]. \
                             An aggregate is the single entry point over a feature; a second \
                             method turns it into a capability seam.",
                            methods.len(),
                            names.join(", ")
                        ),
                        "Keep exactly one `fn execute(...)` on the aggregate and move the other methods into a `contract_<domain>_protocol.rs` capability seam.",
                        "Aggregate trait declares more than one method.",
                    ));
                }
                in_aggregate = false;
                depth = 0;
                methods.clear();
                continue;
            }
            if t.starts_with("fn ") {
                methods.push((i + 1, method_name(t)));
            }
        }
    }

    // ===============================================================
    // Dispatch bag — an operation string is a dispatcher inside a contract
    // ===============================================================

    /// Report a trait method that takes an operation name plus a bag of loose
    /// values, i.e. `fn execute(op: &str, ...)`. The capability then branches
    /// on `op` itself, so the contract no longer names what it does.
    fn _dispatch_bag_rust(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        for (i, l) in content.lines().enumerate() {
            let t = l.trim();
            if !t.starts_with("fn ") {
                continue;
            }
            let Some(params) = parameter_list(t) else {
                continue;
            };
            if !carries_operation_name(&params) {
                continue;
            }
            let method = method_name(t);
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
                "Split the operation into its own trait method with a VO-typed parameter, so the signature states the intent.",
                "Contract seam takes an operation-name dispatch parameter.",
            ));
        }
    }

    // ===============================================================
    // Untyped return — a response enum of bare primitives
    // ===============================================================

    /// Report an `enum` in the contract file where every variant holds only
    /// primitives. A response type with no VO field carries no domain meaning
    /// back across the layer boundary.
    fn _untyped_return_rust(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        if !utility::is_contract_file(path) {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut in_enum = false;
        let mut depth = 0i32;
        let mut enum_name = String::new();
        let mut header_line = 0usize;
        let mut variants = 0usize;
        let mut carries_vo = false;

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if let Some(rest) = enum_header(t) {
                in_enum = true;
                enum_name = rest;
                header_line = i;
                depth = brace_delta(t);
                variants = 0;
                carries_vo = false;
                continue;
            }
            if !in_enum {
                continue;
            }
            depth += brace_delta(t);
            if depth <= 0 {
                report_untyped_enum(
                    path,
                    &enum_name,
                    header_line + 1,
                    variants,
                    carries_vo,
                    violations,
                );
                in_enum = false;
                depth = 0;
                continue;
            }
            // A unit variant is `Name,` / `Name,` on its own line.
            let is_variant = !t.is_empty()
                && !t.starts_with("//")
                && !t.starts_with("///")
                && (t.ends_with(',') || t.ends_with('{'));
            if is_variant {
                variants += 1;
                if field_carries_vo(t) {
                    carries_vo = true;
                }
            }
        }
    }
}

// === Free Functions ===

/// The trait name from a `pub trait Name ... {` header line.
fn trait_header(line: &str) -> Option<String> {
    let rest = line
        .strip_prefix("pub trait ")
        .or_else(|| line.strip_prefix("trait "))?;
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() || !line.contains('{') {
        return None;
    }
    Some(name)
}

/// The enum name from a `pub enum Name ... {` header line.
fn enum_header(line: &str) -> Option<String> {
    let rest = line
        .strip_prefix("pub enum ")
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

/// The name of a `fn name(...)` declaration.
fn method_name(line: &str) -> String {
    line.strip_prefix("pub fn ")
        .or_else(|| line.strip_prefix("fn "))
        .unwrap_or("")
        .split(['(', '<'])
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

/// The text between the first `(` and its balancing `)`.
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
        let ty = p.split(':').nth(1).unwrap_or("").trim();
        matches!(
            lhs,
            "op" | "operation" | "command" | "action" | "kind" | "mode"
        ) && (ty.contains("str") || ty.contains("String"))
    })
}

/// The brace delta of a line: `+1` per `{`, `-1` per `}`.
fn brace_delta(line: &str) -> i32 {
    let (opens, closes) = (line.matches('{').count(), line.matches('}').count());
    i32::try_from(opens).unwrap_or(0) - i32::try_from(closes).unwrap_or(0)
}

/// True when a variant declares at least one non-primitive field type.
///
/// A VO-typed field means the enum is a real domain response, so the
/// untyped-return rule does not apply. Empty tuples and unit variants carry
/// nothing and leave the question to the primitive sub-check.
fn field_carries_vo(variant: &str) -> bool {
    // The Rust primitives a field type may hold. Anything else on a response
    // variant is a domain type, so the enum is a real typed response.
    const RUST_PRIMITIVES: [&str; 14] = [
        "String", "str", "bool", "char", "usize", "isize", "f32", "f64", "i8", "i16", "i32", "i64",
        "u8", "u16",
    ];
    let Some(open) = variant.find('(') else {
        return false;
    };
    let Some(close) = variant.rfind(')') else {
        return false;
    };
    variant[open + 1..close]
        .split(',')
        .filter_map(|f| f.split(':').nth(1))
        .map(str::trim)
        .any(|ty| !ty.is_empty() && !RUST_PRIMITIVES.contains(&ty))
}

/// Report a multi-variant enum whose variants hold only primitives.
fn report_untyped_enum(
    path: &str,
    enum_name: &str,
    line: usize,
    variants: usize,
    carries_vo: bool,
    violations: &mut Vec<LintResult>,
) {
    if variants < 2 || carries_vo {
        return;
    }
    violations.push(utility::build_violation(
        path,
        line as u32,
        utility::rule_code(),
        Severity::MEDIUM,
        &format!(
            "Response enum `{enum_name}` has {variants} variants and every field is a primitive. \
             A response that returns only primitives carries no domain meaning across the \
             layer boundary, so the caller cannot tell the variants apart."
        ),
        "Give the enum a VO-typed payload field, or return the existing typed response VO for this seam.",
        "Response enum returns only primitive fields.",
    ));
}
