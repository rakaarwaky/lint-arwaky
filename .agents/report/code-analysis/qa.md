Hi Raka, I reviewed the `code-analysis` crate and the imported `shared` definitions. Below are the concrete functional defects and performance bottlenecks I found, followed by drop-in fixes. These changes only correct and optimize existing AES301–AES305 behavior; they do not add new features or new rules.

---

## 1. Defects and bottlenecks found

### Functional bugs

1. **`clippy::collapsible_match` in `capabilities_check_bypass_checker.rs`**

   - The bypass-token classification used nested conditional logic where a guarded `match` is cleaner and satisfies Clippy.
   - Fixed by converting the token dispatch into a proper guarded `match`.
2. **Static `Lazy` multiline block skipping did not actually skip lines**

   - The checker computed the brace depth of a `static ... Lazy` block, but the outer line iterator still processed the following lines.
   - This could produce false positives inside multiline static initializers.
   - Fixed by advancing the line index past the whole brace-delimited block.
3. **`#[cfg(test)]` handling in dead-inheritance checking skipped the rest of the file**

   - `in_test_module` was set to `true` and never reset.
   - Result: every line after the first test attribute was ignored.
   - Fixed by skipping only the actual test module block, not the remainder of the file.
4. **Public unit structs were not detected as dead inheritance**

   - The old check required the line to start with `struct `, so `pub struct Foo;` and `pub(crate) struct Foo;` were missed.
   - Fixed by stripping Rust visibility modifiers before checking.
5. **Exported JS/TS empty classes were not detected**

   - The old check required lines to start with `class `, missing:
     - `export class Foo {}`
     - `export default class Foo {}`
     - `declare class Foo {}`
     - `abstract class Foo {}`
   - Fixed with declaration-keyword awareness.
6. **`rust_declares_type` had false positives and false negatives**

   - It could treat comments as definitions.
   - It incorrectly rejected tuple structs because of the `!line.contains('(')` condition.
   - Fixed by:
     - stripping line comments,
     - matching whole keyword tokens,
     - allowing tuple structs.
7. **AES305 thresholds were hardcoded in the orchestrator**

   - `run_all_checks` used fixed values:
     - `min_dup_lines = 5`
     - `threshold_pct = 50.0`
   - This ignored configured AES305 values.
   - Fixed by reading thresholds from `ArchitectureConfig`.
8. **`CodeDuplicationAnalyzer::handle_duplicates` ignored runtime configuration**

   - It always used `default_aes_config()`.
   - Fixed by allowing the analyzer to carry an `ArchitectureConfig` instance.
9. **Fallback bypass pattern list was missing**

   - The checker comment said a fallback default list applies when config patterns are empty, but the implementation did not provide one.
   - Fixed by adding a default forbidden-bypass pattern list when the configured list is empty.
10. **Inner attribute bypasses were missed**

- `#[allow(...)]` and `#[expect(...)]` were detected, but inner attribute forms such as `#![allow(...)]` were not.
- Fixed by detecting both outer and inner attribute forms.

11. **Safe/unsafe unwrap detection could produce false negatives**

- If a line contained both a safe helper and an unsafe call, the old line-level contains-check could suppress the whole line.
- Fixed with a position-aware scanner that distinguishes safe helper variants from unsafe calls.

---

### Performance bottlenecks

1. **Duplication detection normalized every window twice**

   - First pass built the global window map.
   - Second pass re-normalized every window again to count shared windows.
   - Fixed by storing normalized window IDs per file during the first pass.
2. **Duplication interner stored unused strings**

   - `interned_keys` retained every normalized key string but was never read.
   - Fixed by removing it.
3. **Duplication global map stored more data than needed**

   - It stored `(file_index, line_number)` locations.
   - For file-level similarity, only file membership is needed.
   - Fixed by storing only file indices.
4. **Bypass checking lowercased patterns repeatedly**

   - Each pattern was lowercased for every line.
   - Fixed by precomputing lowercase patterns once per file scan.

---

## 2. Fixed code

Apply the following replacements.

---

# File: `crates/shared/src/code-analysis/utility_bypass.rs`

Add these helpers at the end of the file. They are stateless utility functions, so they remain compliant with the Utility layer.

```rust
/// Skip a brace-delimited block starting at `start`.
///
/// Returns the index of the first line after the block.
/// If the starting line is already balanced or has no opening brace,
/// returns `start + 1`.
pub fn skip_brace_block(lines: &[&str], start: usize) -> usize {
    if start >= lines.len() {
        return start;
    }

    let mut depth = lines[start].matches('{').count() as i32
        - lines[start].matches('}').count() as i32;
    let mut idx = start + 1;

    if depth <= 0 {
        return idx;
    }

    while idx < lines.len() {
        depth += lines[idx].matches('{').count() as i32
            - lines[idx].matches('}').count() as i32;
        idx += 1;

        if depth <= 0 {
            break;
        }
    }

    idx
}

/// Skip a `#[cfg(test)]` module block when present.
///
/// If the attribute is followed by a test module, returns the first line
/// after that module. Otherwise, returns `start + 1`, skipping only the
/// attribute line.
pub fn skip_cfg_test_block(lines: &[&str], start: usize) -> usize {
    if start >= lines.len() {
        return start;
    }

    let mut idx = start + 1;

    // Skip blank lines and additional attributes.
    while idx < lines.len() {
        let t = lines[idx].trim();
        if t.is_empty() || t.starts_with('#') {
            idx += 1;
            continue;
        }
        break;
    }

    if idx >= lines.len() {
        return idx;
    }

    let t = lines[idx].trim();
    let is_mod = t.split_whitespace().any(|w| w == "mod");

    // Not a module attribute; skip only the attribute line.
    if !is_mod {
        return start + 1;
    }

    // Module declaration without body, e.g. `mod tests;`.
    if t.ends_with(';') && !t.contains('{') {
        return idx + 1;
    }

    let mut depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
    idx += 1;

    // The module body opened and closed on the same line, e.g. `mod tests {}`.
    if depth <= 0 && t.contains('{') {
        return idx;
    }

    // Look for an opening brace on following lines.
    if depth <= 0 {
        while idx < lines.len() {
            let st = lines[idx].trim();
            depth += st.matches('{').count() as i32 - st.matches('}').count() as i32;
            idx += 1;

            if depth > 0 {
                break;
            }

            // Opened and closed immediately on the next line.
            if depth <= 0 && st.contains('{') {
                return idx;
            }
        }
    }

    // Consume until the module body closes.
    while idx < lines.len() && depth > 0 {
        let st = lines[idx].trim();
        depth += st.matches('{').count() as i32 - st.matches('}').count() as i32;
        idx += 1;
    }

    idx
}
```

---

# File: `crates/shared/src/code-analysis/utility_mandatory.rs`

Replace the file content with:

```rust
// PURPOSE: Stateless utility functions for mandatory definition checking (AES303)
// Extracted from capabilities_mandatory_definition_checker.rs — pure functions, no &self, no I/O

/// Check if a line declares a Rust struct, enum, or trait.
///
/// Handles visibility modifiers and avoids comment-based false positives.
/// Tuple structs are valid definitions and must not be rejected.
pub fn rust_declares_type(line: &str) -> bool {
    let code = line
        .split_once("//")
        .map(|(code, _comment)| code)
        .unwrap_or(line);

    contains_word_token(code, "struct")
        || contains_word_token(code, "enum")
        || contains_word_token(code, "trait")
}

/// Returns true when `word` appears as a whole identifier token in `haystack`.
fn contains_word_token(haystack: &str, word: &str) -> bool {
    haystack
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|token| token == word)
}
```

---

# File: `crates/code-analysis/src/capabilities_check_bypass_checker.rs`

Replace the file content with:

```rust
// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES304: detect bypass annotations, panics, and fallback calls
// ALGORITHM:
//   1. Skip #[cfg(test)] blocks and static Lazy<Regex> multiline inits
//   2. Detect source language from the file extension (shared Language VO).
//   3. For each line, classify forbidden tokens using word-boundary aware substring matching.
//   4. Patterns are read from ArchitectureConfig.code_analysis.forbidden_bypass.values so
//      YAML config is honored. A fallback default list applies if empty.

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::{
    AesCodeAnalysisViolation, Language,
};
use shared::code_analysis::utility_bypass::{
    is_ident_continue, is_ident_start, matches_word_token, skip_brace_block, skip_cfg_test_block,
    starts_with_allow_attr,
};
use shared::common::taxonomy_common_vo::PatternList;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct BypassChecker {
    rule: CodeAnalysisRuleVO,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IBypassCheckerProtocol for BypassChecker {
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>) {
        let mut in_clippy_section = false;

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();

            if t.starts_with("[workspace.lints.clippy]") || t.starts_with("[lints.clippy]") {
                in_clippy_section = true;
                continue;
            }

            if in_clippy_section {
                if t.starts_with('[') {
                    in_clippy_section = false;
                    continue;
                }

                if let Some(eq_pos) = t.find('=') {
                    let val = t[eq_pos + 1..].trim();
                    if val == "\"allow\"" || val == "'allow'" {
                        violations.push(LintResult::new_arch(
                            "Cargo.toml",
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            format!("Cargo.toml clippy allow bypass: `{}`", t),
                        ));
                    }
                }
            }
        }
    }

    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let patterns = &self.rule.forbidden_bypass;

        if patterns.values.is_empty() {
            return;
        }

        // Early-exit: skip scan if file contains none of the bypass-related tokens.
        let content_lower = content.to_lowercase();
        let has_bypass_token = patterns
            .values
            .iter()
            .any(|p| !p.is_empty() && content_lower.contains(&p.to_lowercase()))
            || content_lower.contains("raise ")
            || content_lower.contains("throw new");

        if !has_bypass_token {
            return;
        }

        // Precompute lowercase patterns once per file.
        let lowered_patterns: Vec<String> = patterns
            .values
            .iter()
            .map(|p| p.to_lowercase())
            .collect();

        let language = Language::from_file(file);
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let t = lines[i].trim();

            // Skip doc comments — documentation references to patterns are not runtime violations.
            if t.starts_with("///") || t.starts_with("//!") {
                i += 1;
                continue;
            }

            // Skip test modules when present.
            if t.starts_with("#[cfg(test)]") {
                i = skip_cfg_test_block(&lines, i);
                continue;
            }

            // Skip static Lazy initialization blocks.
            if t.contains("static ") && t.contains("Lazy") {
                i = skip_brace_block(&lines, i);
                continue;
            }

            // Allow/expect attributes are bypasses.
            if starts_with_allow_attr(t)
                || t.starts_with("#![allow(")
                || t.starts_with("#![expect(")
            {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES304",
                    Severity::CRITICAL,
                    AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                ));
                i += 1;
                continue;
            }

            let t_lower = t.to_lowercase();
            let mut matched = false;

            for (p_str, p_lower) in patterns
                .values
                .iter()
                .map(|p| p.as_str())
                .zip(lowered_patterns.iter())
            {
                if p_str.is_empty() {
                    continue;
                }

                match p_str {
                    "unwrap" if Self::contains_unsafe_unwrap(t) => {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::UnwrapExpect { reason: None }.to_string(),
                        ));
                        matched = true;
                        break;
                    }

                    // Safe helper variants are not violations.
                    "unwrap" => {}

                    "expect" if matches_word_token(t, p_str, true) => {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::UnwrapExpect { reason: None }.to_string(),
                        ));
                        matched = true;
                        break;
                    }

                    "panic" if matches_word_token(t, p_str, false) => {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Panic { reason: None }.to_string(),
                        ));
                        matched = true;
                        break;
                    }

                    "todo" if matches_word_token(t, p_str, false) => {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Todo { reason: None }.to_string(),
                        ));
                        matched = true;
                        break;
                    }

                    "unimplemented" | "unreachable"
                        if matches_word_token(t, p_str, false) =>
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Unimplemented { reason: None }.to_string(),
                        ));
                        matched = true;
                        break;
                    }

                    _ if !Self::is_word_pattern_token(p_str)
                        && t_lower.contains(p_lower.as_str()) =>
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                        ));
                        matched = true;
                        break;
                    }

                    _ => {}
                }
            }

            // One violation per line is enough; avoid duplicate reporting.
            if matched {
                i += 1;
                continue;
            }

            // Language-scoped phrase patterns.
            let line_lc = t.to_lowercase();

            match language {
                Language::Python => {
                    if line_lc.contains("raise notimplementederror")
                        || line_lc.contains("raise notimplemented")
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Unimplemented { reason: None }.to_string(),
                        ));
                    } else if line_lc.contains("assert false") {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Panic { reason: None }.to_string(),
                        ));
                    }
                }
                Language::JavaScript | Language::TypeScript => {
                    if line_lc.contains("throw new error")
                        || line_lc.contains("throw new typeerror")
                        || line_lc.contains("throw new rangeerror")
                        || line_lc.contains("throw new referenceerror")
                        || line_lc.contains("throw new syntaxerror")
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            AesCodeAnalysisViolation::Panic { reason: None }.to_string(),
                        ));
                    }
                }
                Language::Rust => {}
            }

            i += 1;
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for BypassChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BypassChecker {
    pub fn new() -> Self {
        Self {
            rule: CodeAnalysisRuleVO {
                forbidden_bypass: Self::default_forbidden_bypass(),
                ..CodeAnalysisRuleVO::default()
            },
        }
    }

    /// Build a BypassChecker from an ArchitectureConfig-derived CodeAnalysisRuleVO.
    pub fn from_rule(rule: CodeAnalysisRuleVO) -> Self {
        let mut rule = rule;

        if rule.forbidden_bypass.is_empty() {
            rule.forbidden_bypass = Self::default_forbidden_bypass();
        }

        Self { rule }
    }

    /// Build a BypassChecker from a PatternList (forbidden_bypass patterns).
    pub fn from_patterns(patterns: &PatternList) -> Self {
        let forbidden_bypass = if patterns.is_empty() {
            Self::default_forbidden_bypass()
        } else {
            patterns.clone()
        };

        Self {
            rule: CodeAnalysisRuleVO {
                forbidden_bypass,
                ..CodeAnalysisRuleVO::default()
            },
        }
    }

    /// Default forbidden bypass tokens used when configuration provides none.
    fn default_forbidden_bypass() -> PatternList {
        PatternList::new(vec![
            "unwrap",
            "expect",
            "panic",
            "todo",
            "unimplemented",
            "unreachable",
            "noqa",
            "type: ignore",
            "@ts-ignore",
            "@ts-expect-error",
            "eslint-disable",
            "eslint-disable-next-line",
        ])
    }

    /// Tokens that require call-site style matching rather than plain contains.
    fn is_word_pattern_token(token: &str) -> bool {
        matches!(
            token,
            "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable"
        )
    }

    /// Detect unsafe unwrap calls while allowing safe helper variants.
    ///
    /// Safe:
    ///   .unwrap_or(...)
    ///   .unwrap_or_else(...)
    ///   .unwrap_or_default(...)
    ///
    /// Unsafe:
    ///   .unwrap(...)
    ///   .unwrap_err(...)
    fn contains_unsafe_unwrap(line: &str) -> bool {
        let bytes = line.as_bytes();
        let token = "unwrap".as_bytes();
        let tlen = token.len();
        let mut i = 0;

        while i + tlen <= bytes.len() {
            if &bytes[i..i + tlen] == token {
                // Require method-call style: `.unwrap`.
                if i > 0 && bytes[i - 1] == b'.' {
                    let mut j = i + tlen;

                    if j < bytes.len() {
                        // Direct unsafe call: `.unwrap(`
                        if bytes[j] == b'(' {
                            return true;
                        }

                        // Suffixed variants.
                        if bytes[j] == b'_' {
                            let starts_with_or = j + 3 <= bytes.len()
                                && bytes[j + 1] == b'o'
                                && bytes[j + 2] == b'r';

                            // Safe family: unwrap_or, unwrap_or_else, unwrap_or_default, etc.
                            if starts_with_or {
                                // Intentionally not a violation.
                            } else {
                                // Examples: unwrap_err(...)
                                let mut k = j + 1;

                                if k < bytes.len() && is_ident_start(bytes[k]) {
                                    k += 1;

                                    while k < bytes.len() && is_ident_continue(bytes[k]) {
                                        k += 1;
                                    }

                                    if k < bytes.len() && bytes[k] == b'(' {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            i += 1;
        }

        false
    }
}
```

---

# File: `crates/code-analysis/src/capabilities_mandatory_definition_checker.rs`

Replace the file content with:

```rust
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::{
    AesCodeAnalysisViolation, Language,
};
use shared::code_analysis::utility_bypass::skip_cfg_test_block;
use shared::code_analysis::utility_mandatory::rust_declares_type;
use shared::taxonomy_definition_vo::LayerDefinition;

// PURPOSE: MandatoryDefinitionChecker — AES303: enforce struct/enum/trait/class definitions exist AND are non-empty.
// Sub-check 1: file must define at least one struct/trait/enum/class (IMandatoryClassProtocol).
// Sub-check 2: empty unit struct (struct Foo;) and empty class (class Foo: pass, class Foo {}) flagged as dead inheritance.
//
// ALGORITHM (check_mandatory_class_definition):
//   1. Skip barrel/constant files (mod.rs, __init__.py, _constant.*)
//   2. If no LayerDefinition or mandatory_class_definition disabled → skip
//   3. Check if filename is in exception list
//   4. Scan passed content for class/struct/trait/enum keyword declarations
//   5. If none found → AES303 MANDATORY_DEFINITION
//
// ALGORITHM (check_dead_inheritance):
//   1. Iterate lines; skip #[cfg(test)] blocks correctly
//   2. For each `struct Foo;` (unit struct) → flag unless followed by impl block
//   3. For each `class Foo: pass` (Python empty class) → flag
//   4. For each `class Foo {}` or `interface Foo {}` (JS/TS empty declaration) → flag

use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MandatoryDefinitionChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

/// AES303 sub-check 2: detect empty struct/impl blocks (dead/empty definitions)
impl IDeadInheritanceProtocol for MandatoryDefinitionChecker {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let t = lines[i].trim();

            // Skip test modules correctly; do not skip the rest of the file.
            if t.starts_with("#[cfg(test)]") {
                i = skip_cfg_test_block(&lines, i);
                continue;
            }

            // Rust: unit struct `struct Foo;` or `pub struct Foo;`
            if Self::is_unit_struct_line(t) {
                // Skip if followed by impl block or attribute (intentional placeholder).
                let mut next_idx = i + 1;

                while next_idx < lines.len() {
                    let next_t = lines[next_idx].trim();
                    if next_t.is_empty() || next_t.starts_with('#') {
                        next_idx += 1;
                    } else {
                        break;
                    }
                }

                let next_is_impl = match lines.get(next_idx) {
                    Some(l) => l.trim().starts_with("impl "),
                    None => false,
                };

                if !next_is_impl {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                }

                i += 1;
                continue;
            }

            // Python: empty class.
            if Self::is_empty_python_class(&lines, i, t) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES303",
                    Severity::MEDIUM,
                    AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                ));
                i += 1;
                continue;
            }

            // JS/TS: empty class or interface.
            if Self::is_empty_js_declaration(t) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES303",
                    Severity::MEDIUM,
                    AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                ));
                i += 1;
                continue;
            }

            i += 1;
        }
    }
}

/// AES303 sub-check 1: file must have at least one struct/enum/trait/class definition
impl IMandatoryClassProtocol for MandatoryDefinitionChecker {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

        if matches!(
            basename.as_str(),
            "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs" | "main.rs"
        ) {
            return;
        }

        if basename.ends_with("_constant.rs") || basename.ends_with("_constant.py") {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if !def.code_analysis.mandatory_class_definition.value {
            return;
        }

        if def.exceptions.values.contains(&basename) {
            return;
        }

        let language = Language::from_file(file);
        let mut has_primary_symbol = false;

        for line in content.lines() {
            let t = line.trim();

            if t.is_empty() {
                continue;
            }

            let detected = match language {
                Language::Rust => rust_declares_type(t),
                Language::Python => Self::python_declares_class(t),
                Language::JavaScript | Language::TypeScript => {
                    Self::js_ts_declares_primary_symbol(t)
                }
            };

            if detected {
                has_primary_symbol = true;
                break;
            }
        }

        if !has_primary_symbol {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES303",
                Severity::HIGH,
                AesCodeAnalysisViolation::MandatoryClassDefinition { reason: None }.to_string(),
            ));
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for MandatoryDefinitionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryDefinitionChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Detect Rust unit structs, including visibility-qualified ones.
    fn is_unit_struct_line(line: &str) -> bool {
        let code = line
            .split_once("//")
            .map(|(code, _comment)| code)
            .unwrap_or(line);

        let stripped = Self::strip_rust_visibility(code);
        let tokens: Vec<&str> = stripped.split_whitespace().collect();

        tokens.first() == Some(&"struct") && stripped.ends_with(';') && !stripped.contains('(')
    }

    /// Remove common Rust visibility modifiers.
    fn strip_rust_visibility(line: &str) -> &str {
        let mut s = line;

        loop {
            let trimmed = s.trim_start();
            let mut changed = false;

            for prefix in ["pub(crate)", "pub(super)", "pub(self)", "pub"] {
                if let Some(rest) = trimmed.strip_prefix(prefix) {
                    s = rest.trim_start();
                    changed = true;
                    break;
                }
            }

            if !changed {
                return trimmed;
            }
        }
    }

    /// Detect Python empty class declarations.
    fn is_empty_python_class(lines: &[&str], idx: usize, line: &str) -> bool {
        let code = line.split_once('#').map(|(code, _comment)| code).trim_end();
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.first() != Some(&"class") {
            return false;
        }

        if code.ends_with(": pass") || code.ends_with(":pass") {
            return true;
        }

        if code.ends_with(':') {
            let mut next = idx + 1;

            while next < lines.len() {
                let nt = lines[next].trim();

                if nt.is_empty() || nt.starts_with('#') {
                    next += 1;
                    continue;
                }

                return nt == "pass" || nt == "..." || nt == "Ellipsis";
            }
        }

        false
    }

    /// Detect JS/TS empty class or interface declarations.
    fn is_empty_js_declaration(line: &str) -> bool {
        let code = line
            .split_once("//")
            .map(|(code, _comment)| code)
            .trim_end();

        let compact: String = code.split_whitespace().collect();

        compact.ends_with("{}") && Self::js_ts_declares_primary_symbol(code)
    }

    /// Detect Python class declarations.
    fn python_declares_class(line: &str) -> bool {
        let code = line.split_once('#').map(|(code, _comment)| code).trim_end();
        code.split_whitespace().next() == Some("class")
    }

    /// Detect JS/TS primary symbols: class or interface.
    fn js_ts_declares_primary_symbol(line: &str) -> bool {
        let code = line
            .split_once("//")
            .map(|(code, _comment)| code)
            .trim_end();

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if let Some(pos) = tokens
            .iter()
            .position(|tok| *tok == "class" || *tok == "interface")
        {
            if pos == 0 {
                return true;
            }

            return matches!(
                tokens[pos - 1],
                "export" | "default" | "abstract" | "declare"
            );
        }

        false
    }
}
```

---

# File: `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`

Replace the file content with:

```rust
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::config_system::taxonomy_config_vo::{default_aes_config, ArchitectureConfig};

// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Resolve target directory (default: ".")
//   2. Walk all lintable files via utility_target::collect_source_files (handles ignored patterns)
//   3. For each file, read content and tokenize into lines
//   4. Slide a window of `min_lines` over lines; normalize each window (trim, alphanumeric-only)
//   5. Intern normalized windows as u32 IDs and store per-file window IDs
//   6. Identify which normalized keys appear in 2+ files (shared keys)
//   7. For each file, calculate what % of its windows are shared
//   8. If a file's shared % exceeds `threshold_pct`, emit a single violation per file

use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CodeDuplicationAnalyzer {
    config: ArchitectureConfig,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(&self, path: Option<String>) -> Vec<AesCodeAnalysisViolation> {
        let root = shared::code_analysis::utility_target::resolve_target(path);
        let src =
            shared::code_analysis::utility_target::detect_source_dir(std::path::Path::new(&root));

        let config = &self.config;

        let ignored_vec: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();

        let min_lines = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .map(|r| r.code_analysis.min_lines.value as usize)
            .filter(|&v| v > 0)
            .unwrap_or(10);

        let threshold_pct = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .and_then(|r| r.code_analysis.duplication_threshold)
            .unwrap_or(50.0);

        let dir_path = match shared::common::taxonomy_path_vo::DirectoryPath::new(
            src.to_string_lossy().to_string(),
        ) {
            Ok(dp) => dp,
            Err(_) => return Vec::new(),
        };

        let source_files = shared::code_analysis::utility_target::collect_source_files(
            &src,
            &dir_path,
            &ignored_vec,
        );

        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();

        self.check_file_similarity(&file_strs, min_lines, threshold_pct)
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for CodeDuplicationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeDuplicationAnalyzer {
    pub fn new() -> Self {
        Self {
            config: default_aes_config(),
        }
    }

    pub fn from_config(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    /// Legacy per-block duplication detection.
    /// Kept for backward compatibility; prefer `check_file_similarity`.
    pub fn check_duplicates(
        &self,
        files: &[String],
        min_dup_lines: usize,
    ) -> Vec<AesCodeAnalysisViolation> {
        let entries = shared::code_analysis::utility_duplication::collect_file_entries(files);
        let total_loc = entries.iter().map(|(_, c)| c.lines().count()).sum();

        let blocks = shared::code_analysis::utility_duplication::scan_duplicate_blocks(
            entries,
            min_dup_lines,
        );

        shared::code_analysis::utility_duplication::build_violations(
            &blocks,
            total_loc,
            min_dup_lines,
        )
    }

    /// File-level similarity analysis using pre-read entries.
    ///
    /// Optimized version:
    /// - normalizes each window once,
    /// - stores per-file window IDs,
    /// - avoids unused interned string retention,
    /// - avoids second-pass normalization.
    pub fn check_file_similarity_entries(
        &self,
        entries: &[(String, String)],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        if entries.is_empty() || min_dup_lines == 0 {
            return Vec::new();
        }

        // Intern normalized window keys to u32 IDs.
        let mut interner: HashMap<String, u32> = HashMap::new();
        let mut next_id: u32 = 0;

        // Per-file window IDs.
        let mut file_windows: Vec<Vec<u32>> = Vec::with_capacity(entries.len());

        // Global map: window ID -> file indices where this window appears.
        let mut global: HashMap<u32, Vec<usize>> = HashMap::new();

        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            let mut windows: Vec<u32> = Vec::new();

            if lines.len() >= min_dup_lines {
                windows.reserve(lines.len() - min_dup_lines + 1);

                for w in lines.windows(min_dup_lines) {
                    let key = shared::code_analysis::utility_duplication::normalize_window(w);

                    let id = match interner.get(&key) {
                        Some(&id) => id,
                        None => {
                            let id = next_id;
                            next_id = next_id.saturating_add(1);
                            interner.insert(key, id);
                            id
                        }
                    };

                    windows.push(id);
                    global.entry(id).or_default().push(fi);
                }
            }

            file_windows.push(windows);
        }

        // Identify keys that appear in 2+ different files.
        let mut shared_ids: HashSet<u32> = HashSet::new();
        let mut file_to_others: Vec<HashSet<usize>> = vec![HashSet::new(); entries.len()];

        for (id, locs) in &global {
            // `locs` is ordered by file index because the outer loop processes files sequentially.
            // Adjacent deduplication is enough.
            let mut unique_files: Vec<usize> = Vec::new();
            let mut last: Option<usize> = None;

            for &fi in locs {
                if last != Some(fi) {
                    unique_files.push(fi);
                    last = Some(fi);
                }
            }

            if unique_files.len() <= 1 {
                continue;
            }

            shared_ids.insert(*id);

            for &fi in &unique_files {
                file_to_others[fi].extend(
                    unique_files
                        .iter()
                        .copied()
                        .filter(|&other| other != fi),
                );
            }
        }

        let mut violations = Vec::new();

        for (fi, (file_path, _content)) in entries.iter().enumerate() {
            let windows = &file_windows[fi];

            if windows.is_empty() {
                continue;
            }

            let total_win = windows.len();
            let shared_count = windows
                .iter()
                .filter(|id| shared_ids.contains(id))
                .count();

            let pct = shared_count as f64 / total_win as f64 * 100.0;

            if threshold_pct.is_finite() && pct > threshold_pct {
                let other_indices = &file_to_others[fi];

                let mut other_files: Vec<String> = other_indices
                    .iter()
                    .filter_map(|&ofi| entries.get(ofi).map(|e| e.0.clone()))
                    .collect();

                other_files.sort();

                let mut msg = format!(
                    "AES305: {:.0}% of this file's content appears in other files (threshold: {:.0}%). {} of {} windows are non-unique.",
                    pct,
                    threshold_pct,
                    shared_count,
                    total_win,
                );

                if !other_files.is_empty() {
                    msg.push_str(&format!(
                        " Similar files ({}): {}",
                        other_files.len(),
                        other_files
                            .iter()
                            .take(5)
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }

                violations.push((
                    file_path.clone(),
                    AesCodeAnalysisViolation::CodeDuplication {
                        reason: Some(LintMessage::new(msg)),
                    },
                ));
            }
        }

        violations
    }

    /// File-level similarity analysis (legacy API — reads files internally).
    /// Prefer `check_file_similarity_entries` to avoid double I/O.
    pub fn check_file_similarity(
        &self,
        files: &[String],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let entries = shared::code_analysis::utility_duplication::collect_file_entries(files);

        self.check_file_similarity_entries(
            &entries
                .iter()
                .map(|(p, c)| (p.display().to_string(), c.clone()))
                .collect::<Vec<_>>(),
            min_dup_lines,
            threshold_pct,
        )
    }
}
```

---

# File: `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`

Replace only the AES305 section inside `run_all_checks`.

Find this block:

```rust
        // AES305: File-level similarity check (run once across all files, using pre-read entries)
        let min_dup_lines: usize = 5;
        let threshold_pct: f64 = 50.0;
```

Replace it with:

```rust
        // AES305: File-level similarity check (run once across all files, using pre-read entries)
        let min_dup_lines: usize = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .map(|r| r.code_analysis.min_lines.value as usize)
            .filter(|&v| v > 0)
            .unwrap_or(5);

        let threshold_pct: f64 = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .and_then(|r| r.code_analysis.duplication_threshold)
            .unwrap_or(50.0);
```

Leave the rest of the orchestrator unchanged.

---

# File: `crates/code-analysis/src/root_code_analysis_container.rs`

In `CodeAnalysisCheckerContainer::new`, replace:

```rust
            code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::new()),
```

with:

```rust
            code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::from_config(config.clone())),
```

---

## 3. Summary of improvements

### Correctness improvements

- Fixed the reported Clippy issue in the bypass checker.
- Fixed multiline static initializer skipping.
- Fixed test-module skipping in dead-inheritance checking.
- Fixed detection of public Rust unit structs.
- Fixed detection of exported JS/TS empty classes.
- Fixed mandatory-definition detection for tuple structs.
- Fixed comment-based false positives in Rust definition detection.
- Fixed configuration-driven AES305 thresholds.
- Fixed runtime configuration propagation into duplication analysis.
- Added the intended fallback bypass-pattern list.
- Improved detection of inner attribute bypasses.
- Improved safe/unsafe unwrap distinction.

### Performance improvements

- Duplication detection now normalizes each window once.
- Removed unused interned-string storage.
- Reduced global duplication map payload.
- Avoided repeated lowercasing of bypass patterns per line.

These changes keep the module within the existing AES architecture and do not introduce new functionalit
