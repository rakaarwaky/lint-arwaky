# Role-Rules Fix Plan

> Generated from BA + QA report validation against actual codebase.
> 17 validated issues across 6 phases.

## Decisions

| Decision                     | Choice                                                                                        |
| ---------------------------- | --------------------------------------------------------------------------------------------- |
| AES404 Utility Purity        | Implement checker                                                                             |
| AES401-R2 VO Check           | Deprecate (VOs/Constants CAN use primitives; entity/error/event MUST use VOs)                 |
| AES013 Forbidden Inheritance | Document in FRD                                                                               |
| Contract Protocol Primitives | Only `&str` and `bool` acceptable; all other primitives forbidden                             |
| Async File I/O               | Add `tokio::task::spawn_blocking`                                                             |
| Severity Scheme              | CRITICAL=security/correctness, HIGH=false positives/negatives, MEDIUM=improvements, LOW=style |

## Severity Legend

- **CRITICAL**: Security vulnerability or data-correctness bug
- **HIGH**: False positive/negative producing wrong lint results
- **MEDIUM**: Improvement, missing check, or architectural alignment
- **LOW**: Style, documentation, or minor optimization

---

## Phase 1: Critical Bug Fixes (CRITICAL)

### P1.1 — Fix `surface` vs `surfaces` layer mismatch

**Skill**: `create-utility-rust` — stateless constant map, domain-agnostic.
**File**: `crates/shared/src/common/utility_layer_detector.rs`
**Severity**: CRITICAL
**AES Code**: AES406
**Problem**: `detect_layer_from_prefix()` returns `"surface"` for `surface_*.rs` files, but `layer_surfaces()` returns `"surfaces"`. The comparison in `capabilities_surface_role_auditor.rs:189` never matches, silently disabling ALL AES406 checks.

**Before**:

```rust
const PREFIX_MAP: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("capabilities_", "capabilities"),
    ("utility_", "utility"),
    ("agent_", "agent"),
    ("surface_", "surface"),
    ("root_", "root"),
];
```

**After**:

```rust
const PREFIX_MAP: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("capabilities_", "capabilities"),
    ("capability_", "capabilities"),
    ("utility_", "utility"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("surfaces_", "surfaces"),
    ("root_", "root"),
];
```

---

### P1.2 — Fix router misclassification (smart vs utility)

**Skill**: `create-agent-rust` — classification logic is domain knowledge; ideally belongs in a capabilities helper. However, this is a simple pattern match used for orchestration routing, not business computation. Acceptable in agent if kept as a pure helper function.
**File**: `crates/role-rules/src/agent_role_orchestrator.rs`
**Severity**: CRITICAL
**AES Code**: AES406
**Problem**: ARCHITECTURE.md §10 lists `router` under Smart surfaces, but orchestrator line 143 classifies `_router` as utility. Smart surfaces get different checks than utility surfaces.

**Before** (line 135-143):

```rust
let is_smart = filename.contains("_command")
    || filename.contains("_controller")
    || filename.contains("_page")
    || filename.contains("_entry");
let is_utility = filename.contains("_hook")
    || filename.contains("_store")
    || filename.contains("_action")
    || filename.contains("_screen")
    || filename.contains("_router");
```

**After**:

```rust
let is_smart = filename.contains("_command")
    || filename.contains("_controller")
    || filename.contains("_page")
    || filename.contains("_entry")
    || filename.contains("_router");
let is_utility = filename.contains("_hook")
    || filename.contains("_store")
    || filename.contains("_action")
    || filename.contains("_screen");
```

Also update `check_surface_roles()` smart-surface detection (line 208-211):

```rust
// Before:
let is_smart = basename.ends_with("_command")
    || basename.ends_with("_controller")
    || basename.ends_with("_page")
    || basename.ends_with("_entry");

// After:
let is_smart = basename.ends_with("_command")
    || basename.ends_with("_controller")
    || basename.ends_with("_page")
    || basename.ends_with("_entry")
    || basename.ends_with("_router");
```

Also update `_check_passive()` smart-surface exemption (line 291-296):

```rust
// Before:
if basename.ends_with("_command")
    || basename.ends_with("_controller")
    || basename.ends_with("_page")
    || basename.ends_with("_entry")
{
    return;
}

// After:
if basename.ends_with("_command")
    || basename.ends_with("_controller")
    || basename.ends_with("_page")
    || basename.ends_with("_entry")
    || basename.ends_with("_router")
{
    return;
}
```

---

### P1.3 — Implement `check_passive_surface()` (currently no-op)

**Skill**: `create-capabilities-rust` — implementing protocol method, allowed.
**File**: `crates/role-rules/src/capabilities_surface_role_auditor.rs`
**Severity**: CRITICAL
**AES Code**: AES406
**Problem**: `check_passive_surface()` at lines 62-67 is empty. The real passive check exists in `check_surface_hierarchy()` but is not called from the orchestrator. AES406 passive-surface violations are missed.

**Before** (line 62-67):

```rust
fn check_passive_surface(
    &self,
    _source: &SourceContentVO,
    _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
}
```

**After**:

```rust
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

Add new helper method:

```rust
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

---

## Phase 2: False Positive/Negative Fixes (HIGH)

### P2.1 — Fix AES405 `any` detection false positives

**Skill**: `create-capabilities-rust` — adding regex + comment stripping to capabilities, allowed.
**File**: `crates/role-rules/src/capabilities_agent_role_auditor.rs`
**Severity**: HIGH
**AES Code**: AES405
**Problem**: Substring matching flags `anyhow::Error` (contains `: any`), comments with `: any`, etc.

**Before** (line 73-79):

```rust
if t.contains(": any")
    || t.contains(": Any")
    || t.contains("-> any")
    || t.contains("-> Any")
    || t.contains("Any<")
    || t.contains("Any[")
    || t.contains("any[")
```

**After**: Add regex and comment-stripping:

```rust
use once_cell::sync::Lazy;
use regex::Regex;

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

Replace `check_any_type_annotation` body:

```rust
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

---

### P2.2 — Fix AES403 Rust routing (inherent impls accepted)

**Skill**: `create-capabilities-rust` — rewriting protocol implementation check, allowed.
**File**: `crates/role-rules/src/capabilities_capabilities_role_auditor.rs`
**Severity**: HIGH
**AES Code**: AES403
**Problem**: `impl Foo { ... }` (inherent) is accepted as proof of protocol implementation. Also uses substring matching and suppresses checks when >3 structs.

**Before** (line 63-124): Single-pass with substring matching, `structs.len() <= 3` suppression.

**After**: Replace `_check_rust_routing()` with struct-aware single-pass:

```rust
fn _check_rust_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
    let has_proto_import = content.contains("use ")
        && (content.contains("_protocol::") || content.contains("_port::"));

    if !has_proto_import {
        violations.push(LintResult::new_arch(
            file, 1, "AES403", Severity::MEDIUM,
            AesRoleViolation::CapabilityNoProtocol { reason: None },
        ));
        return;
    }

    let mut structs: Vec<(String, usize)> = Vec::new();
    let mut implemented: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut in_cfg_test = false;
    let mut cfg_base_depth: i32 = 0;
    let mut brace_depth: i32 = 0;

    for (idx, raw) in content.lines().enumerate() {
        let t = raw.trim();
        let opens = t.matches('{').count() as i32;
        let closes = t.matches('}').count() as i32;

        if in_cfg_test {
            brace_depth += opens - closes;
            if brace_depth <= cfg_base_depth { in_cfg_test = false; }
            continue;
        }
        if t.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            cfg_base_depth = brace_depth;
            brace_depth += opens - closes;
            continue;
        }
        brace_depth += opens - closes;

        // Collect struct definitions
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

        // Collect trait implementations (NOT inherent impls)
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

            // Protocol traits use I-prefix convention: IAgentRoleChecker, etc.
            let is_protocol_like = trait_name.starts_with('I')
                && trait_name.chars().nth(1)
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
                file, line, "AES403", Severity::MEDIUM,
                AesRoleViolation::CapabilityRouting {
                    struct_name: SymbolName::new(struct_name),
                    reason: None,
                },
            ));
        }
    }
}
```

---

### P2.3 — Fix AES402 Python/TS primitive false positives

**Skill**: `create-utility-rust` — adding token-aware helpers, stateless standalone functions.
**File**: `crates/shared/src/common/utility_signature_parser.rs`
**Severity**: HIGH
**AES Code**: AES402
**Problem**: `lower.contains(": str")` matches `string_like`. `lower.contains(": any")` matches in wrong contexts.

**Before** (line 100-142): Substring matching with `lower.contains(": str")`.

**After**: Add token-aware helpers:

```rust
fn starts_with_type_token(s: &str, token: &str) -> bool {
    match s.strip_prefix(token) {
        Some(rest) => rest.chars().next()
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
            || s[after..].chars().next()
                .map_or(true, |c| !c.is_ascii_alphanumeric() && c != '_');
        if boundary_ok { return true; }
        from = abs + 1;
    }
    false
}
```

Replace `python_signature_uses_forbidden_primitive`:

```rust
pub fn python_signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let lower = sig.to_lowercase();
    let compact = lower.replace(": ", ":").replace(" :", ":")
        .replace("-> ", "->").replace(" ->", "->");

    let mut forbidden: Vec<&'static str> = Vec::new();
    let tokens = ["str", "int", "float", "bool", "list", "dict"];

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

Replace `typescript_signature_uses_forbidden_primitive`:

```rust
pub fn typescript_signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let lower = sig.to_lowercase();
    let compact = lower.replace(": ", ":").replace(" :", ":");

    let mut forbidden: Vec<&'static str> = Vec::new();
    let tokens = ["string", "number", "boolean", "any"];

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

Also fix `extract_python_method_signatures` to detect parameter-only annotations (not just `->`):

```rust
// Before (line 76):
if trimmed.starts_with("def ") && trimmed.contains("->") {

// After:
if trimmed.starts_with("def ") && (trimmed.contains(':') || trimmed.contains("->")) {
```

---

### P2.4 — Fix `extract_trait_method_signatures` invalid boolean logic

**Skill**: `create-utility-rust` — rewriting utility function, stateless standalone.
**File**: `crates/shared/src/common/utility_signature_parser.rs`
**Severity**: HIGH
**AES Code**: AES402
**Problem**: Line 26: `line.contains(')').ge(&line.contains('('))` is semantically wrong.

**Before** (line 24-26):

```rust
let is_trait_header = (line.starts_with("pub trait ") || line.starts_with("trait "))
    && line.contains('{')
    && line.contains(')').ge(&line.contains('('));
```

**After**: Replace with brace-aware trait detection:

```rust
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
                    if brace_depth <= 0 { in_trait_depth = 0; brace_depth = 0; }
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
                if brace_depth <= 0 { in_trait_depth = 0; brace_depth = 0; }
            }
            continue;
        }

        if line.starts_with("fn ") && line.contains(';') {
            results.push((line_no, raw.to_string()));
        }

        brace_depth += line.matches('{').count() as i32
            - line.matches('}').count() as i32;
        if brace_depth <= 0 { in_trait_depth = 0; brace_depth = 0; }
    }

    results
}
```

---

### P2.5 — Fix forbidden inheritance grouped import parsing

**Skill**: `create-capabilities-rust` — adding import parser helper to capabilities, allowed.
**File**: `crates/role-rules/src/capabilities_contract_role_auditor.rs`
**Severity**: HIGH
**AES Code**: AES013
**Problem**: `use foo::bar::{Baz, Qux}` only extracts `Qux` (last segment). Also reports `line: 0`.

**Before** (line 83-98): Uses `t.split("::").last()`.

**After**: Add import-symbol parser:

```rust
fn imported_symbol_names(import_line: &str) -> Vec<String> {
    let t = import_line.trim().trim_end_matches(';').trim();

    // Rust grouped import: use foo::bar::{Baz, Qux};
    if let Some(open) = t.find('{') {
        let close = t.rfind('}').unwrap_or(t.len());
        if close > open {
            let inner = &t[open + 1..close];
            return inner.split(',').filter_map(|item| {
                let item = item.trim();
                if item.is_empty() { return None; }
                let name = if let Some(pos) = item.find(" as ") {
                    item[..pos].trim()
                } else { item };
                let name = name.trim()
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .trim();
                if name.is_empty() { None } else { Some(name.to_string()) }
            }).collect();
        }
    }

    // Python: from foo import Bar, Baz
    if let Some(pos) = t.find(" import ") {
        let names = &t[pos + 8..];
        return names.split(',').filter_map(|name| {
            let name = name.trim();
            let name = if let Some(as_pos) = name.find(" as ") {
                name[..as_pos].trim()
            } else { name };
            if name.is_empty() { None } else { Some(name.to_string()) }
        }).collect();
    }

    // Rust single import: use foo::bar::Baz;
    let last = t.split("::").last().unwrap_or_default().trim();
    let last = if let Some(pos) = last.find(" as ") {
        last[..pos].trim()
    } else { last };
    let last = last.trim_start_matches('{').trim_end_matches('}').trim();
    if last.is_empty() { Vec::new() } else { vec![last.to_string()] }
}
```

Replace import extraction in `check_aggregate()` (line 56-99):

```rust
for line in content.lines() {
    let t = line.trim();
    let is_import = t.starts_with("use ")
        || (t.starts_with("from ") && t.contains(" import "))
        || (t.starts_with("import ") && t.contains(" from "));
    if !is_import { continue; }

    for pattern in &def.role.forbidden_inheritance.values {
        let (layer, suffixes) = Self::resolve_scope(pattern);
        let lower = t.to_lowercase();
        let layer_match = lower.contains(&format!("{}::", layer))
            || lower.contains(&format!("::{}::", layer))
            || lower.contains(&format!("{}.", layer))
            || lower.contains(&format!(".{}.", layer))
            || lower.contains(&format!("{}/", layer))
            || lower.contains(&format!("/{}/", layer));
        if !layer_match { continue; }
        if !suffixes.is_empty()
            && !suffixes.iter().any(|s| {
                lower.contains(&format!("_{}", s))
                    || lower.contains(&format!("::{}", s))
            })
        { continue; }

        forbidden_traits.extend(Self::imported_symbol_names(t));
    }
}
forbidden_traits.sort();
forbidden_traits.dedup();
```

Replace violation reporting (line 101-120) with line-numbered version:

```rust
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
                    file, line_no + 1, "AES013", Severity::HIGH, &msg,
                ));
            }
        }
    }
}
```

---

### P2.6 — Fix Rust passive-surface private method counting

**Skill**: `create-capabilities-rust` — changing regex and adding helper, allowed.
**File**: `crates/role-rules/src/capabilities_surface_role_auditor.rs`
**Severity**: HIGH
**AES Code**: AES406
**Problem**: `RUST_FN_RE` matches both `fn private()` and `pub fn public()`. Passive surfaces should only count public methods.

**Before** (line 123-124):

```rust
static RUST_FN_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(").ok());
```

**After**:

```rust
static RUST_PUB_FN_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"^\s*pub\s+(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)\s*\(").ok()
});
```

Update `_check_rust_passive()` to use `RUST_PUB_FN_RE` and add brace-aware function end detection:

```rust
fn rust_fn_end_line(lines: &[&str], start: usize) -> usize {
    let mut depth: i32 = 0;
    let mut started = false;
    for (k, line) in lines.iter().enumerate().skip(start) {
        for ch in line.chars() {
            match ch {
                '{' => { depth += 1; started = true; }
                '}' => { depth -= 1; }
                _ => {}
            }
        }
        if started && depth <= 0 { return k; }
    }
    lines.len()
}
```

---

### P2.7 — Fix JS class regex (misses non-exported classes)

**Skill**: `create-capabilities-rust` — changing regex, allowed.
**File**: `crates/role-rules/src/capabilities_surface_role_auditor.rs`
**Severity**: HIGH
**AES Code**: AES406
**Problem**: `JS_CLASS_RE` only matches `export class Foo`, misses `class Foo` and `export default class Foo`.

**Before** (line 109):

```rust
static JS_CLASS_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^export\s+class\s+(\w+)").ok());
```

**After**:

```rust
static JS_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"^(?:export\s+)?(?:default\s+)?class\s+(\w+)").ok()
});
```

---

### P2.8 — Fix nesting depth hardcoded 4-space assumption

**Skill**: `create-capabilities-rust` — rewriting nesting check, allowed.
**File**: `crates/role-rules/src/capabilities_surface_role_auditor.rs`
**Severity**: HIGH
**AES Code**: AES406
**Problem**: `let depth = indent / 4;` assumes 4-space indentation. Wrong for 2-space, tabs, or mixed.

**Before** (line 616-618):

```rust
let indent = line.len() - line.trim_start().len();
let depth = indent / 4;
if depth > max_depth { max_depth = depth; }
```

**After**: Use indentation stack:

```rust
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

            while let Some(&last) = if_indents.last() {
                if indent <= last { if_indents.pop(); } else { break; }
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

## Phase 3: Contract Protocol Cleanup (MEDIUM)

### P3.1 — Fix AES403 contract raw `&str` parameter

**Skill**: `create-contract-rust` — `&str` is acceptable per user decision, no change needed.
**File**: `crates/shared/src/role-rules/contract_capabilities_role_protocol.rs`
**Severity**: MEDIUM
**AES Code**: AES402
**Problem**: `layer: &str` is a raw primitive in a contract protocol. User decision: only `&str` and `bool` are acceptable.

**Decision**: `&str` is acceptable per user. **No change needed.**

---

### P3.2 — Fix AES405 contract raw `usize` parameter

**Skill**: `create-contract-rust` — replacing primitive with `Count` VO, must remain object-safe.
**File**: `crates/shared/src/role-rules/contract_agent_role_protocol.rs`
**Severity**: MEDIUM
**AES Code**: AES402
**Problem**: `max_lines: usize` is a raw primitive in a contract protocol. User decision: all primitives except `&str` and `bool` are forbidden.

**Before** (line 9-14):

```rust
fn check_file_size_limit(
    &self,
    source: &SourceContentVO,
    max_lines: usize,
    violations: &mut Vec<LintResult>,
);
```

**After**: Use `Count` VO:

```rust
use crate::common::taxonomy_common_vo::Count;

fn check_file_size_limit(
    &self,
    source: &SourceContentVO,
    max_lines: Count,
    violations: &mut Vec<LintResult>,
);
```

Update implementation in `capabilities_agent_role_auditor.rs`:

```rust
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
            file, 1, "AES405", Severity::HIGH,
            AesRoleViolation::AgentFileSizeLimit { max_lines: max_lines_usize }.to_string(),
        ));
    }
}
```

Update orchestrator call:

```rust
use shared::common::taxonomy_common_vo::Count;

"agent" => {
    let checker = self.aggregate.agent();
    checker.check_file_size_limit(&source_vo, Count::new(max_lines as i64), violations);
    checker.check_any_type_annotation(&source_vo, violations);
    // ... rest unchanged
}
```

---

## Phase 4: New Capability — AES404 Utility Purity (MEDIUM)

### P4.1 — Implement utility purity checker

**Skill**: `create-capabilities-rust` — new capabilities file, must follow 3-block structure (struct → protocol impl → constructors/helpers).
**File**: `crates/role-rules/src/capabilities_utility_role_auditor.rs` (NEW)
**Severity**: MEDIUM
**AES Code**: AES404
**Problem**: AES404 requires utility files to be stateless standalone functions. Zero implementation exists.

**New file**:

```rust
// PURPOSE: UtilityRoleChecker — AES404: utility files must be stateless standalone functions
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct UtilityRoleChecker {}

impl UtilityRoleChecker {
    pub fn new() -> Self { Self {} }

    pub fn check_utility_purity(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let file = source.file_path.value();
        let content = source.content.value();
        let li = detect_language_info_from_source(source);

        if li.is_rs {
            self._check_rust_utility(file, content, violations);
        } else if li.is_py {
            self._check_python_utility(file, content, violations);
        } else if li.is_js {
            self._check_js_utility(file, content, violations);
        }
    }

    fn _check_rust_utility(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Utility must not have struct/enum/trait/impl definitions
            if t.starts_with("pub struct ") || t.starts_with("struct ")
                || t.starts_with("pub enum ") || t.starts_with("enum ")
                || t.starts_with("pub trait ") || t.starts_with("trait ")
                || t.starts_with("impl ")
            {
                violations.push(LintResult::new_arch(
                    file, i + 1, "AES404", Severity::MEDIUM,
                    AesRoleViolation::UtilityPurity {
                        reason: Some("Utility files must not contain struct/enum/trait/impl definitions".into()),
                    }.to_string(),
                ));
            }
            // Utility must not have static mut or lazy_static
            if t.contains("static mut ") || t.contains("lazy_static!") {
                violations.push(LintResult::new_arch(
                    file, i + 1, "AES404", Severity::MEDIUM,
                    AesRoleViolation::UtilityPurity {
                        reason: Some("Utility files must not contain mutable state".into()),
                    }.to_string(),
                ));
            }
        }
    }

    fn _check_python_utility(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Utility must not have class definitions
            if t.starts_with("class ") {
                violations.push(LintResult::new_arch(
                    file, i + 1, "AES404", Severity::MEDIUM,
                    AesRoleViolation::UtilityPurity {
                        reason: Some("Utility files must not contain class definitions".into()),
                    }.to_string(),
                ));
            }
            // Utility must not have global mutable state
            if t.starts_with("global ") || t.contains("= []") || t.contains("= {}") {
                if !t.starts_with("def ") && !t.starts_with("#") {
                    violations.push(LintResult::new_arch(
                        file, i + 1, "AES404", Severity::MEDIUM,
                        AesRoleViolation::UtilityPurity {
                            reason: Some("Utility files must not contain mutable state".into()),
                        }.to_string(),
                    ));
                }
            }
        }
    }

    fn _check_js_utility(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Utility must not have class definitions
            if t.starts_with("class ") || t.starts_with("export class ") {
                violations.push(LintResult::new_arch(
                    file, i + 1, "AES404", Severity::MEDIUM,
                    AesRoleViolation::UtilityPurity {
                        reason: Some("Utility files must not contain class definitions".into()),
                    }.to_string(),
                ));
            }
        }
    }
}
```

Add `UtilityPurity` variant to `AesRoleViolation` in `taxonomy_violation_role_vo.rs`.

Wire into orchestrator:

```rust
"utility" => {
    let checker = UtilityRoleChecker::new();
    checker.check_utility_purity(&source_vo, violations);
}
```

**Module registration:** Add `pub mod capabilities_utility_role_auditor;` to `crates/role-rules/src/mod.rs`.

---

## Phase 5: Performance Fixes (MEDIUM)

### P5.1 — Add `tokio::task::spawn_blocking` for filesystem I/O

**Skill**: `create-agent-rust` — agent must have "zero I/O." Using `spawn_blocking` delegates I/O to a thread pool, which is a performance optimization. The agent itself doesn't perform I/O directly. Acceptable if the blocking task is a pure function call. If tokio is not a dependency, document the limitation instead.
**File**: `crates/role-rules/src/agent_role_orchestrator.rs`
**Severity**: MEDIUM
**AES Code**: N/A (performance)
**Problem**: `run_audit()` is async but uses blocking `std::fs::read_dir` and `std::fs::read_to_string`.

**Before** (line 221-227):

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
    let mut results = Vec::new();
    let files = self.collect_files(target);
    let file_strings: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
    self.run_all_role_checks(&file_strings, 500, &mut results);
    results
}
```

**After**:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
    let target = target.clone();
    let this = self.clone_for_blocking();

    let files = tokio::task::spawn_blocking(move || {
        this.collect_files(&target)
    })
    .await
    .unwrap_or_else(|_| FilePathList::new(Vec::new()));

    let file_strings: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
    let mut results = Vec::new();
    self.run_all_role_checks(&file_strings, 500, &mut results);
    results
}
```

Note: Requires `RoleOrchestrator` to implement `Clone` or use `Arc` wrapping for the blocking task. If tokio is not a dependency, document the limitation instead.

---

### P5.2 — Fix `line: 0` precision reporting

**Skill**: `create-capabilities-rust` — fixing line number reporting in capabilities, allowed.
**File**: Multiple files
**Severity**: MEDIUM
**AES Code**: AES403, AES405, AES406
**Problem**: Multiple violations emit `line: 0` instead of actual line numbers.

**Files to fix**:

- `capabilities_capabilities_role_auditor.rs:68` — `CapabilityNoProtocol` → line 1
- `capabilities_agent_role_auditor.rs:56` — `AgentFileSizeLimit` → line 1
- `capabilities_surface_role_auditor.rs:84` — `SurfaceRoleViolation` → line 1

Replace `line: 0` with `line: 1` in all three locations.

---

## Phase 6: Documentation & FRD Updates (LOW)

### P6.1 — Document AES013 in FRD

**Skill**: `add-docs-rust` — documenting architecture rule in FRD.
**File**: FRD document
**Severity**: LOW
**AES Code**: AES013
**Problem**: AES013 (forbidden inheritance) is implemented but undocumented.

**Action**: Add AES013 requirement to FRD:

```
### AES013 — Forbidden Inheritance

Contract files must not inherit from forbidden source layers.
The forbidden inheritance patterns are defined in LayerDefinition.role.forbidden_inheritance.
Any `impl Trait for X` or equivalent that uses a disallowed trait is flagged as HIGH severity.
```

---

### P6.2 — Deprecate AES401-R2 (VO primitive check)

**Skill**: `add-docs-rust` — updating FRD documentation, deprecating rule.
**File**: FRD document
**Severity**: LOW
**AES Code**: AES401-R2
**Problem**: `check_vo()` is a no-op. User clarified: VOs/Constants CAN use primitives; entity/error/event MUST use VOs.

**Action**: Update FRD:

```
### AES401-R2 — VO Primitive Usage (DEPRECATED)

This requirement is deprecated. Taxonomy VOs and Constants are permitted to use raw primitive types
as they form the foundation layer. Entity, Error, and Event files must use VOs/Constants instead
of raw primitives (enforced by AES401 entity/error/event checks).
```

Remove or mark `check_vo_impl()` as deprecated in code.

---

### P6.3 — Add measurable acceptance criteria to FRD

**Skill**: `add-docs-rust` — adding acceptance criteria to FRD.
**File**: FRD document
**Severity**: LOW
**Problem**: No acceptance criteria, no test scenarios, no measurable thresholds.

**Action**: Add acceptance criteria section:

```
## Acceptance Criteria

| Rule | Criterion | Measurable Threshold |
|------|-----------|---------------------|
| AES401 | Constant purity | Non-const declaration in *_constant.rs → 1 violation |
| AES402 | Contract primitives | Primitive type in trait signature → 1 violation per line |
| AES403 | Capability routing | Struct without I-prefix trait impl → 1 violation per struct |
| AES404 | Utility purity | Struct/enum/trait/impl in utility file → 1 violation per definition |
| AES405 | Agent purity | File > 500 lines → 1 violation; `: any` annotation → 1 per line |
| AES406 | Surface passive | >10 public methods → 1 violation; >80 line method → 1 per method |
| AES013 | Forbidden inheritance | Forbidden trait impl → 1 violation per occurrence |
```

---

## Execution Order

1. **Phase 1** (P1.1-P1.3): CRITICAL fixes — layer mismatch, router classification, passive surface.
   - **Verify:** `cargo check -p role-rules && cargo check -p shared`
2. **Phase 2** (P2.1-P2.8): HIGH false positive/negative fixes. Can run in parallel with Phase 1.
   - **Verify:** `cargo check -p role-rules && cargo check -p shared`
3. **Phase 3** (P3.1-P3.2): MEDIUM contract cleanup. Depends on Phase 2 (signature parser changes).
   - **Verify:** `cargo check -p shared && cargo check -p role-rules`
4. **Phase 4** (P4.1): MEDIUM new capability. Independent.
   - **Verify:** `cargo check -p role-rules`
5. **Phase 5** (P5.1-P5.2): MEDIUM performance. Independent.
   - **Verify:** `cargo check -p role-rules`
6. **Phase 6** (P6.1-P6.3): LOW documentation. Independent.

**Final verification (all phases complete):**

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --workspace
cargo run --bin lint-arwaky-cli -- check .
```

---

## Summary

| Phase | Items     | Severity | Description                                                  |
| ----- | --------- | -------- | ------------------------------------------------------------ |
| 1     | P1.1-P1.3 | CRITICAL | Layer mismatch, router classification, passive surface no-op |
| 2     | P2.1-P2.8 | HIGH     | False positives/negatives in AES405, AES403, AES402, AES406  |
| 3     | P3.1-P3.2 | MEDIUM   | Contract protocol primitive cleanup                          |
| 4     | P4.1      | MEDIUM   | New AES404 utility purity checker                            |
| 5     | P5.1-P5.2 | MEDIUM   | Performance (async I/O) and precision reporting              |
| 6     | P6.1-P6.3 | LOW      | FRD documentation updates                                    |

**Total**: 17 items across 6 phases.

---

## Files Summary

### New files (1)

- `crates/role-rules/src/capabilities_utility_role_auditor.rs` — AES404 utility purity checker

### Modified files (8)

- `crates/shared/src/common/utility_layer_detector.rs` — fix surface/surfaces prefix map (P1.1)
- `crates/shared/src/common/utility_signature_parser.rs` — token-aware helpers, fix trait extraction (P2.3, P2.4)
- `crates/shared/src/role-rules/contract_agent_role_protocol.rs` — replace `usize` with `Count` VO (P3.2)
- `crates/role-rules/src/agent_role_orchestrator.rs` — fix router classification, add spawn_blocking (P1.2, P5.1)
- `crates/role-rules/src/capabilities_agent_role_auditor.rs` — fix any detection, line precision (P2.1, P5.2)
- `crates/role-rules/src/capabilities_capabilities_role_auditor.rs` — fix routing check (P2.2)
- `crates/role-rules/src/capabilities_contract_role_auditor.rs` — fix import parsing (P2.5)
- `crates/role-rules/src/capabilities_surface_role_auditor.rs` — fix passive surface, regex, nesting (P1.3, P2.6, P2.7, P2.8, P5.2)
- `crates/role-rules/src/mod.rs` — register new capabilities_utility_role_auditor module (P4.1)
