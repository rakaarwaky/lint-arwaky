Yes — I reviewed `role-rules v1.10.106` as a QA + developer pass.

## Summary

The module already has the right architecture, but several existing checks are either:

1. **Functionally incorrect** — producing false positives or false negatives.
2. **Partially wired** — some checkers are invoked through no-op trait methods.
3. **Performance-suboptimal** — repeated full-content scans and duplicate I/O.

Below are the concrete issues, why they matter, and the fixed code.
No new features are introduced — these are corrections and optimizations of existing behavior.

---

# 1. Functional Bugs

---

## 1.1 `surface` vs `surfaces` layer mismatch disables AES406 checks

### Location

`crates/shared/src/common/utility_layer_detector.rs`

### Problem

`detect_layer_from_prefix()` returns:

```rust
"surface"
```

for `surface_*.rs` files.

But the rest of the system expects the layer name:

```rust
"surfaces"
```

For example:

```rust
layer_surfaces().value == "surfaces"
```

So in:

```rust
crates/role-rules/src/capabilities_surface_role_auditor.rs
```

this condition never becomes true for normal surface files:

```rust
let is_surface = layer_vo == layer_surfaces().value
    || layer_vo.starts_with(&format!("{}(", layer_surfaces().value));
```

Result:

- AES406 `no_domain_logic` checks are silently skipped.
- Surface files can violate role rules without being reported.

### Fix

Normalize the prefix detector to the canonical layer names used everywhere else.

```rust
// crates/shared/src/common/utility_layer_detector.rs

pub fn detect_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("capability_", "capabilities"),
        ("utility_", "utility"),
        ("surface_", "surfaces"),
        ("surfaces_", "surfaces"),
        ("agent_", "agent"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }

    None
}
```

### Why this is a bug fix

This does not add a new layer or rule. It makes existing AES406 checks actually run.

---

## 1.2 Orchestrator misclassifies `router` surfaces

### Location

`crates/role-rules/src/agent_role_orchestrator.rs`

### Problem

Architecture defines smart surfaces as including:

- `command`
- `controller`
- `page`
- `router`

But the orchestrator classifies `router` as a utility surface:

```rust
let is_utility = filename.contains("_hook")
    || filename.contains("_store")
    || filename.contains("_action")
    || filename.contains("_screen")
    || filename.contains("_router");
```

This is inconsistent with `ARCHITECTURE.md`.

### Fix

```rust
// crates/role-rules/src/agent_role_orchestrator.rs

let is_smart = filename.contains("_command")
    || filename.contains("_controller")
    || filename.contains("_page")
    || filename.contains("_router")
    || filename.contains("_entry");

let is_utility = filename.contains("_hook")
    || filename.contains("_store")
    || filename.contains("_action")
    || filename.contains("_screen");
```

### Why this matters

Smart surfaces are allowed to initiate behavior through aggregates.
Utility surfaces are not equivalent. Misclassification can suppress or apply the wrong checks.

---

## 1.3 Passive surface checks are not executed from the orchestrator

### Location

`crates/role-rules/src/capabilities_surface_role_auditor.rs`

### Problem

The orchestrator calls:

```rust
checker.check_passive_surface(&source_vo, violations);
```

But the implementation is a no-op:

```rust
fn check_passive_surface(
    &self,
    _source: &SourceContentVO,
    _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
}
```

The real passive check exists in:

```rust
check_surface_hierarchy()
```

but that method is not called by the orchestrator.

Result:

- AES406 passive-surface violations are missed during normal role auditing.
- The trait method exists but does nothing.

### Fix

Add a source-based passive checker and wire the trait method to it.

Add this helper inside `impl SurfaceRoleChecker`:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

fn _check_passive_source(
    &self,
    source: &SourceContentVO,
    results: &mut LintResultList,
) {
    let f = &source.file_path;
    let f_str = f.to_string();

    let basename = std::path::Path::new(&f_str)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    // Smart surfaces are allowed to contain orchestration/dispatch logic.
    if basename.ends_with("_command")
        || basename.ends_with("_controller")
        || basename.ends_with("_page")
        || basename.ends_with("_entry")
        || basename.ends_with("_router")
    {
        return;
    }

    let content = source.content.value();
    let lines: Vec<&str> = content.lines().collect();
    let mut violations: Vec<String> = Vec::new();

    let li = detect_language_info_from_source(source);

    match li.lang {
        DetLang::Rust => self._check_rust_passive(f, &lines, &mut violations),
        DetLang::JavaScript | DetLang::TypeScript => {
            self._check_javascript_passive(f, &lines, &mut violations)
        }
        _ => self._check_python_passive(f, &lines, &mut violations),
    }

    if !violations.is_empty() {
        self._report_aes0306(f, violations, results);
    }
}
```

Then implement the previously no-op trait method:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

fn check_passive_surface(
    &self,
    source: &SourceContentVO,
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let mut results = LintResultList::default();
    self._check_passive_source(source, &mut results);
    violations.extend(results.values);
}
```

Also update the existing `_check_passive()` smart-surface exemption to include router:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

if basename.ends_with("_command")
    || basename.ends_with("_controller")
    || basename.ends_with("_page")
    || basename.ends_with("_entry")
    || basename.ends_with("_router")
{
    return;
}
```

And update `check_surface_roles()` smart-surface detection:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

let is_smart = basename.ends_with("_command")
    || basename.ends_with("_controller")
    || basename.ends_with("_page")
    || basename.ends_with("_entry")
    || basename.ends_with("_router");
```

### Why this is not a new feature

The passive-surface rule already exists. This simply makes the existing orchestrator path execute it.

---

## 1.4 AES405 `any` detection has false positives

### Location

`crates/role-rules/src/capabilities_agent_role_auditor.rs`

### Problem

Current logic uses substring matching:

```rust
if t.contains(": any")
    || t.contains(": Any")
    || t.contains("-> any")
    || t.contains("-> Any")
    || t.contains("Any<")
    || t.contains("Any[")
    || t.contains("any[")
```

This can incorrectly flag:

```rust
let x: anyhow::Error = ...;
```

or:

```ts
let company: Company[] = [];
```

or comments containing `: any`.

### Fix

Use a token-aware regex and strip comments before matching.

Add imports:

```rust
// crates/role-rules/src/capabilities_agent_role_auditor.rs

use once_cell::sync::Lazy;
use regex::Regex;
```

Add a static regex:

```rust
// crates/role-rules/src/capabilities_agent_role_auditor.rs

static ANY_TYPE_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"(?::|->)\s*(?:any|Any)\b|\bAny\s*[<\[]|\bany\s*\[").ok()
});

fn code_without_comment(line: &str) -> &str {
    if let Some(pos) = line.find("//") {
        return &line[..pos];
    }

    if let Some(pos) = line.find('#') {
        return &line[..pos];
    }

    line
}
```

Replace `check_any_type_annotation`:

```rust
// crates/role-rules/src/capabilities_agent_role_auditor.rs

fn check_any_type_annotation(
    &self,
    source: &SourceContentVO,
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let content = source.content.value();
    let file = source.file_path.value();

    for (i, line) in content.lines().enumerate() {
        let code = code_without_comment(line);

        let has_any = ANY_TYPE_RE
            .as_ref()
            .map(|re| re.is_match(code))
            .unwrap_or(false);

        if has_any {
            violations.push(LintResult::new_arch(
                file,
                i + 1,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AnyType { reason: None }.to_string(),
            ));
        }
    }
}
```

### Result

Now these are flagged:

```ts
let x: any;
let y: Any;
let z: any[];
let a: Any<T>;
```

But these are not:

```ts
let x: anyhow::Error;
let y: Company[];
```

---

## 1.5 AES403 Rust capability routing accepts inherent impls incorrectly

### Location

`crates/role-rules/src/capabilities_capabilities_role_auditor.rs`

### Problem

Current logic accepts:

```rust
impl Foo { ... }
```

as proof that `Foo` implements a protocol.

But AES403 requires a capability to implement a contract protocol, for example:

```rust
impl IFoo for Foo { ... }
```

Current code also:

- Uses substring matching, causing false positives.
- Suppresses all checks when there are more than 3 structs.
- Reports line `0`, reducing precision.

### Fix

Replace `_check_rust_routing()` with a single-pass, struct-aware implementation.

```rust
// crates/role-rules/src/capabilities_capabilities_role_auditor.rs

fn _check_rust_routing(
    &self,
    file: &str,
    content: &str,
    violations: &mut Vec<LintResult>,
) {
    let has_proto_import = content.contains("use ")
        && (content.contains("_protocol::") || content.contains("_port::"));

    if !has_proto_import {
        violations.push(LintResult::new_arch(
            file,
            1,
            "AES403",
            Severity::MEDIUM,
            AesRoleViolation::CapabilityNoProtocol { reason: None },
        ));
        return;
    }

    let mut structs: Vec<(String, usize)> = Vec::new();
    let mut implemented: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    let mut in_cfg_test = false;
    let mut cfg_base_depth: i32 = 0;
    let mut brace_depth: i32 = 0;

    for (idx, raw) in content.lines().enumerate() {
        let t = raw.trim();

        let opens = t.matches('{').count() as i32;
        let closes = t.matches('}').count() as i32;

        if in_cfg_test {
            brace_depth += opens - closes;

            if brace_depth <= cfg_base_depth {
                in_cfg_test = false;
            }

            continue;
        }

        if t.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            cfg_base_depth = brace_depth;
            brace_depth += opens - closes;
            continue;
        }

        brace_depth += opens - closes;

        // Collect struct definitions.
        if t.starts_with("pub struct ") || t.starts_with("struct ") {
            let words: Vec<&str> = t.split_whitespace().collect();

            if let Some(pos) = words.iter().position(|w| *w == "struct") {
                if let Some(raw_name) = words.get(pos + 1) {
                    let name: String = raw_name
                        .trim_end_matches(';')
                        .trim_end_matches('{')
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect();

                    if !name.is_empty() && !name.starts_with('_') {
                        structs.push((name, idx + 1));
                    }
                }
            }
        }

        // Collect trait implementations:
        //
        //     impl Trait for Struct
        //
        // Inherent impls are intentionally ignored:
        //
        //     impl Struct
        //
        if (t.starts_with("impl ") || t.starts_with("impl<"))
            && t.contains(" for ")
        {
            let mut parts = t.splitn(2, " for ");

            let trait_part = parts.next().unwrap_or_default();
            let struct_part = parts.next().unwrap_or_default().trim_start();

            let struct_name: String = struct_part
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();

            let trait_token = trait_part
                .split_whitespace()
                .last()
                .unwrap_or_default()
                .split("::")
                .last()
                .unwrap_or_default();

            let trait_name: String = trait_token
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();

            // Existing codebase convention uses protocol traits like:
            //
            //     IAgentRoleChecker
            //     IContractRoleChecker
            //
            let is_protocol_like = trait_name.starts_with('I')
                && trait_name
                    .chars()
                    .nth(1)
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false);

            if !struct_name.is_empty() && is_protocol_like {
                implemented.insert(struct_name);
            }
        }
    }

    for (struct_name, line) in structs {
        if !implemented.contains(&struct_name) {
            violations.push(LintResult::new_arch(
                file,
                line,
                "AES403",
                Severity::MEDIUM,
                AesRoleViolation::CapabilityRouting {
                    struct_name: SymbolName::new(struct_name),
                    reason: None,
                },
            ));
        }
    }
}
```

### What this fixes

- Inherent impls no longer satisfy AES403.
- Struct names are matched as whole identifiers.
- The arbitrary `structs.len() <= 3` suppression is removed.
- Violations now point to the struct definition line.

---

## 1.6 AES403 contract should not use raw `&str`

### Location

`crates/shared/src/role-rules/contract_capabilities_role_protocol.rs`

### Problem

The protocol uses a raw primitive:

```rust
layer: &str
```

This violates AES402’s contract primitive restriction.

### Fix

Use a taxonomy value object.

```rust
// crates/shared/src/role-rules/contract_capabilities_role_protocol.rs

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ICapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &LayerNameVO,
        violations: &mut Vec<LintResult>,
    );
}
```

Update the implementation:

```rust
// crates/role-rules/src/capabilities_capabilities_role_auditor.rs

use shared::common::taxonomy_layer_vo::LayerNameVO;

impl ICapabilitiesRoleChecker for CapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &LayerNameVO,
        violations: &mut Vec<LintResult>,
    ) {
        let layer = layer.value();

        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }

        let file = source.file_path.value();
        let content = source.content.value();
        let li = detect_language_info_from_source(source);

        if li.is_rs {
            self._check_rust_routing(file, content, violations);
        } else if li.is_py {
            self._check_python_routing(file, content, violations);
        } else if li.is_js {
            self._check_js_routing(file, content, violations);
        }
    }
}
```

Update the orchestrator call:

```rust
// crates/role-rules/src/agent_role_orchestrator.rs

use shared::role_rules::taxonomy_layer_names_vo::layer_capabilities;

"capabilities" | "capability" => {
    let checker = self.aggregate.capabilities();
    checker.check_capability_routing(
        &source_vo,
        &layer_capabilities(),
        violations,
    );
}
```

---

## 1.7 AES405 contract should not use raw `usize`

### Location

`crates/shared/src/role-rules/contract_agent_role_protocol.rs`

### Problem

```rust
max_lines: usize
```

is a raw primitive in a contract protocol.

### Fix

Use `Count`.

```rust
// crates/shared/src/role-rules/contract_agent_role_protocol.rs

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IAgentRoleChecker: Send + Sync {
    fn check_container(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    );

    fn check_orchestrator(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    );

    fn check_lifecycle(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    );

    fn check_file_size_limit(
        &self,
        source: &SourceContentVO,
        max_lines: Count,
        violations: &mut Vec<LintResult>,
    );

    fn check_any_type_annotation(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    );
}
```

Update implementation:

```rust
// crates/role-rules/src/capabilities_agent_role_auditor.rs

use shared::common::taxonomy_common_vo::Count;

fn check_file_size_limit(
    &self,
    source: &SourceContentVO,
    max_lines: Count,
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let content = source.content.value();
    let file = source.file_path.value();

    let max_lines_usize = max_lines.value().max(0) as usize;

    if content.lines().count() > max_lines_usize {
        violations.push(LintResult::new_arch(
            file,
            0,
            "AES405",
            Severity::HIGH,
            AesRoleViolation::AgentFileSizeLimit {
                max_lines: max_lines_usize,
            }
            .to_string(),
        ));
    }
}
```

Update orchestrator call:

```rust
// crates/role-rules/src/agent_role_orchestrator.rs

use shared::common::taxonomy_common_vo::Count;

"agent" => {
    let checker = self.aggregate.agent();

    checker.check_file_size_limit(
        &source_vo,
        Count::new(max_lines as i64),
        violations,
    );

    checker.check_any_type_annotation(&source_vo, violations);

    if filename.contains("_container") {
        checker.check_container(&source_vo, violations);
    } else if filename.contains("_orchestrator") {
        checker.check_orchestrator(&source_vo, violations);
    } else if filename.contains("_lifecycle") {
        checker.check_lifecycle(&source_vo, violations);
    }
}
```

---

## 1.8 AES402 primitive detection has substring false positives

### Location

`crates/shared/src/common/utility_signature_parser.rs`

### Problem

Python and TypeScript checks use substring matching:

```rust
lower.contains(": str")
```

This can incorrectly match:

```python
def foo(x: string_like): ...
```

or:

```ts
foo(x: stringify): void;
```

Also, Python detection requires `->`, so parameter-only primitive annotations can be missed.

### Fix

Add token-aware helpers.

```rust
// crates/shared/src/common/utility_signature_parser.rs

fn starts_with_type_token(s: &str, token: &str) -> bool {
    match s.strip_prefix(token) {
        Some(rest) => rest
            .chars()
            .next()
            .map_or(true, |c| !c.is_ascii_alphanumeric() && c != '_'),
        None => false,
    }
}

fn contains_colon_type_token(s: &str, token: &str) -> bool {
    let needle = format!(":{}", token);
    let mut from = 0;

    while let Some(pos) = s[from..].find(&needle) {
        let abs = from + pos;
        let after = abs + needle.len();

        let boundary_ok = after >= s.len()
            || s[after..]
                .chars()
                .next()
                .map_or(true, |c| !c.is_ascii_alphanumeric() && c != '_');

        if boundary_ok {
            return true;
        }

        from = abs + 1;
    }

    false
}
```

Replace Python forbidden-primitive checker:

```rust
// crates/shared/src/common/utility_signature_parser.rs

pub fn python_signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let lower = sig.to_lowercase();

    let compact = lower
        .replace(": ", ":")
        .replace(" :", ":")
        .replace("-> ", "->")
        .replace(" ->", "->");

    let mut forbidden: Vec<&'static str> = Vec::new();

    let tokens = [
        "str", "int", "float", "bool", "list", "dict",
    ];

    for token in tokens {
        if contains_colon_type_token(&compact, token) {
            forbidden.push(token);
        }
    }

    if let Some(idx) = compact.find("->") {
        let ret = compact[idx + 2..].trim();

        for token in tokens {
            if starts_with_type_token(ret, token) {
                forbidden.push(token);
            }
        }
    }

    forbidden.sort();
    forbidden.dedup();
    forbidden
}
```

Replace TypeScript forbidden-primitive checker:

```rust
// crates/shared/src/common/utility_signature_parser.rs

pub fn typescript_signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let lower = sig.to_lowercase();

    let compact = lower
        .replace(": ", ":")
        .replace(" :", ":");

    let mut forbidden: Vec<&'static str> = Vec::new();

    let tokens = [
        "string",
        "number",
        "boolean",
        "any",
    ];

    for token in tokens {
        if contains_colon_type_token(&compact, token) {
            forbidden.push(token);
        }
    }

    forbidden.sort();
    forbidden.dedup();
    forbidden
}
```

Update Python signature extraction so parameter-only annotations are not skipped:

```rust
// crates/shared/src/common/utility_signature_parser.rs

pub fn extract_python_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();

    let mut in_class = false;
    let mut class_indent = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw.trim();

        if trimmed.starts_with("class ") && trimmed.contains(':') {
            in_class = true;
            class_indent = raw.len() - raw.trim_start().len();
            continue;
        }

        if !in_class {
            continue;
        }

        let current_indent = raw.len() - raw.trim_start().len();

        if current_indent <= class_indent && !trimmed.is_empty() {
            in_class = false;
            continue;
        }

        if trimmed.starts_with("def ")
            && (trimmed.contains(':') || trimmed.contains("->"))
        {
            results.push((line_no, raw.to_string()));
        }
    }

    results
}
```

Update TypeScript extraction to use the corrected checker:

```rust
// crates/shared/src/common/utility_signature_parser.rs

pub fn extract_typescript_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut in_block = false;
    let mut brace_depth = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw.trim();

        if (trimmed.starts_with("export interface ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("export class ")
            || trimmed.starts_with("class "))
            && trimmed.contains('{')
        {
            in_block = true;
            brace_depth = trimmed.matches('{').count() as i32
                - trimmed.matches('}').count() as i32;

            if brace_depth == 0 {
                if let Some(open) = trimmed.find('{') {
                    if let Some(close) = trimmed.rfind('}') {
                        let inner = &trimmed[open + 1..close];

                        if inner.contains('(') && inner.contains(':') {
                            if !typescript_signature_uses_forbidden_primitive(inner)
                                .is_empty()
                            {
                                results.push((line_no, raw.to_string()));
                            }
                        }
                    }
                }

                in_block = false;
            }

            continue;
        }

        if in_block {
            brace_depth += trimmed.matches('{').count() as i32
                - trimmed.matches('}').count() as i32;

            if brace_depth <= 0 {
                in_block = false;
                brace_depth = 0;
                continue;
            }

            if trimmed.contains('(') && trimmed.contains(':') {
                if !typescript_signature_uses_forbidden_primitive(trimmed).is_empty() {
                    results.push((line_no, raw.to_string()));
                }
            }
        }
    }

    results
}
```

Also add missing Rust primitives to `signature_uses_forbidden_primitive`:

```rust
// crates/shared/src/common/utility_signature_parser.rs

if regex_lite_match_whole_token(&combined, "bool") {
    forbidden.push("bool");
}

if regex_lite_match_whole_token(&combined, "str") {
    forbidden.push("str");
}
```

---

## 1.9 `extract_trait_method_signatures()` has invalid boolean logic

### Location

`crates/shared/src/common/utility_signature_parser.rs`

### Problem

Current code contains this condition:

```rust
line.contains(')').ge(&line.contains('('))
```

This is semantically wrong and makes trait detection fragile.

### Fix

Replace with a simpler, correct trait-header detector.

```rust
// crates/shared/src/common/utility_signature_parser.rs

pub fn extract_trait_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();

    let mut in_trait_depth: i32 = 0;
    let mut brace_depth: i32 = 0;
    let mut pending_trait = false;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.trim();

        if in_trait_depth == 0 && !pending_trait {
            if line.starts_with("pub trait ") || line.starts_with("trait ") {
                if line.contains('{') {
                    in_trait_depth = 1;
                    brace_depth = line.matches('{').count() as i32
                        - line.matches('}').count() as i32;

                    if brace_depth <= 0 {
                        in_trait_depth = 0;
                        brace_depth = 0;
                    }
                } else {
                    pending_trait = true;
                }
            }

            continue;
        }

        if pending_trait {
            if line.contains('{') {
                pending_trait = false;
                in_trait_depth = 1;
                brace_depth = line.matches('{').count() as i32
                    - line.matches('}').count() as i32;

                if brace_depth <= 0 {
                    in_trait_depth = 0;
                    brace_depth = 0;
                }
            }

            continue;
        }

        if line.starts_with("fn ") && line.contains(';') {
            results.push((line_no, raw.to_string()));
        }

        brace_depth += line.matches('{').count() as i32
            - line.matches('}').count() as i32;

        if brace_depth <= 0 {
            in_trait_depth = 0;
            brace_depth = 0;
        }
    }

    results
}
```

---

## 1.10 Forbidden inheritance check parses only the first grouped import

### Location

`crates/role-rules/src/capabilities_contract_role_auditor.rs`

### Problem

For:

```rust
use foo::bar::{Baz, Qux};
```

current logic extracts only:

```rust
Baz
```

and misses:

```rust
Qux
```

It also reports line `0`.

### Fix

Add an import-symbol parser:

```rust
// crates/role-rules/src/capabilities_contract_role_auditor.rs

fn imported_symbol_names(import_line: &str) -> Vec<String> {
    let t = import_line.trim().trim_end_matches(';').trim();

    // Rust grouped import:
    //
    //     use foo::bar::{Baz, Qux};
    //
    if let Some(open) = t.find('{') {
        let close = t.rfind('}').unwrap_or(t.len());

        if close > open {
            let inner = &t[open + 1..close];

            return inner
                .split(',')
                .filter_map(|item| {
                    let item = item.trim();

                    if item.is_empty() {
                        return None;
                    }

                    let name = if let Some(pos) = item.find(" as ") {
                        item[..pos].trim()
                    } else {
                        item
                    };

                    let name = name
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .trim();

                    if name.is_empty() {
                        None
                    } else {
                        Some(name.to_string())
                    }
                })
                .collect();
        }
    }

    // Python:
    //
    //     from foo import Bar, Baz
    //
    if let Some(pos) = t.find(" import ") {
        let names = &t[pos + 8..];

        return names
            .split(',')
            .filter_map(|name| {
                let name = name.trim();

                let name = if let Some(as_pos) = name.find(" as ") {
                    name[..as_pos].trim()
                } else {
                    name
                };

                if name.is_empty() {
                    None
                } else {
                    Some(name.to_string())
                }
            })
            .collect();
    }

    // Rust single import:
    //
    //     use foo::bar::Baz;
    //
    let last = t.split("::").last().unwrap_or_default().trim();

    let last = if let Some(pos) = last.find(" as ") {
        last[..pos].trim()
    } else {
        last
    };

    let last = last
        .trim_start_matches('{')
        .trim_end_matches('}')
        .trim();

    if last.is_empty() {
        Vec::new()
    } else {
        vec![last.to_string()]
    }
}
```

Then replace the import extraction part inside `check_aggregate()`:

```rust
// crates/role-rules/src/capabilities_contract_role_auditor.rs

for line in content.lines() {
    let t = line.trim();

    let is_import = t.starts_with("use ")
        || (t.starts_with("from ") && t.contains(" import "))
        || (t.starts_with("import ") && t.contains(" from "));

    if !is_import {
        continue;
    }

    for pattern in &def.role.forbidden_inheritance.values {
        let (layer, suffixes) = Self::resolve_scope(pattern);
        let lower = t.to_lowercase();

        let layer_match = lower.contains(&format!("{}::", layer))
            || lower.contains(&format!("::{}::", layer))
            || lower.contains(&format!("{}.", layer))
            || lower.contains(&format!(".{}.", layer))
            || lower.contains(&format!("{}/", layer))
            || lower.contains(&format!("/{}/", layer));

        if !layer_match {
            continue;
        }

        if !suffixes.is_empty()
            && !suffixes.iter().any(|s| {
                lower.contains(&format!("_{}", s))
                    || lower.contains(&format!("::{}", s))
            })
        {
            continue;
        }

        forbidden_traits.extend(Self::imported_symbol_names(t));
    }
}

forbidden_traits.sort();
forbidden_traits.dedup();
```

And report with line numbers:

```rust
// crates/role-rules/src/capabilities_contract_role_auditor.rs

let mut reported = std::collections::HashSet::new();

for (line_no, line) in content.lines().enumerate() {
    let t = line.trim();

    for trait_name in &forbidden_traits {
        let rust_pattern = format!("impl {} for ", trait_name);
        let py_pattern = format!("({}", trait_name);
        let js_extends = format!("extends {}", trait_name);
        let js_implements = format!("implements {}", trait_name);

        if t.contains(&rust_pattern)
            || t.contains(&py_pattern)
            || t.contains(&js_extends)
            || t.contains(&js_implements)
        {
            let key = (line_no, trait_name.clone());

            if reported.insert(key) {
                let msg = Self::aes013_forbidden_inheritance(trait_name);

                violations.push(LintResult::new_arch(
                    file,
                    line_no + 1,
                    "AES013",
                    Severity::HIGH,
                    &msg,
                ));
            }
        }
    }
}
```

---

## 1.11 Rust passive-surface check counts private methods as public

### Location

`crates/role-rules/src/capabilities_surface_role_auditor.rs`

### Problem

The Rust passive checker uses:

```rust
RUST_FN_RE
```

which matches both:

```rust
fn private_method()
```

and:

```rust
pub fn public_method()
```

This can create false positives for passive surfaces.

### Fix

Use a public-function-only regex.

Replace:

```rust
static RUST_FN_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(").ok());
```

with:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

static RUST_PUB_FN_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"^\s*pub\s+(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)\s*\(").ok()
});
```

Add a brace-aware function-end helper:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

fn rust_fn_end_line(lines: &[&str], start: usize) -> usize {
    let mut depth: i32 = 0;
    let mut started = false;

    for (k, line) in lines.iter().enumerate().skip(start) {
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    started = true;
                }
                '}' => {
                    depth -= 1;
                }
                _ => {}
            }
        }

        if started && depth <= 0 {
            return k;
        }
    }

    lines.len()
}
```

Inside `_check_rust_passive()`, replace the function regex usage:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

let fn_re = match &*RUST_PUB_FN_RE {
    Some(r) => r,
    None => return,
};
```

and replace method-end estimation:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

if let (Some((name, _start)), Some(cap)) = (&current_impl, fn_re.captures(trimmed)) {
    let method_name = match cap.get(1).map(|m| m.as_str()) {
        Some(s) => s.to_string(),
        None => String::new(),
    };

    if !method_name.starts_with('_')
        && !name.contains("Drop")
        && !name.contains("Clone")
    {
        let end_line = Self::rust_fn_end_line(lines, i);

        methods.push((method_name, i + 1, Some(end_line)));
    }
}
```

Update `_add_impl_violations()` body-length calculation:

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

for (method_name, s, e) in methods {
    if let Some(end_line) = e {
        let body_len = (*end_line as i64) - (*s as i64) + 1;

        if body_len > MAX_FUNCTION_BODY_LINES {
            violations.push(format!(
                "Method '{}' is {} lines (max {})",
                method_name, body_len, MAX_FUNCTION_BODY_LINES
            ));
        }
    }
}
```

---

## 1.12 JavaScript passive check only detects exported classes

### Location

`crates/role-rules/src/capabilities_surface_role_auditor.rs`

### Problem

Current regex:

```rust
static JS_CLASS_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^export\s+class\s+(\w+)").ok());
```

misses:

```ts
class Foo {}
```

and:

```ts
export default class Foo {}
```

### Fix

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

static JS_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"^(?:export\s+)?(?:default\s+)?class\s+(\w+)").ok()
});
```

---

## 1.13 Control-flow nesting check assumes 4-space indentation

### Location

`crates/role-rules/src/capabilities_surface_role_auditor.rs`

### Problem

Current logic does:

```rust
let depth = indent / 4;
```

This is wrong for projects using:

- 2 spaces
- tabs
- mixed indentation

### Fix

Use an indentation stack instead of a hardcoded divisor.

```rust
// crates/role-rules/src/capabilities_surface_role_auditor.rs

fn _check_method_nesting(
    &self,
    class_name: &str,
    lines: &[&str],
    pub_methods: &[(String, usize, Option<usize>)],
    violations: &mut Vec<String>,
) {
    for (method_name, start, end) in pub_methods {
        let end_line = match end {
            Some(e) => *e,
            None => lines.len(),
        };

        let mut max_depth: usize = 0;
        let mut if_indents: Vec<usize> = Vec::new();

        for i in *start..end_line.min(lines.len()) {
            let line = lines[i];
            let trimmed = line.trim();
            let indent = line.len() - line.trim_start().len();

            // Close blocks that are no longer active.
            while let Some(&last) = if_indents.last() {
                if indent <= last {
                    if_indents.pop();
                } else {
                    break;
                }
            }

            if IF_RE.as_ref().is_some_and(|re| re.is_match(trimmed)) {
                if_indents.push(indent);
                max_depth = max_depth.max(if_indents.len());
            }
        }

        if max_depth > MAX_IF_DEPTH {
            violations.push(format!(
                "Method '{}.{}' has deep control flow (if-nesting > {})",
                class_name, method_name, MAX_IF_DEPTH
            ));
        }
    }
}
```

---

# 2. Performance Bottlenecks

---

## 2.1 Repeated full-content scans in AES403 Rust routing

### Problem

Old code did this per struct:

```rust
content.contains(...)
content.contains(...)
content.contains(...)
```

For `N` structs, this is `O(N * file_size)`.

### Fixed by

The replacement `_check_rust_routing()` now performs a single pass over the file and collects:

- struct definitions
- protocol implementations

This changes the hot path from:

```text
O(structs * lines)
```

to:

```text
O(lines)
```

---

## 2.2 Duplicate file reads for surface passive checks

### Problem

The orchestrator already reads the file:

```rust
let content = std::fs::read_to_string(file).unwrap_or_default();
```

But `_check_passive()` reads the file again through:

```rust
role_io::read_file(...)
```

This doubles I/O for surface files if both paths are used.

### Fixed by

The added `_check_passive_source()` method reuses the already-loaded `SourceContentVO`.

This avoids redundant disk reads when the orchestrator executes:

```rust
checker.check_passive_surface(&source_vo, violations);
```

---

## 2.3 Blocking filesystem I/O inside async audit

### Location

`crates/role-rules/src/agent_role_orchestrator.rs`

### Problem

`run_audit()` is async:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>
```

But it performs blocking filesystem operations:

```rust
std::fs::read_dir
std::fs::read_to_string
```

This can stall the async executor when scanning large workspaces.

### Recommended fix

If your runtime provides `spawn_blocking`, move filesystem traversal there.

Example pattern, if Tokio is available in the surrounding workspace:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
    let target = target.clone();

    let files = tokio::task::spawn_blocking(move || {
        self.collect_files(&target)
    })
    .await
    .unwrap_or_else(|_| FilePathList::new(Vec::new()));

    let file_strings: Vec<String> = files
        .values
        .iter()
        .map(|f| f.to_string())
        .collect();

    let mut results = Vec::new();

    self.run_all_role_checks(&file_strings, 500, &mut results);

    results
}
```

If you do not want to add a runtime dependency, at least document that `run_audit()` is blocking despite being async.

---

## 2.4 Regex compilation should be cached

### Problem

If regexes are compiled inside hot functions, performance degrades.

### Fixed by

The provided AES405 fix uses cached regexes:

```rust
static ANY_TYPE_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(...).ok()
});
```

This ensures regex compilation happens once.

---

# 3. Additional Correctness Notes

These are not patched above because they are either architectural or require broader API changes, but they are real issues.

---

## 3.1 Silent file-read failures can hide violations

### Location

`crates/role-rules/src/agent_role_orchestrator.rs`

Current code:

```rust
let content = std::fs::read_to_string(file).unwrap_or_default();
```

If a file cannot be read, it is treated as empty.

This can hide violations.

### Recommended behavior

Either:

1. Skip the file but emit an I/O warning result, or
2. Propagate an error result.

This requires choosing an error code/policy, so I did not inject a new one here.

---

## 3.2 Unknown prefixes are silently skipped

### Location

`crates/role-rules/src/agent_role_orchestrator.rs`

The comment says unknown prefixes should emit an INFO-level structured violation, but the code does:

```rust
_ => {}
```

If that behavior is intended, it should emit a diagnostic. If not, the comment should be removed.

---

## 3.3 Some contract methods still use primitives

Examples:

```rust
fn name(&self) -> &str;
```

and some `Vec<LintResult>` returns.

Depending on how strictly AES402 is enforced, these may also need taxonomy wrappers such as:

- `AdapterName`
- `LintResultList`

This is a broader API normalization task.

---

# 4. High-Priority Patch List

If you want the smallest set of patches that gives the largest correctness improvement, apply these first:

1. Fix `utility_layer_detector.rs` prefix mapping.
2. Fix orchestrator surface classification.
3. Implement `check_passive_surface()` using source content.
4. Replace AES405 `any` detection with regex token matching.
5. Replace AES403 Rust routing with the single-pass implementation.
6. Fix AES402 primitive token matching in `utility_signature_parser.rs`.
7. Fix grouped import parsing in `check_aggregate()`.
8. Fix Rust passive-surface public-method detection.
9. Fix indentation-based nesting detection.
10. Fix JavaScript class regex.

These changes correct existing AES401–AES406 behavior without adding new lint rules.
