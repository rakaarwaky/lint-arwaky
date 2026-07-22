# Fix Plan — code-analysis crate (v1.10.106)

> Concrete implementation plan with exact before/after code for every fix.
> Validated against actual source. Each fix shows the exact lines to change.

---

## Resolved Ambiguities (from BA report)

| ID              | Question                                   | Decision                   | Rationale                                                 |
| --------------- | ------------------------------------------ | -------------------------- | --------------------------------------------------------- |
| AMB-02 / CON-01 | AES302 default: 10 lines or 0 (disabled)?  | **10 lines**               | Match FRD intent — files under 10 lines are suspicious    |
| AMB-04 / CON-04 | AES305: token/AST or window percentage?    | **Keep window percentage** | Fast, no new deps, catches ~80% of duplication            |
| AMB-06          | AES304 scope: 3 tokens or 6?               | **Keep all 6**             | todo/unimplemented/unreachable are real bypass risks      |
| AMB-05          | AES303 primary symbol: 4 types or 6?       | **Add type/interface**     | TypeScript `type` and `interface` are primary definitions |
| CON-05          | Clippy self-violation: exception or fix?   | **Fix the code**           | No special treatment — the linter must pass its own rules |
| R13             | Column numbers: remove claim or implement? | **Implement now**          | Byte-offset-to-column conversion is a small change        |

---

## Phase 0: Ambiguity Resolutions (P0 — New Fixes)

---

### P0.1 AES302 default threshold — change from 0 to 10

**File:** `crates/shared/src/config_system/taxonomy_config_vo.rs` (or wherever `CodeAnalysisRuleVO::default()` is defined)
**Bug:** `min_lines: Count::default()` resolves to 0, disabling AES302. FRD says 10.

**AFTER:** Change the default:

```rust
// BEFORE:
min_lines: Count::default(),  // 0

// AFTER:
min_lines: Count::new(10),    // Match FRD: "at least 10 lines"
```

If `Count::default()` is used elsewhere and can't change, override in `CodeAnalysisRuleVO::default()`:

```rust
impl Default for CodeAnalysisRuleVO {
    fn default() -> Self {
        Self {
            min_lines: Count::new(10),
            // ... other defaults
        }
    }
}
```

---

### P0.2 AES303 — add `type` and `interface` to primary symbol detection

**File:** `crates/shared/src/code-analysis/utility_mandatory.rs`

**BEFORE:**

```rust
pub fn rust_declares_type(line: &str) -> bool {
    let code = line.split_once("//").map(|(code, _)| code).unwrap_or(line);
    contains_word_token(code, "struct")
        || contains_word_token(code, "enum")
        || contains_word_token(code, "trait")
}
```

**AFTER:**

```rust
pub fn rust_declares_type(line: &str) -> bool {
    let code = line.split_once("//").map(|(code, _)| code).unwrap_or(line);
    contains_word_token(code, "struct")
        || contains_word_token(code, "enum")
        || contains_word_token(code, "trait")
        || contains_word_token(code, "type")
}
```

**Also update** `js_ts_declares_primary_symbol` in `capabilities_mandatory_definition_checker.rs`:

```rust
fn js_ts_declares_primary_symbol(line: &str) -> bool {
    let code = line.split_once("//").map(|(code, _)| code).unwrap_or(line);
    let tokens: Vec<&str> = code.split_whitespace().collect();
    if let Some(pos) = tokens.iter().position(|tok| {
        *tok == "class" || *tok == "interface" || *tok == "type"
    }) {
        if pos == 0 { return true; }
        return matches!(tokens[pos - 1], "export" | "default" | "abstract" | "declare");
    }
    false
}
```

**Also update** `check_mandatory_class_definition` to detect `type` and `interface`:

```rust
// BEFORE:
if trimmed.starts_with("class ")
    || trimmed.starts_with("export class ")
    || trimmed.starts_with("export default class ")
    || rust_declares_type(trimmed)

// AFTER:
if trimmed.starts_with("class ")
    || trimmed.starts_with("export class ")
    || trimmed.starts_with("export default class ")
    || trimmed.starts_with("interface ")
    || trimmed.starts_with("export interface ")
    || trimmed.starts_with("type ")
    || trimmed.starts_with("export type ")
    || rust_declares_type(trimmed)
```

---

### P0.3 Implement column number tracking

**File:** `crates/shared/src/cli_commands/taxonomy_result_vo.rs` (or wherever `LintResult::new_arch` is defined)

**BEFORE:**

```rust
pub fn new_arch(file: &str, line: usize, code: &str, sev: Severity, msg: String) -> Self {
    Self {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(line as i64),
        column: ColumnNumber::new(0),  // Always 0
        // ...
    }
}
```

**AFTER:** Add column parameter or compute from content:

```rust
pub fn new_arch(file: &str, line: usize, code: &str, sev: Severity, msg: String) -> Self {
    Self {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(line as i64),
        column: ColumnNumber::new(0),  // Default, callers can override
        // ...
    }
}

pub fn new_arch_with_column(
    file: str, line: usize, column: usize,
    code: &str, sev: Severity, msg: String,
) -> Self {
    Self {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(line as i64),
        column: ColumnNumber::new(column as i64),
        // ...
    }
}
```

**Also create** new utility file `crates/shared/src/code-analysis/utility_column.rs`:

```rust
// PURPOSE: Stateless utility functions for column position computation
// Pure functions only — no struct, no &self, no I/O

/// Compute 1-indexed column position of `pattern` in `line`.
/// Returns 0 if pattern not found.
pub fn compute_column(line: &str, pattern: &str) -> usize {
    line.find(pattern).map(|pos| pos + 1).unwrap_or(0)
}

/// Compute 1-indexed column position of a byte offset in a line.
pub fn byte_offset_to_column(line: &str, offset: usize) -> usize {
    line[..offset.min(line.len())].chars().count() + 1
}
```

**Register** in `crates/shared/src/code-analysis/mod.rs`:

```rust
pub mod utility_column;
```

**Then update** callers in `capabilities_check_bypass_checker.rs` to pass column:

```rust
// BEFORE:
violations.push(LintResult::new_arch(file, i + 1, "AES304", Severity::CRITICAL, msg));

// AFTER:
let col = shared::code_analysis::utility_column::compute_column(line, pattern);
violations.push(LintResult::new_arch_with_column(file, i + 1, col, "AES304", Severity::CRITICAL, msg));
```

---

## Phase 1: Correctness Bugs (P0)

---

### P1.1 `#[cfg(test)]` skip never resets — skips entire file after first test module

**File:** `crates/code-analysis/src/capabilities_mandatory_definition_checker.rs`
**Lines:** 36-48

**BEFORE (broken):**

```rust
let mut in_test_module = false;
while i < lines.len() {
    let t = lines[i].trim();
    // Skip test modules
    if t.starts_with("#[cfg(test)]") {
        in_test_module = true;
        i += 1;
        continue;
    }
    if in_test_module {
        i += 1;
        continue;
    }
```

**AFTER (fixed):**

```rust
while i < lines.len() {
    let t = lines[i].trim();
    // Skip test modules — advance past the entire module block
    if t.starts_with("#[cfg(test)]") {
        i = skip_cfg_test_block(&lines, i);
        continue;
    }
```

**Also add** `use shared::code_analysis::utility_bypass::skip_cfg_test_block;` to imports.

**Also add** the `skip_cfg_test_block` function to `utility_bypass.rs` (see P1.8 below).

---

### P1.2 `clippy::collapsible_match` — nested if inside for loop

**File:** `crates/code-analysis/src/capabilities_check_bypass_checker.rs`
**Lines:** 109-153

**BEFORE (broken):**

```rust
for p in &patterns.values {
    let p_str = p.as_str();
    if matches!(
        p_str,
        "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable"
    ) {
        if matches_word_token(t, p_str, matches!(p_str, "unwrap" | "expect")) {
            // Safe .unwrap_or*() variants don't panic — skip.
            if p_str == "unwrap" && t.contains(".unwrap_or") {
                continue;
            }
            let vo = match Self::classify_token(p_str) {
                // ... build vo ...
            };
            violations.push(LintResult::new_arch(
                file, i + 1, "AES304", Severity::CRITICAL, vo.to_string(),
            ));
            break;
        }
    } else if !p_str.is_empty() && t_lower.contains(&p_str.to_lowercase()) {
        violations.push(LintResult::new_arch(
            file, i + 1, "AES304", Severity::CRITICAL,
            AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
        ));
        break;
    }
}
```

**AFTER (fixed):**

```rust
for p in &patterns.values {
    let p_str = p.as_str();
    match p_str {
        p if is_word_pattern_token(p)
            && matches_word_token(t, p, matches!(p, "unwrap" | "expect"))
            && !(p == "unwrap" && contains_unsafe_unwrap(t)) =>
        {
            let vo = match Self::classify_token(p) {
                ViolationKind::UnwrapExpect => AesCodeAnalysisViolation::UnwrapExpect { reason: None },
                ViolationKind::Panic => AesCodeAnalysisViolation::Panic { reason: None },
                ViolationKind::Todo => AesCodeAnalysisViolation::Todo { reason: None },
                ViolationKind::Unimplemented => AesCodeAnalysisViolation::Unimplemented { reason: None },
                ViolationKind::BypassComment => AesCodeAnalysisViolation::BypassComment { reason: None },
            };
            violations.push(LintResult::new_arch(
                file, i + 1, "AES304", Severity::CRITICAL, vo.to_string(),
            ));
            break;
        }
        p if !is_word_pattern_token(p)
            && !p.is_empty()
            && t_lower.contains(&p.to_lowercase()) =>
        {
            violations.push(LintResult::new_arch(
                file, i + 1, "AES304", Severity::CRITICAL,
                AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
            ));
            break;
        }
        _ => {}
    }
}
```

**Also add** these helper functions to `BypassChecker` impl block:

```rust
fn is_word_pattern_token(token: &str) -> bool {
    matches!(token, "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable")
}

fn contains_unsafe_unwrap(line: &str) -> bool {
    // Position-aware: detect .unwrap( but NOT .unwrap_or( / .unwrap_or_else( / .unwrap_or_default(
    let bytes = line.as_bytes();
    let token = b"unwrap";
    let mut i = 0;
    while i + token.len() <= bytes.len() {
        if &bytes[i..i + token.len()] == token {
            if i > 0 && bytes[i - 1] == b'.' {
                let mut j = i + token.len();
                if j < bytes.len() {
                    if bytes[j] == b'(' { return true; }
                    if bytes[j] == b'_' {
                        // Check if it's unwrap_or* (safe) or unwrap_err* (unsafe)
                        if j + 3 <= bytes.len() && bytes[j+1] == b'o' && bytes[j+2] == b'r' {
                            // safe family — skip
                        } else {
                            // e.g. unwrap_err — find closing paren
                            let mut k = j + 1;
                            if k < bytes.len() && is_ident_start(bytes[k]) {
                                k += 1;
                                while k < bytes.len() && is_ident_continue(bytes[k]) { k += 1; }
                                if k < bytes.len() && bytes[k] == b'(' { return true; }
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
```

**Also add** `use shared::code_analysis::utility_bypass::{is_ident_continue, is_ident_start};` to imports.

---

### P1.3 `Lazy` multiline block skip — doesn't advance line index

**File:** `crates/code-analysis/src/capabilities_check_bypass_checker.rs`
**Lines:** 71-93

**BEFORE (broken):**

```rust
let language = Language::from_file(file);
for (i, line) in content.lines().enumerate() {
    let t = line.trim();
    // ...
    // Skip static Lazy<Regex> initialization (multiline)
    if t.contains("static ") && t.contains("Lazy") {
        let depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
        let mut d = depth;
        for subsequent_line in content.lines().skip(i + 1) {
            let st = subsequent_line.trim();
            d += st.matches('{').count() as i32 - st.matches('}').count() as i32;
            if d <= 0 { break; }
        }
        continue;  // BUG: outer for loop still processes next line
    }
```

**AFTER (fixed):** Convert to `while` loop with index advancement:

```rust
let language = Language::from_file(file);
let lines: Vec<&str> = content.lines().collect();
let mut i = 0;
while i < lines.len() {
    let t = lines[i].trim();
    // ...
    // Skip static Lazy<Regex> initialization (multiline)
    if t.contains("static ") && t.contains("Lazy") {
        i = skip_brace_block(&lines, i);
        continue;
    }
```

**Also add** `skip_brace_block` to `utility_bypass.rs`:

```rust
pub fn skip_brace_block(lines: &[&str], start: usize) -> usize {
    if start >= lines.len() { return start; }
    let mut depth = lines[start].matches('{').count() as i32
        - lines[start].matches('}').count() as i32;
    let mut idx = start + 1;
    if depth <= 0 { return idx; }
    while idx < lines.len() {
        depth += lines[idx].matches('{').count() as i32
            - lines[idx].matches('}').count() as i32;
        idx += 1;
        if depth <= 0 { break; }
    }
    idx
}
```

---

### P1.4 `rust_declares_type` — substring match + rejects tuple structs

**File:** `crates/shared/src/code-analysis/utility_mandatory.rs`
**Lines:** 5-12

**BEFORE (broken):**

```rust
pub fn rust_declares_type(line: &str) -> bool {
    let keywords = ["struct", "enum", "trait"];
    for kw in keywords {
        if line.contains(kw) && !line.contains('(') {
            return true;
        }
    }
    false
}
```

**AFTER (fixed):**

```rust
pub fn rust_declares_type(line: &str) -> bool {
    let code = line.split_once("//").map(|(code, _)| code).unwrap_or(line);
    contains_word_token(code, "struct")
        || contains_word_token(code, "enum")
        || contains_word_token(code, "trait")
}

fn contains_word_token(haystack: &str, word: &str) -> bool {
    haystack
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|token| token == word)
}
```

**Why:** `line.contains("struct")` matches "obstruction", "structure". `!line.contains('(')` rejects `pub struct Point(i64, i64);` which is a valid definition. The fix uses word-boundary splitting and allows tuple structs.

---

### P1.5 AES305 thresholds hardcoded in orchestrator

**File:** `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`
**Lines:** 233-235

**BEFORE (broken):**

```rust
// AES305: File-level similarity check (run once across all files, using pre-read entries)
let min_dup_lines: usize = 5;
let threshold_pct: f64 = 50.0;
```

**AFTER (fixed):**

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

---

### P1.6 `CodeDuplicationAnalyzer::handle_duplicates` uses `default_aes_config()`

**File:** `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`
**Lines:** 21, 26-30, 72-76

**BEFORE (broken):**

```rust
pub struct CodeDuplicationAnalyzer {}

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(&self, path: Option<String>) -> Vec<AesCodeAnalysisViolation> {
        // ...
        let config = default_aes_config();  // BUG: ignores project config
```

**AFTER (fixed):**

```rust
pub struct CodeDuplicationAnalyzer {
    config: ArchitectureConfig,
}

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(&self, path: Option<String>) -> Vec<AesCodeAnalysisViolation> {
        // ...
        let config = &self.config;  // Uses injected config
```

**Also change constructor:**

```rust
impl CodeDuplicationAnalyzer {
    pub fn new() -> Self {
        Self { config: default_aes_config() }
    }
    pub fn from_config(config: ArchitectureConfig) -> Self {
        Self { config }
    }
}
```

**Also update** `root_code_analysis_container.rs` line 48:

```rust
// BEFORE:
code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::new()),
// AFTER:
code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::from_config(config.clone())),
```

---

### P1.7 Missing fallback bypass pattern list

**File:** `crates/code-analysis/src/capabilities_check_bypass_checker.rs`
**Lines:** 211-214

**BEFORE (broken):**

```rust
pub fn new() -> Self {
    Self {
        rule: CodeAnalysisRuleVO::default(),  // forbidden_bypass is empty
    }
}
```

**AFTER (fixed):**

```rust
pub fn new() -> Self {
    Self {
        rule: CodeAnalysisRuleVO {
            forbidden_bypass: Self::default_forbidden_bypass(),
            ..CodeAnalysisRuleVO::default()
        },
    }
}

fn default_forbidden_bypass() -> PatternList {
    PatternList::new(vec![
        "unwrap", "expect", "panic", "todo", "unimplemented", "unreachable",
        "noqa", "type: ignore", "@ts-ignore", "@ts-expect-error",
        "eslint-disable", "eslint-disable-next-line",
    ])
}
```

**Also update** `from_rule` to apply fallback when empty:

```rust
pub fn from_rule(rule: CodeAnalysisRuleVO) -> Self {
    let mut rule = rule;
    if rule.forbidden_bypass.is_empty() {
        rule.forbidden_bypass = Self::default_forbidden_bypass();
    }
    Self { rule }
}
```

---

### P1.8 Inner attribute bypass detection incomplete

**File:** `crates/shared/src/code-analysis/utility_bypass.rs`
**Lines:** 16-26

**BEFORE (broken):**

```rust
pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: std::sync::OnceLock<[String; 2]> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let a: String = ['#', '[', 'a', 'l', 'l', 'o', 'w', '('].iter().collect();
        let e: String = ['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '('].iter().collect();
        [a, e]
    });
    line.starts_with(&prefixes[0]) || line.starts_with(&prefixes[1])
}
```

**AFTER (fixed):**

```rust
pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let mk = |chars: &[char]| chars.iter().collect::<String>();
        vec![
            mk(&['#', '[', 'a', 'l', 'l', 'o', 'w', '(']),       // #[allow(
            mk(&['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']),   // #[expect(
            mk(&['#', '[', 'w', 'a', 'r', 'n', '(']),              // #[warn(
            mk(&['#', '!', '[', 'a', 'l', 'l', 'o', 'w', '(']),   // #![allow(
            mk(&['#', '!', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']), // #![expect(
            mk(&['#', '!', '[', 'w', 'a', 'r', 'n', '(']),        // #![warn(
            mk(&['#', '[', 'c', 'l', 'i', 'p', 'p', 'y', ':', ':', 'a', 'l', 'l', 'o', 'w', '(']), // #[clippy::allow(
        ]
    });
    prefixes.iter().any(|prefix| line.starts_with(prefix))
}
```

**Also add** `skip_brace_block` and `skip_cfg_test_block` functions (see P1.1 and P1.3).

---

### P1.9 Safe/unsafe unwrap — line-level contains suppresses whole line

**File:** `crates/code-analysis/src/capabilities_check_bypass_checker.rs`
**Line:** 117

**BEFORE (broken):**

```rust
if p_str == "unwrap" && t.contains(".unwrap_or") {
    continue;  // BUG: if line has both .unwrap_or AND .unwrap, both suppressed
}
```

**AFTER (fixed):** Use the `contains_unsafe_unwrap` function from P1.2:

```rust
p if is_word_pattern_token(p)
    && matches_word_token(t, p, matches!(p, "unwrap" | "expect"))
    && !(p == "unwrap" && contains_unsafe_unwrap(t)) =>
{
    // Only match if there's an actual unsafe .unwrap( call
```

This is already handled in the P1.2 fix above — the `contains_unsafe_unwrap` function does position-aware scanning.

---

### P1.10 Public unit struct detection misses visibility modifiers

**File:** `crates/code-analysis/src/capabilities_mandatory_definition_checker.rs`
**Line:** 50

**BEFORE (broken):**

```rust
if t.starts_with("struct ") && t.ends_with(';') && !t.contains('(') {
```

**AFTER (fixed):**

```rust
if Self::is_unit_struct_line(t) {
```

**Add helper:**

```rust
fn is_unit_struct_line(line: &str) -> bool {
    let code = line.split_once("//").map(|(code, _)| code).unwrap_or(line);
    let stripped = Self::strip_rust_visibility(code);
    let tokens: Vec<&str> = stripped.split_whitespace().collect();
    tokens.first() == Some(&"struct") && stripped.ends_with(';') && !stripped.contains('(')
}

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
        if !changed { return trimmed; }
    }
}
```

---

### P1.11 Exported JS/TS empty class detection

**File:** `crates/code-analysis/src/capabilities_mandatory_definition_checker.rs`
**Line:** 101

**BEFORE (broken):**

```rust
if t.starts_with("class ") && t.ends_with("{}") {
```

**AFTER (fixed):**

```rust
if Self::is_empty_js_declaration(t) {
```

**Add helper:**

```rust
fn is_empty_js_declaration(line: &str) -> bool {
    let code = line.split_once("//").map(|(code, _)| code).unwrap_or(line);
    let compact: String = code.split_whitespace().collect();
    compact.ends_with("{}") && Self::js_ts_declares_primary_symbol(code)
}

fn js_ts_declares_primary_symbol(line: &str) -> bool {
    let code = line.split_once("//").map(|(code, _)| code).unwrap_or(line);
    let tokens: Vec<&str> = code.split_whitespace().collect();
    if let Some(pos) = tokens.iter().position(|tok| *tok == "class" || *tok == "interface") {
        if pos == 0 { return true; }
        return matches!(tokens[pos - 1], "export" | "default" | "abstract" | "declare");
    }
    false
}
```

---

## Phase 2: Performance (P1)

---

### P2.1 Duplication — normalize each window twice

**File:** `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`
**Lines:** 180-188

**BEFORE (slow):**

```rust
// Second pass re-normalizes every window
let shared_count = lines
    .windows(min_dup_lines)
    .enumerate()
    .filter(|(_, w)| {
        let key = shared::code_analysis::utility_duplication::normalize_window(w);
        let id = get_id(key);
        shared_ids.contains(&id)
    })
    .count();
```

**AFTER (fast):** Store per-file window IDs during first pass:

```rust
// First pass: store per-file window IDs
let mut file_windows: Vec<Vec<u32>> = Vec::with_capacity(entries.len());
for (fi, (_, content)) in entries.iter().enumerate() {
    let lines: Vec<&str> = content.lines().collect();
    let mut windows: Vec<u32> = Vec::new();
    if lines.len() >= min_dup_lines {
        for w in lines.windows(min_dup_lines) {
            let key = shared::code_analysis::utility_duplication::normalize_window(w);
            let id = get_id(key);
            windows.push(id);
            global.entry(id).or_default().push((fi, 0)); // line number not needed
        }
    }
    file_windows.push(windows);
}

// Second pass: use stored IDs, no re-normalization
let shared_count = file_windows[fi].iter().filter(|id| shared_ids.contains(id)).count();
```

---

### P2.2 Remove unused `interned_keys`

**File:** `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`
**Line:** 122

**BEFORE:**

```rust
let mut interned_keys: Vec<String> = Vec::new();
```

**AFTER:** Delete this line and remove `interned_keys.push(key.clone());` from the closure.

---

### P2.3 Store only file indices, not (file, line) tuples

**File:** `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`
**Line:** 134

**BEFORE:**

```rust
let mut global: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
```

**AFTER:**

```rust
let mut global: HashMap<u32, Vec<usize>> = HashMap::new();
```

And change push from `.push((fi, li + 1))` to `.push(fi)`.

---

### P2.4 Repeated pattern lowercasing per line

**File:** `crates/code-analysis/src/capabilities_check_bypass_checker.rs`
**Line:** 144

**BEFORE:**

```rust
} else if !p_str.is_empty() && t_lower.contains(&p_str.to_lowercase()) {
```

**AFTER:** Precompute once before the line loop:

```rust
let lowered_patterns: Vec<String> = patterns.values.iter().map(|p| p.to_lowercase()).collect();
// Then in the loop:
} else if !p_str.is_empty() && t_lower.contains(&lowered_patterns[idx]) {
```

---

### P2.5 Remove whole-file lowercase allocation

**File:** `crates/code-analysis/src/capabilities_check_bypass_checker.rs`
**Lines:** 59-68

**BEFORE:**

```rust
let content_lower = content.to_lowercase();
let has_bypass_token = patterns.values.iter()
    .any(|p| content_lower.contains(p.as_str()))
    || content_lower.contains("raise ")
    || content_lower.contains("throw new");
if !has_bypass_token { return; }
```

**AFTER:** Remove the whole-file allocation. The per-line `t_lower` check is sufficient:

```rust
// Remove the early-exit block entirely — per-line scanning handles this
```

---

## Phase 3: Architecture (P1)

---

### P3.1 Remove duplicated utility functions from agent layer

**File:** `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`
**Lines:** 43-84

**BEFORE:** Agent defines `detect_source_dir`, `collect_source_files`, `resolve_target`.

**AFTER:** Delete these 3 functions. They already exist in `shared::code_analysis::utility_target`. Update callers to use the shared versions.

---

### P3.2 Agent layer does filesystem I/O — move to utility

**AES VIOLATION:** Agent layer must have zero I/O (create-agent-rust skill rule).

**Step 1:** Create `crates/shared/src/code-analysis/utility_file_reader.rs`:

```rust
// PURPOSE: Stateless utility functions for reading lintable files
// Domain-agnostic, reusable — valid utility per ARCHITECTURE §7

/// Maximum file size for linting (2 MiB).
pub const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024;

/// Read a file for linting. Returns:
/// - Ok(Some(content)) if file is readable and within size limit
/// - Ok(None) if file exceeds size limit
/// - Err(message) if file is unreadable
pub fn read_lintable_file(path: &str) -> Result<Option<String>, String> {
    let meta = std::fs::metadata(path).map_err(|e| format!("{}: {}", path, e))?;
    if meta.len() > MAX_LINT_FILE_BYTES {
        return Ok(None);
    }
    std::fs::read_to_string(path)
        .map(Some)
        .map_err(|e| format!("{}: {}", path, e))
}
```

**Step 2:** Register in `crates/shared/src/code-analysis/mod.rs`:

```rust
pub mod utility_file_reader;
```

**Step 3:** Update `agent_code_analysis_orchestrator.rs` to call utility:

```rust
// BEFORE (agent does I/O — VIOLATES AES):
let c = match std::fs::read_to_string(file) {
    Ok(content) => content,
    Err(_) => continue,
};

// AFTER (agent delegates to utility — COMPLIES with AES):
let c = match shared::code_analysis::utility_file_reader::read_lintable_file(file) {
    Ok(Some(content)) => content,
    Ok(None) => {
        violations.push(LintResult::new_arch(file, 0, "AES000", Severity::LOW,
            "File skipped: exceeds maximum lintable size (2 MiB)".to_string()));
        continue;
    }
    Err(e) => {
        violations.push(LintResult::new_arch(file, 0, "AES000", Severity::LOW,
            format!("File skipped: {}", e)));
        continue;
    }
};
```

**Also update** Cargo.toml reading:

```rust
// BEFORE:
if let Ok(cargo_content) = std::fs::read_to_string(cargo_path) {

// AFTER:
if let Ok(Some(cargo_content)) = shared::code_analysis::utility_file_reader::read_lintable_file(
    &cargo_path.to_string_lossy()
) {
```

---

### P3.3 Global container state

**File:** `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`
**Lines:** 35-40, 106-113

**BEFORE:**

```rust
static GLOBAL_CONTAINER: OnceLock<Arc<CodeAnalysisCheckerContainer>> = OnceLock::new();
pub fn init_global_checker(container: Arc<CodeAnalysisCheckerContainer>) {
    GLOBAL_CONTAINER.set(container).ok();
}
impl CodeAnalysisOrchestrator {
    pub fn new() -> Self {
        Self {
            container: match GLOBAL_CONTAINER.get().cloned() {
                Some(c) => c,
                None => Arc::new(CodeAnalysisCheckerContainer::default()),
            },
        }
    }
```

**AFTER:** Deprecate global, prefer injection:

```rust
#[deprecated(note = "Inject CodeAnalysisCheckerContainer explicitly")]
static GLOBAL_CONTAINER: OnceLock<Arc<CodeAnalysisCheckerContainer>> = OnceLock::new();

impl CodeAnalysisOrchestrator {
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self {
            container: match GLOBAL_CONTAINER.get().cloned() {
                Some(c) => c,
                None => Arc::new(CodeAnalysisCheckerContainer::default()),
            },
        }
    }
```

Keep `new_with_container` as the preferred constructor.

---

## Phase 4: Security & Error Handling (P1)

---

### P4.1 Path traversal / symlink escape

**File:** `crates/shared/src/common/utility_file.rs` (in `walk_source_files_inner`)

**AFTER:** Add root confinement check before recursing into symlinked dirs:

```rust
if let Ok(target) = std::fs::canonicalize(&path) {
    // Prevent symlink escape
    if !target.starts_with(&root) {
        continue;
    }
    if target_meta.is_dir() {
        walk_source_files_inner(&target, root, files, ignored, visited);
    }
}
```

---

### P4.2 No file-size limit

Already covered in P3.2 — the `read_lintable_file` helper enforces 2 MiB limit.

---

### P4.3 Silent error handling

Already covered in P3.2 — `read_lintable_file` returns `Err` which emits `AES000` diagnostic.

---

## Phase 5: Documentation (P2)

### P5.1-P5.5

Update FRD with acceptance criteria, language parity matrix, default patterns, resolve CON-01/CON-04/CON-05 contradictions, add traceability matrix. (Documentation-only, no code changes.)

---

## Phase 6: Testability (P2)

### P6.1 Fix `active_rules()` returning empty

**File:** `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`
**Lines:** 297-299

**BEFORE:**

```rust
fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
    Vec::new()
}
```

**AFTER:**

```rust
fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
    self.container.config().rules.iter()
        .map(|r| r.code_analysis.clone())
        .collect()
}
```

---

## Execution Order

```
Phase 0 (ambiguity resolutions):
  0. taxonomy_config_vo.rs        (P0.1 — AES302 default to 10)
  1. utility_mandatory.rs         (P0.2 — add type/interface)
  2. utility_column.rs [NEW]      (P0.3 — column computation)
  3. taxonomy_result_vo.rs        (P0.3 — new_arch_with_column)
  4. mandatory_definition_checker.rs (P0.2 — interface/type detection)

Phase 1 (correctness bugs):
  5. utility_mandatory.rs         (P1.4 — word-token matching)
  6. utility_bypass.rs            (P1.8 + skip_brace_block + skip_cfg_test_block)
  7. mandatory_definition_checker.rs (P1.1, P1.10, P1.11)
  8. bypass_checker.rs            (P1.2, P1.3, P1.7, P1.9, P2.4, P2.5)
  9. duplication_analyzer.rs      (P1.6, P2.1, P2.2, P2.3)
 10. orchestrator.rs              (P1.5, P3.1, P3.3, P6.1)

Phase 3 (architecture — AES compliance):
 11. utility_file_reader.rs [NEW] (P3.2 — move I/O from agent to utility)
 12. orchestrator.rs              (P3.2 — call utility instead of direct I/O)
 13. root_container.rs            (P1.6 wiring)

Phase 5 (FRD updates — after code changes):
 14. Update FRD: AES302 default=10, AES305=window%, AES304=6 tokens,
      AES303=type+interface, column tracking, traceability matrix
```

---

_Generated: 2026-07-21 | 33 items (3 new from ambiguity resolution) | 9 files to modify (2 new utility files)_
