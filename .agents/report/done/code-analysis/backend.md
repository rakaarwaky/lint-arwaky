
Hi Raka — below is a backend-focused review of the `code-analysis` crate and its related `shared` contracts/utilities from the uploaded knowledge base.

I focused on:

- API design
- error handling
- business logic correctness
- security vulnerabilities
- performance bottlenecks
- architecture / AES layer compliance
- scalability and maintainability
- SOLID principles

> Note: this crate does not contain database access, so there are no SQL/query issues to review directly. I included database guidance at the end for future persistence of scan results.

---

# 1. Executive Summary

The crate implements AES301–AES305 code-quality checks:

- AES301: file too large
- AES302: file too short
- AES303: mandatory definitions / dead inheritance
- AES304: bypass/panic detection
- AES305: duplication detection

The overall domain separation is good: `taxonomy`, `contract`, `capabilities`, `agent`, and `root` layers are mostly visible. However, there are several important issues:

| Priority | Issue                                                                                                                                                                     |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Critical | `clippy::collapsible_match` violation in `capabilities_check_bypass_checker.rs`                                                                                       |
| Critical | Bypass checker has correctness bugs: multiline`static Lazy` skip does not actually skip, incomplete attribute bypass detection, false-negative risk with `.unwrap_or` |
| High     | Duplication analyzer can consume unbounded memory and CPU on large codebases                                                                                              |
| High     | Orchestrator hardcodes AES305 thresholds instead of honoring configuration                                                                                                |
| High     | Agent layer performs filesystem I/O and utility work, violating AES layer boundaries                                                                                      |
| High     | Global container state makes the system harder to test and violates dependency inversion                                                                                  |
| Medium   | Path traversal / symlink escape risk in file collection and target resolution                                                                                             |
| Medium   | Error handling silently swallows unreadable files, invalid paths, and config parse failures                                                                               |
| Medium   | Mandatory definition / dead inheritance checks have false positives/negatives                                                                                             |
| Low      | Public API mixes orchestration, scoring, and reporting responsibilities                                                                                                   |
| Low      | Some duplicated utility functions exist in agent and shared utility modules                                                                                               |

---

# 2. Prioritized Findings

## 2.1 Critical: Clippy violation in `capabilities_check_bypass_checker.rs`

The reported violation is:

```text
[clippy::collapsible_match] crates/code-analysis/src/capabilities_check_bypass_checker.rs
this `if` can be collapsed into the outer `match`
```

The problematic logic is around forbidden bypass pattern matching:

```rust
if matches!(
    p_str,
    "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable"
) {
    if matches_word_token(t, p_str, matches!(p_str, "unwrap" | "expect")) {
        // ...
    }
} else if !p_str.is_empty() && t_lower.contains(&p_str.to_lowercase()) {
    // ...
}
```

This nested conditional can be collapsed and made clearer.

More importantly, the logic must preserve this behavior:

- If `p_str` is a Rust word token such as `unwrap`, `expect`, `panic`, etc., it should only match as a real token/method/macro.
- It should not fall back to naive substring matching.
- Non-word bypass patterns may use case-insensitive substring matching.

### Fixed version

Replace the pattern-matching block inside `check_bypass_comments` with this:

```rust
// crates/code-analysis/src/capabilities_check_bypass_checker.rs

fn is_rust_word_pattern(token: &str) -> bool {
    matches!(
        token,
        "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable"
    )
}
```

Then inside the loop:

```rust
for p in &patterns.values {
    let p_str = p.as_str();

    match p_str {
        p if is_rust_word_pattern(p)
            && matches_word_token(t, p, matches!(p, "unwrap" | "expect"))
            && !(p == "unwrap" && t.contains(".unwrap_or")) =>
        {
            let vo = match Self::classify_token(p) {
                ViolationKind::UnwrapExpect => {
                    AesCodeAnalysisViolation::UnwrapExpect { reason: None }
                }
                ViolationKind::Panic => {
                    AesCodeAnalysisViolation::Panic { reason: None }
                }
                ViolationKind::Todo => {
                    AesCodeAnalysisViolation::Todo { reason: None }
                }
                ViolationKind::Unimplemented => {
                    AesCodeAnalysisViolation::Unimplemented { reason: None }
                }
                ViolationKind::BypassComment => {
                    AesCodeAnalysisViolation::BypassComment { reason: None }
                }
            };

            violations.push(LintResult::new_arch(
                file,
                i + 1,
                "AES304",
                Severity::CRITICAL,
                vo.to_string(),
            ));

            break;
        }

        p if !is_rust_word_pattern(p)
            && !p.is_empty()
            && t_lower.contains(&p.to_lowercase()) =>
        {
            violations.push(LintResult::new_arch(
                file,
                i + 1,
                "AES304",
                Severity::CRITICAL,
                AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
            ));

            break;
        }

        _ => {}
    }
}
```

This removes the nested `if`, satisfies the clippy intent, and preserves the original semantics.

---

## 2.2 Critical: Bypass checker multiline skip is broken

This block is intended to skip multiline `static Lazy<Regex>` initializations:

```rust
if t.contains("static ") && t.contains("Lazy") {
    let depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
    let mut d = depth;
    for subsequent_line in content.lines().skip(i + 1) {
        let st = subsequent_line.trim();
        d += st.matches('{').count() as i32 - st.matches('}').count() as i32;
        if d <= 0 {
            break;
        }
    }
    continue;
}
```

The problem: the outer loop is a `for (i, line) in content.lines().enumerate()` loop. Computing a future end index does not actually skip subsequent lines. The next iteration still checks the inside of the `static Lazy` block.

That can produce false positives inside static initializers.

### Fix: use indexed iteration and real block skipping

Convert the line loop to a `while` loop and add a helper:

```rust
// crates/code-analysis/src/capabilities_check_bypass_checker.rs

fn skip_brace_block(lines: &[&str], start: usize) -> usize {
    let mut depth: i32 = 0;
    let mut found_open = false;

    for idx in start..lines.len() {
        for ch in lines[idx].chars() {
            match ch {
                '{' => {
                    depth += 1;
                    found_open = true;
                }
                '}' => {
                    depth -= 1;
                }
                _ => {}
            }
        }

        if found_open && depth <= 0 {
            return idx + 1;
        }
    }

    if found_open {
        lines.len()
    } else {
        start + 1
    }
}
```

Then rewrite the main loop:

```rust
fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
    let patterns = &self.rule.forbidden_bypass;
    let language = Language::from_file(file);

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let t = lines[i].trim();

        // Documentation references are not runtime violations.
        if t.starts_with("///") || t.starts_with("//!") {
            i += 1;
            continue;
        }

        // Skip test modules.
        if t.starts_with("#[cfg(test)]") {
            i = skip_brace_block(&lines, i);
            continue;
        }

        // Skip static Lazy<Regex> initialization blocks.
        if t.contains("static ") && t.contains("Lazy") {
            i = skip_brace_block(&lines, i);
            continue;
        }

        // Compiler attribute bypasses.
        if starts_with_allow_attr(t) {
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

        for p in &patterns.values {
            let p_str = p.as_str();

            match p_str {
                p if is_rust_word_pattern(p)
                    && matches_word_token(t, p, matches!(p, "unwrap" | "expect"))
                    && !(p == "unwrap" && t.contains(".unwrap_or")) =>
                {
                    let vo = match Self::classify_token(p) {
                        ViolationKind::UnwrapExpect => {
                            AesCodeAnalysisViolation::UnwrapExpect { reason: None }
                        }
                        ViolationKind::Panic => {
                            AesCodeAnalysisViolation::Panic { reason: None }
                        }
                        ViolationKind::Todo => {
                            AesCodeAnalysisViolation::Todo { reason: None }
                        }
                        ViolationKind::Unimplemented => {
                            AesCodeAnalysisViolation::Unimplemented { reason: None }
                        }
                        ViolationKind::BypassComment => {
                            AesCodeAnalysisViolation::BypassComment { reason: None }
                        }
                    };

                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES304",
                        Severity::CRITICAL,
                        vo.to_string(),
                    ));

                    break;
                }

                p if !is_rust_word_pattern(p)
                    && !p.is_empty()
                    && t_lower.contains(&p.to_lowercase()) =>
                {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES304",
                        Severity::CRITICAL,
                        AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                    ));

                    break;
                }

                _ => {}
            }
        }

        // Language-scoped phrase patterns.
        match language {
            Language::Python => {
                if t_lower.contains("raise notimplementederror")
                    || t_lower.contains("raise notimplemented")
                {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES304",
                        Severity::CRITICAL,
                        AesCodeAnalysisViolation::Unimplemented { reason: None }.to_string(),
                    ));
                } else if t_lower.contains("assert false") {
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
                if t_lower.contains("throw new error")
                    || t_lower.contains("throw new typeerror")
                    || t_lower.contains("throw new rangeerror")
                    || t_lower.contains("throw new referenceerror")
                    || t_lower.contains("throw new syntaxerror")
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

            _ => {}
        }

        i += 1;
    }
}
```

Also remove the whole-file early-exit lowercase allocation:

```rust
let content_lower = content.to_lowercase();
```

It allocates a lowercased copy of the entire file. For large files this is expensive. If you need an early exit, use a proper substring search library such as `aho-corasick` or `memchr`, or simply scan line-by-line as above.

---

## 2.3 Critical: Incomplete compiler-bypass attribute detection

Current code detects:

```rust
#[allow(
#[expect(
```

But AES304 says it should flag compiler/runtime bypasses such as:

```rust
#[allow(...)]
#[warn(...)]
```

The current implementation misses:

```rust
#![allow(...)]
#![expect(...)]
#![warn(...)]
#[clippy::allow(...)]
#[warn(...)]
```

### Fix in `utility_bypass.rs`

Because this linter may scan itself, the existing code intentionally avoids literal bypass strings. Keep that approach.

```rust
// crates/shared/src/code-analysis/utility_bypass.rs

use std::sync::OnceLock;

pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: OnceLock<[String; 7]> = OnceLock::new();

    let prefixes = PREFIXES.get_or_init(|| {
        let mk = |chars: &[char]| chars.iter().collect::<String>();

        [
            // #[allow(
            mk(&['#', '[', 'a', 'l', 'l', 'o', 'w', '(']),
            // #[expect(
            mk(&['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']),
            // #[warn(
            mk(&['#', '[', 'w', 'a', 'r', 'n', '(']),
            // #![allow(
            mk(&['#', '!', '[', 'a', 'l', 'l', 'o', 'w', '(']),
            // #![expect(
            mk(&['#', '!', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']),
            // #![warn(
            mk(&['#', '!', '[', 'w', 'a', 'r', 'n', '(']),
            // #[clippy::allow(
            mk(&[
                '#', '[', 'c', 'l', 'i', 'p', 'p', 'y', ':', ':', 'a', 'l', 'l', 'o', 'w', '(',
            ]),
        ]
    });

    prefixes.iter().any(|prefix| line.starts_with(prefix))
}
```

You may want to rename this function to:

```rust
starts_with_compiler_bypass_attr
```

But keeping the old name avoids a large refactor.

---

## 2.4 High: AES305 configuration is not honored by the orchestrator

In `CodeDuplicationAnalyzer::handle_duplicates`, configuration is read:

```rust
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
```

But in `CodeAnalysisOrchestrator::run_all_checks`, the duplication check is hardcoded:

```rust
let min_dup_lines: usize = 5;
let threshold_pct: f64 = 50.0;
```

This means project YAML configuration for AES305 is ignored during the main scan.

### Fix

Replace the hardcoded values:

```rust
// crates/code-analysis/src/agent_code_analysis_orchestrator.rs

let (min_dup_lines, threshold_pct) = config
    .rules
    .iter()
    .find(|r| r.name.value == "AES305")
    .map(|rule| {
        let min_dup_lines = if rule.code_analysis.min_lines.value > 0 {
            rule.code_analysis.min_lines.value as usize
        } else {
            5
        };

        let threshold_pct = rule
            .code_analysis
            .duplication_threshold
            .unwrap_or(50.0);

        (min_dup_lines, threshold_pct)
    })
    .unwrap_or((5, 50.0));
```

Then use those values:

```rust
let dup_violations = self
    .container
    .duplication_checker()
    .check_file_similarity_entries(&entries, min_dup_lines, threshold_pct);
```

---

## 2.5 High: Duplication analyzer has memory and CPU scalability problems

Current implementation:

```rust
let mut global: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
```

For every file and every sliding window, it stores:

```rust
(file_index, line_number)
```

For a large codebase this can explode.

Example:

- 10,000 files
- average 300 lines
- window size 5

That is roughly:

```text
10,000 * 296 = 2,960,000 windows
```

Each window stores a location tuple. Memory grows quickly.

Also:

```rust
normalize_window(w)
```

allocates a new `String` for every window, twice:

1. during global map construction
2. during per-file similarity calculation

This is CPU-heavy and allocation-heavy.

---

### Recommended fix: hash windows and store only file sets

Move this helper into `utility_duplication.rs`:

```rust
// crates/shared/src/code-analysis/utility_duplication.rs

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_window(window: &[&str]) -> u64 {
    let mut hasher = DefaultHasher::new();

    for line in window {
        let trimmed = line.trim();

        for ch in trimmed.chars() {
            if ch.is_alphanumeric() || ch.is_whitespace() {
                ch.hash(&mut hasher);
            }
        }

        // Line separator.
        0u8.hash(&mut hasher);
    }

    hasher.finish()
}
```

Then refactor:

```rust
// crates/code-analysis/src/capabilities_code_duplication_analyzer.rs

use std::collections::{HashMap, HashSet};

pub fn check_file_similarity_entries(
    &self,
    entries: &[(String, String)],
    min_dup_lines: usize,
    threshold_pct: f64,
) -> Vec<(String, AesCodeAnalysisViolation)> {
    if entries.is_empty() || min_dup_lines == 0 {
        return Vec::new();
    }

    let mut key_to_files: HashMap<u64, HashSet<usize>> = HashMap::new();

    // First pass: build key -> unique files map.
    for (file_idx, (_, content)) in entries.iter().enumerate() {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < min_dup_lines {
            continue;
        }

        for window in lines.windows(min_dup_lines) {
            let key = shared::code_analysis::utility_duplication::hash_window(window);
            key_to_files.entry(key).or_default().insert(file_idx);
        }
    }

    // Keep only keys that appear in more than one file.
    let shared_keys: HashSet<u64> = key_to_files
        .iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(key, _)| *key)
        .collect();

    // Build file -> related files map.
    let mut file_to_others: Vec<HashSet<usize>> = vec![HashSet::new(); entries.len()];

    for files in key_to_files.values() {
        if files.len() <= 1 {
            continue;
        }

        for file_idx in files {
            for other_idx in files {
                if file_idx != other_idx {
                    file_to_others[*file_idx].insert(*other_idx);
                }
            }
        }
    }

    let mut violations = Vec::new();

    // Second pass: calculate shared window percentage per file.
    for (file_idx, (file_path, content)) in entries.iter().enumerate() {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < min_dup_lines {
            continue;
        }

        let total_windows = lines.len() - min_dup_lines + 1;

        let shared_count = lines
            .windows(min_dup_lines)
            .filter(|window| {
                let key = shared::code_analysis::utility_duplication::hash_window(window);
                shared_keys.contains(&key)
            })
            .count();

        let pct = shared_count as f64 / total_windows as f64 * 100.0;

        if pct > threshold_pct {
            let mut other_files: Vec<String> = file_to_others[file_idx]
                .iter()
                .map(|other_idx| entries[*other_idx].0.clone())
                .collect();

            other_files.sort();

            let mut msg = format!(
                "AES305: {:.0}% of this file's content appears in other files (threshold: {:.0}%). {} of {} windows are non-unique.",
                pct,
                threshold_pct,
                shared_count,
                total_windows,
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
```

### Notes

This removes:

```rust
Vec<(usize, usize)>
```

location storage.

It stores only:

```rust
key -> HashSet<file_index>
```

That is much smaller.

For even better scalability:

- use `ahash` or `xxhash` instead of `DefaultHasher`
- use 128-bit hashes or collision verification if you need absolute correctness
- add a maximum file-size limit
- add a maximum window budget
- parallelize per-file window hashing with Rayon
- cache results by file content hash for incremental scans

---

## 2.6 High: Agent layer violates AES architecture

According to `ARCHITECTURE.md`:

> Agent may depend only on Taxonomy and Contract.

But `CodeAnalysisOrchestrator` currently does:

```rust
std::fs::read_to_string(file)
```

and:

```rust
shared::common::utility_file::walk_source_files(...)
```

It also defines utility-like functions:

```rust
pub fn detect_source_dir(...)
pub fn collect_source_files(...)
pub fn resolve_target(...)
```

These are technical filesystem concerns, not orchestration policy.

This violates:

- AES layer rules
- SRP
- DIP
- testability

The agent should coordinate checks, not walk directories and read files directly.

---

### Recommended architecture

Introduce contracts for file discovery and file reading.

Example:

```rust
// crates/shared/src/code-analysis/contract_file_provider_protocol.rs

use crate::common::taxonomy_path_vo::FilePath;

pub struct FileEntry {
    pub path: FilePath,
    pub content: String,
}

pub trait IFileProviderProtocol: Send + Sync {
    fn collect_files(&self, root: &FilePath) -> Vec<FilePath>;

    fn read_files(&self, files: &[FilePath]) -> Vec<FileEntry>;
}
```

Then the orchestrator becomes:

```rust
pub struct CodeAnalysisOrchestrator {
    container: Arc<CodeAnalysisCheckerContainer>,
    file_provider: Arc<dyn IFileProviderProtocol>,
}
```

And the root layer wires the concrete filesystem implementation.

This gives:

- clean agent layer
- easier tests with fake file providers
- better scalability later: filesystem, git, in-memory, remote workspace, etc.

---

## 2.7 High: Global container state reduces testability

Current code:

```rust
static GLOBAL_CONTAINER: OnceLock<Arc<CodeAnalysisCheckerContainer>> = OnceLock::new();

pub fn init_global_checker(container: Arc<CodeAnalysisCheckerContainer>) {
    GLOBAL_CONTAINER.set(container).ok();
}
```

And:

```rust
impl CodeAnalysisOrchestrator {
    pub fn new() -> Self {
        Self {
            container: match GLOBAL_CONTAINER.get().cloned() {
                Some(c) => c,
                None => Arc::new(CodeAnalysisCheckerContainer::default()),
            },
        }
    }
}
```

This is a service-locator pattern. It hides dependencies and makes tests order-dependent.

### Fix

Prefer explicit injection:

```rust
pub struct CodeAnalysisOrchestrator {
    container: Arc<CodeAnalysisCheckerContainer>,
}

impl CodeAnalysisOrchestrator {
    pub fn new(container: Arc<CodeAnalysisCheckerContainer>) -> Self {
        Self { container }
    }
}
```

If backward compatibility is required, deprecate the global initializer:

```rust
#[deprecated(note = "Inject CodeAnalysisCheckerContainer explicitly instead.")]
pub fn init_global_checker(container: Arc<CodeAnalysisCheckerContainer>) {
    GLOBAL_CONTAINER.set(container).ok();
}
```

But new code should avoid global state.

---

## 2.8 High: `CodeDuplicationAnalyzer::handle_duplicates` uses default config

Current implementation:

```rust
let config = default_aes_config();
```

This means when the protocol method is used directly, it ignores the actual project configuration.

### Fix

Inject configuration into the analyzer.

```rust
pub struct CodeDuplicationAnalyzer {
    min_dup_lines: usize,
    threshold_pct: f64,
    ignored_paths: Vec<String>,
}
```

Constructor:

```rust
impl CodeDuplicationAnalyzer {
    pub fn from_config(config: &ArchitectureConfig) -> Self {
        let aes305 = config
            .rules
            .iter()
            .find(|rule| rule.name.value == "AES305");

        let min_dup_lines = aes305
            .map(|rule| {
                if rule.code_analysis.min_lines.value > 0 {
                    rule.code_analysis.min_lines.value as usize
                } else {
                    10
                }
            })
            .unwrap_or(10);

        let threshold_pct = aes305
            .and_then(|rule| rule.code_analysis.duplication_threshold)
            .unwrap_or(50.0);

        let ignored_paths = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();

        Self {
            min_dup_lines,
            threshold_pct,
            ignored_paths,
        }
    }
}
```

Then `handle_duplicates` uses `self.min_dup_lines`, `self.threshold_pct`, and `self.ignored_paths`.

Wire it in the container:

```rust
code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::from_config(&config)),
```

---

## 2.9 Medium: Path traversal / symlink escape risk

`walk_source_files` canonicalizes symlink targets and walks them:

```rust
if let Ok(target) = std::fs::canonicalize(&path) {
    if target_meta.is_dir() {
        walk_source_files_inner(&target, files, ignored, visited);
    }
}
```

But it does not verify that the symlink target remains inside the scan root.

A symlink could point outside the project:

```text
project/src/evil -> /etc
```

This may cause the scanner to read files outside the intended workspace.

### Fix

Add root confinement.

```rust
// crates/shared/src/common/utility_file.rs

pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let root = match std::fs::canonicalize(dir) {
        Ok(root) => root,
        Err(_) => return,
    };

    let mut visited = HashSet::new();
    walk_source_files_inner(&root, &root, files, ignored, &mut visited);
}

fn walk_source_files_inner(
    dir: &Path,
    root: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<u64>,
) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if is_ignored_dir(&path, ignored) {
            continue;
        }

        if let Ok(sym_meta) = std::fs::symlink_metadata(&path) {
            if sym_meta.file_type().is_symlink() {
                if let Ok(target) = std::fs::canonicalize(&path) {
                    // Prevent symlink escape.
                    if !target.starts_with(root) {
                        continue;
                    }

                    if let Ok(target_meta) = target.metadata() {
                        let inode = get_inode(&target_meta);

                        if !visited.insert(inode) {
                            continue;
                        }

                        if target_meta.is_dir() {
                            walk_source_files_inner(&target, root, files, ignored, visited);
                        } else if target_meta.is_file() {
                            collect_source_file(&target, files);
                        }
                    }
                }

                continue;
            }
        }

        if path.is_dir() {
            let dir_name = path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();

            if dir_name == "tests" {
                continue;
            }

            if let Ok(meta) = fs::metadata(&path) {
                let inode = get_inode(&meta);

                if !visited.insert(inode) {
                    continue;
                }
            }

            walk_source_files_inner(&path, root, files, ignored, visited);
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if is_source_file(ext) {
                collect_source_file(&path, files);
            }
        }
    }
}
```

Also add a target confinement helper for user-provided scan paths:

```rust
pub fn confine_path(root: &Path, target: &Path) -> Result<std::path::PathBuf, String> {
    let root = root
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize root: {e}"))?;

    let target = if target.is_absolute() {
        target.to_path_buf()
    } else {
        root.join(target)
    };

    let target = target
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize target: {e}"))?;

    if !target.starts_with(&root) {
        return Err(format!(
            "target path escapes scan root: {}",
            target.display()
        ));
    }

    Ok(target)
}
```

Use this in the surface layer before calling the orchestrator.

---

## 2.10 Medium: No file-size limit creates denial-of-service risk

The scanner reads entire files into memory:

```rust
std::fs::read_to_string(file)
```

A huge generated file can cause high memory usage or slow scans.

### Fix

Add a maximum lintable file size.

```rust
const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024; // 2 MiB

fn read_lintable_file(path: &str) -> Result<Option<String>, std::io::Error> {
    let metadata = std::fs::metadata(path)?;

    if metadata.len() > MAX_LINT_FILE_BYTES {
        return Ok(None);
    }

    std::fs::read_to_string(path).map(Some)
}
```

In the orchestrator:

```rust
for file in files {
    let content = match read_lintable_file(file) {
        Ok(Some(content)) => content,
        Ok(None) => {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES301",
                Severity::LOW,
                "File skipped: exceeds maximum lintable file size.",
            ));
            continue;
        }
        Err(_) => {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES000",
                Severity::LOW,
                "File skipped: unable to read file.",
            ));
            continue;
        }
    };

    entries.push((file.clone(), content.clone()));
}
```

You may want a dedicated code such as:

```text
AES000 SCAN_WARNING
```

or:

```text
AES001 FILE_UNREADABLE
```

instead of reusing AES301.

---

## 2.11 Medium: Error handling is too silent

There are many places where errors disappear:

```rust
Err(_) => continue
```

Example:

```rust
let c = match std::fs::read_to_string(file) {
    Ok(content) => content,
    Err(_) => continue,
};
```

This hides:

- permission errors
- broken symlinks
- I/O failures
- invalid UTF-8

For a linting tool, silent skips are dangerous because users may believe a project is clean when it was not fully scanned.

### Recommendation

Return structured scan diagnostics.

Preferred long-term API:

```rust
pub trait ICodeAnalysisAggregate: Send + Sync {
    fn run_code_analysis(
        &self,
        project_root: &FilePath,
    ) -> Result<LintResultList, ScanError>;
}
```

If breaking changes are not acceptable, add a new API:

```rust
fn try_run_code_analysis(
    &self,
    project_root: &FilePath,
) -> Result<LintResultList, ScanError>;
```

At minimum, emit low-severity diagnostics for unreadable files:

```rust
Err(err) => {
    violations.push(LintResult::new_arch(
        file,
        0,
        "AES000",
        Severity::LOW,
        format!("Unable to read file: {err}"),
    ));
    continue;
}
```

---

## 2.12 Medium: `LintResult::new_arch` swallows invalid file paths

Current constructor:

```rust
file: FilePath::new(file.to_string()).unwrap_or_default(),
```

If the path is invalid, the violation is still emitted with an empty/default path.

That makes reports confusing.

### Fix

Add a fallible constructor:

```rust
impl LintResult {
    pub fn try_new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Result<Self, String> {
        Ok(Self {
            file: FilePath::new(file.to_string())?,
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        })
    }
}
```

Keep `new_arch` temporarily for backward compatibility, but migrate internal callers to `try_new_arch`.

---

## 2.13 Medium: Cargo.toml bypass detection should use a TOML parser

Current implementation scans Cargo.toml line-by-line:

```rust
if let Some(eq_pos) = t.find('=') {
    let val = t[eq_pos + 1..].trim();
    if val == "\"allow\"" || val == "'allow'" {
        // violation
    }
}
```

This can produce false positives:

```toml
# example = "allow"
```

It can also miss valid TOML structures:

```toml
[workspace.lints.clippy]
unused = { level = "allow", priority = 1 }
```

### Fix

Use a real TOML parser.

Add dependency:

```toml
toml = "0.8"
```

Then:

```rust
fn check_cargo_toml(&self, path: &str, content: &str, violations: &mut Vec<LintResult>) {
    let value: toml::Value = match toml::from_str(content) {
        Ok(value) => value,
        Err(_) => return,
    };

    let clippy_table = value
        .get("workspace")
        .and_then(|w| w.get("lints"))
        .and_then(|l| l.get("clippy"))
        .or_else(|| value.get("lints").and_then(|l| l.get("clippy")));

    let Some(clippy_table) = clippy_table.and_then(|v| v.as_table()) else {
        return;
    };

    for (key, val) in clippy_table {
        if is_toml_allow(val) {
            violations.push(LintResult::new_arch(
                path,
                0,
                "AES304",
                Severity::CRITICAL,
                format!("Cargo.toml clippy allow bypass: `{key}`"),
            ));
        }
    }
}

fn is_toml_allow(value: &toml::Value) -> bool {
    match value {
        toml::Value::String(s) => s == "allow",

        toml::Value::Table(table) => table
            .get("level")
            .and_then(|level| level.as_str())
            .is_some_and(|level| level == "allow"),

        toml::Value::Array(values) => values.iter().any(is_toml_allow),

        _ => false,
    }
}
```

This requires changing the protocol:

```rust
pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);

    fn check_cargo_toml(
        &self,
        path: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
```

Then pass the real Cargo.toml path from the orchestrator.

---

## 2.14 Medium: Mandatory definition checker has false positives/negatives

Current helper:

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

Problems:

### False positives

It can match comments:

```rust
// This struct is removed.
```

It can match strings:

```rust
let s = "struct";
```

It can match partial words:

```rust
let structure = 1;
```

Although `contains("struct")` matches `structure`.

### False negatives

It excludes lines containing `(`:

```rust
pub struct Point(i64, i64);
```

Tuple structs are valid primary definitions and should count.

### Better implementation

```rust
// crates/shared/src/code-analysis/utility_mandatory.rs

pub fn rust_declares_type(line: &str) -> bool {
    let trimmed = line.trim();

    // Ignore comments.
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }

    let mut t = trimmed;

    // Strip simple visibility modifiers.
    if let Some(rest) = t.strip_prefix("pub") {
        t = rest.trim_start();
    }

    // Strip pub(crate), pub(super), pub(in path)
    if let Some(rest) = t.strip_prefix("pub(") {
        if let Some((_, after)) = rest.split_once(')') {
            t = after.trim_start();
        }
    }

    if let Some(rest) = t.strip_prefix("crate") {
        t = rest.trim_start();
    }

    let keywords = ["struct", "enum", "trait"];

    for keyword in keywords {
        if let Some(after) = t.strip_prefix(keyword) {
            let next = after.chars().next();

            if matches!(next, Some(' ') | Some('\t') | Some('(') | Some('<')) {
                return true;
            }
        }
    }

    false
}
```

This is still heuristic, but much safer than `contains`.

For production-grade accuracy, use a real parser:

- Rust: `syn`
- Python: AST parser
- TypeScript/JavaScript: tree-sitter or SWC

---

## 2.15 Medium: Dead inheritance checker misses common cases

Current logic checks:

```rust
if t.starts_with("struct ") && t.ends_with(';') && !t.contains('(')
```

This misses:

```rust
pub struct Foo;
pub(crate) struct Bar;
```

It also only flags a unit struct if an `impl` block immediately follows. But this is valid:

```rust
pub struct Foo;

impl Foo {
    fn new() -> Self {
        Self
    }
}
```

If another item appears between the struct and impl, it may be falsely flagged.

### Recommendation

Parse the whole file for definitions and impl blocks.

Simplified approach:

```rust
fn has_impl_for_type(lines: &[&str], type_name: &str) -> bool {
    let impl_prefix = format!("impl {type_name}");
    let impl_generic_prefix = format!("impl<");

    lines.iter().any(|line| {
        let trimmed = line.trim();

        trimmed.starts_with(&impl_prefix)
            || trimmed.starts_with("impl ")
                && trimmed.contains(type_name)
            || trimmed.starts_with(&impl_generic_prefix)
                && trimmed.contains(type_name)
    })
}
```

Also handle visibility modifiers when extracting struct names.

For JS/TS, current logic misses:

```ts
export class Foo {}
export default class Bar {}
```

For Python, current logic misses:

```python
class Foo:
    pass  # comment
```

A comment-stripping step is needed.

Again, a real parser is the robust solution.

---

## 2.16 Medium: `active_rules()` returns empty

Current implementation:

```rust
fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
    Vec::new()
}
```

This is misleading. API consumers may expect active rule metadata.

### Fix

Return configured rules:

```rust
fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
    self.container
        .config()
        .rules
        .iter()
        .map(|rule| rule.code_analysis.clone())
        .collect()
}
```

If you need full rule metadata, consider changing the return type to:

```rust
Vec<ArchitectureRule>
```

or introducing:

```rust
ActiveRuleVO
```

---

## 2.17 Medium: Public aggregate mixes too many responsibilities

`ICodeAnalysisAggregate` currently includes:

```rust
run_code_analysis
run_code_analysis_dir
run_code_analysis_path
calc_score
check_critical
format_report
active_rules
```

This violates the Interface Segregation Principle.

A surface that only needs scoring should not depend on report formatting.

A surface that only needs reporting should not depend on scan execution.

### Recommended split

```rust
pub trait ICodeAnalysisRunner: Send + Sync {
    fn run_code_analysis(&self, project_root: &FilePath) -> LintResultList;
    fn run_code_analysis_dir(&self, src_dir: &FilePath) -> LintResultList;
    fn run_code_analysis_path(&self, path: &FilePath) -> LintResultList;
}

pub trait IComplianceScoreCalculator: Send + Sync {
    fn calc_score(&self, results: &[LintResult]) -> Score;
    fn check_critical(&self, results: &[LintResult]) -> bool;
}

pub trait IComplianceReportFormatter: Send + Sync {
    fn format_report(
        &self,
        results: &LintResultList,
        project_root: &FilePath,
    ) -> String;
}

pub trait ICodeAnalysisRuleProvider: Send + Sync {
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO>;
}
```

Then the orchestrator can implement all of them, but consumers depend only on what they need.

---

## 2.18 Medium: Orchestrator is not open for extension

Current `run_all_checks` manually calls each checker:

```rust
self.container.bypass_checker()...
self.container.dead_inheritance_checker()...
self.container.line_checker()...
self.container.class_checker()...
self.container.duplication_checker()...
```

Adding a new AES rule requires modifying the orchestrator.

This violates the Open/Closed Principle.

### Better design: checker registry

Define a generic capability contract:

```rust
pub enum CheckScope {
    AllFiles,
    LayerDependent,
    ProjectWide,
}

pub struct FileCheckContext<'a> {
    pub file: &'a str,
    pub filename: &'a str,
    pub content: &'a str,
    pub layer_definition: Option<&'a LayerDefinition>,
}

pub struct ProjectCheckContext<'a> {
    pub entries: &'a [(String, String)],
    pub root_dir: &'a str,
}

pub trait ICodeAnalysisCheck: Send + Sync {
    fn code(&self) -> &'static str;
    fn scope(&self) -> CheckScope;

    fn check_file(
        &self,
        _ctx: &FileCheckContext<'_>,
        _violations: &mut Vec<LintResult>,
    ) {
    }

    fn check_project(
        &self,
        _ctx: &ProjectCheckContext<'_>,
        _violations: &mut Vec<LintResult>,
    ) {
    }
}
```

Then the orchestrator becomes:

```rust
pub struct CodeAnalysisOrchestrator {
    checks: Vec<Arc<dyn ICodeAnalysisCheck>>,
}
```

And execution:

```rust
for check in &self.checks {
    match check.scope() {
        CheckScope::AllFiles | CheckScope::LayerDependent => {
            // call check_file per file
        }
        CheckScope::ProjectWide => {
            // call check_project once
        }
    }
}
```

This makes adding AES306, AES307, etc. much cleaner.

---

## 2.19 Low: Duplicated utility functions

The agent file defines:

```rust
pub fn detect_source_dir(...)
pub fn collect_source_files(...)
pub fn resolve_target(...)
```

But shared already has:

```rust
shared::code_analysis::utility_target::detect_source_dir
shared::code_analysis::utility_target::collect_source_files
shared::code_analysis::utility_target::resolve_target
```

This violates DRY.

### Fix

Remove the agent-local copies.

If the orchestrator still needs them temporarily, call the shared utility versions. Long-term, move file collection behind a contract as described earlier.

---

## 2.20 Low: Inconsistent return types

Current aggregate:

```rust
fn run_code_analysis(&self, project_root: &FilePath) -> LintResultList;
fn run_code_analysis_dir(&self, src_dir: &FilePath) -> LintResultList;
fn run_code_analysis_path(&self, path: &FilePath) -> Vec<LintResult>;
```

`run_code_analysis_path` returns `Vec<LintResult>` while the others return `LintResultList`.

Prefer consistency:

```rust
fn run_code_analysis_path(&self, path: &FilePath) -> LintResultList;
```

If compatibility is required, add:

```rust
fn run_code_analysis_path_vec(&self, path: &FilePath) -> Vec<LintResult>;
```

and deprecate it.

---

# 3. Security Review

## 3.1 Path traversal

Risk: user-provided scan paths may escape workspace root.

Fix:

- canonicalize root
- canonicalize target
- ensure target starts with root
- prune symlink targets outside root

See section 2.9.

---

## 3.2 Denial of service via large files

Risk:

- huge files read fully into memory
- duplication analyzer stores millions of windows
- whole-file lowercase allocation

Fix:

- max file size
- max total file count
- max duplication window budget
- hash-based window comparison
- streaming line count for AES301/AES302
- ignore generated/vendor directories by default

---

## 3.3 Unsafe string slicing

This line is mostly safe because `=` is ASCII:

```rust
let val = t[eq_pos + 1..].trim();
```

But string slicing in Rust can panic if indices are not char boundaries. In this exact case it is okay, but I still recommend replacing the line-based Cargo.toml parser with a real TOML parser.

---

## 3.4 False positives from comments/strings

The bypass checker currently uses line-based heuristics. This can flag:

```rust
let msg = "do not unwrap here";
```

or:

```rust
// TODO: remove later
```

depending on pattern configuration.

For security-sensitive linting, use:

- Rust: `syn` or tree-sitter
- Python: AST
- TS/JS: tree-sitter/SWC

Line-based scanning is acceptable as a first pass, but it should not be the final architecture.

---

# 4. Performance Review

## 4.1 Duplication detection

Current bottlenecks:

- `normalize_window` allocates a string per window
- global map stores every window location
- second pass re-normalizes every window
- whole-file lowercasing in bypass checker

Fixes:

- hash windows
- store only `key -> HashSet<file_index>`
- remove unused `interned_keys`
- avoid full-file lowercase
- use Aho-Corasick for bypass patterns
- parallelize file reading and per-file checks

---

## 4.2 Sequential file I/O

Current code reads files sequentially:

```rust
for file in files {
    std::fs::read_to_string(file)
}
```

For large repositories, use parallel I/O:

```rust
use rayon::prelude::*;
```

Example:

```rust
let entries: Vec<(String, String)> = files
    .par_iter()
    .filter_map(|file| {
        std::fs::read_to_string(file)
            .ok()
            .map(|content| (file.clone(), content))
    })
    .collect();
```

Then sort violations deterministically:

```rust
violations.sort_by(|a, b| {
    a.file
        .value
        .cmp(&b.file.value)
        .then(a.line.value.cmp(&b.line.value))
        .then(a.code.code().cmp(b.code.code()))
});
```

---

## 4.3 Config cloning

`default_aes_config()` clones the full config each time:

```rust
.clone()
```

For large configs, consider:

```rust
Arc<ArchitectureConfig>
```

Example:

```rust
static DEFAULT_RUST_CONFIG: OnceLock<Arc<ArchitectureConfig>> = OnceLock::new();
```

Then return:

```rust
Arc<ArchitectureConfig>
```

---

# 5. Error Handling Review

Current style:

```rust
Err(_) => continue
```

This is too silent.

Recommended hierarchy:

```text
ScanError
  - path
  - operation
  - cause
  - severity
```

Use:

```rust
Result<LintResultList, ScanError>
```

For partial failures, return both:

```rust
pub struct ScanOutcome {
    pub results: LintResultList,
    pub warnings: Vec<ScanWarning>,
}
```

This is better than silently skipping files.

---

# 6. Business Logic Review

## AES301 / AES302

Current line counting:

```rust
content.lines().count()
```

This counts blank lines and comments.

If the goal is single-responsibility cohesion, you may want:

- total lines
- non-empty lines
- code lines
- comment lines

Consider configurable mode:

```yaml
AES301:
  line_count_mode: total | non_empty | code
```

---

## AES303

Mandatory definition checking should be language-aware.

Current logic mainly detects:

- Rust struct/enum/trait
- Python/JS/TS class

But FRD says:

> struct, enum, class, or interface/trait

For TypeScript, interface and type should count:

```ts
export interface Foo {}
export type Bar = {};
```

For Python, `Protocol`, `TypedDict`, `dataclass`, and maybe functions may count depending on layer.

Recommendation:

- introduce language-specific definition detectors
- use AST where possible
- make mandatory symbol kinds configurable per layer

Example:

```yaml
AES303:
  mandatory_definitions:
    - struct
    - enum
    - trait
    - class
    - interface
    - type
```

---

## AES304

Bypass detection should distinguish:

- comments
- strings
- attributes
- real code

The current implementation is heuristic.

Long-term:

- parse Rust attributes with `syn`
- parse Python AST for `raise NotImplementedError`, `assert False`
- parse TS/JS AST for `throw new Error`

---

## AES305

Duplication detection should support:

- minimum window size from config
- threshold from config
- ignored files
- generated file exclusion
- language-aware normalization
- structural duplication, not only textual similarity

For better quality:

- normalize identifiers
- ignore imports/use statements
- ignore test files optionally
- use token-based duplication instead of line-based

---

# 7. Architecture / SOLID Review

## SRP

`CodeAnalysisOrchestrator` currently handles:

- source directory detection
- file collection
- file reading
- Cargo.toml discovery
- check execution
- duplication configuration
- report formatting
- score calculation

That is too many responsibilities.

Split into:

```text
FileCollector
FileReader
CodeAnalysisRunner
ComplianceScoreCalculator
ComplianceReportFormatter
```

---

## OCP

Adding a new check currently requires modifying orchestrator code.

Use a checker registry.

See section 2.18.

---

## LSP

Contract implementations are mostly okay, but some contracts are too broad.

Splitting the aggregate improves LSP/ISP.

---

## ISP

`ICodeAnalysisAggregate` is too broad.

Split into:

```rust
ICodeAnalysisRunner
IComplianceScoreCalculator
IComplianceReportFormatter
ICodeAnalysisRuleProvider
```

---

## DIP

The orchestrator depends on:

- global state
- concrete `CodeAnalysisCheckerContainer`
- concrete `CodeDuplicationAnalyzer`
- filesystem functions

It should depend on contracts:

```rust
Arc<IFileProviderProtocol>
Arc<ICodeAnalysisCheck>
Arc<IComplianceScoreCalculator>
Arc<IComplianceReportFormatter>
```

Root layer wires concrete implementations.

---

# 8. Database Query Guidance

There is no database layer in this crate.

If you later persist scan results, follow these rules:

## 8.1 Use parameterized queries

Bad:

```rust
let query = format!("SELECT * FROM violations WHERE file = '{file}'");
```

Good with SQLx:

```rust
sqlx::query!(
    "SELECT id, file, line, code, severity, message FROM violations WHERE file = $1",
    file
)
.fetch_all(&pool)
.await?;
```

---

## 8.2 Use a connection pool

```rust
let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await?;
```

---

## 8.3 Use transactions for batch inserts

```rust
let mut tx = pool.begin().await?;

for violation in violations {
    sqlx::query!(
        "INSERT INTO violations (scan_id, file, line, code, severity, message)
         VALUES ($1, $2, $3, $4, $5, $6)",
        scan_id,
        violation.file.value,
        violation.line.value,
        violation.code.code(),
        violation.severity.to_string(),
        violation.message.value,
    )
    .execute(&mut *tx)
    .await?;
}

tx.commit().await?;
```

---

## 8.4 Add indexes

For query patterns like:

```sql
SELECT * FROM violations WHERE scan_id = ?;
SELECT * FROM violations WHERE file = ?;
SELECT * FROM violations WHERE code = ?;
```

Add indexes:

```sql
CREATE INDEX idx_violations_scan_id ON violations(scan_id);
CREATE INDEX idx_violations_file ON violations(file);
CREATE INDEX idx_violations_code ON violations(code);
```

---

# 9. Recommended Refactoring Sequence

## Phase 1: Correctness and lint compliance

1. Fix `clippy::collapsible_match`.
2. Fix multiline `static Lazy` skip.
3. Fix `#[cfg(test)]` block skip.
4. Expand compiler attribute bypass detection.
5. Honor AES305 config in orchestrator.

---

## Phase 2: Safety and reliability

1. Add path confinement.
2. Prevent symlink escape.
3. Add max file size.
4. Emit diagnostics for unreadable files.
5. Replace Cargo.toml line parsing with TOML parsing.

---

## Phase 3: Performance

1. Replace duplication string interning with window hashing.
2. Remove unused `interned_keys`.
3. Remove whole-file lowercase allocation.
4. Add Aho-Corasick for bypass patterns.
5. Parallelize file reads and per-file checks.

---

## Phase 4: Architecture

1. Remove global container.
2. Inject dependencies explicitly.
3. Move file discovery/read out of agent layer.
4. Introduce `IFileProviderProtocol`.
5. Introduce `ICodeAnalysisCheck` registry.
6. Split `ICodeAnalysisAggregate` into smaller contracts.

---

# 10. Minimal Patch for the Reported Clippy Issue

If you want the smallest possible fix for the reported violation only, apply this in `capabilities_check_bypass_checker.rs`:

```rust
fn is_rust_word_pattern(token: &str) -> bool {
    matches!(
        token,
        "unwrap" | "expect" | "panic" | "todo" | "unimplemented" | "unreachable"
    )
}
```

Then replace the existing pattern loop with:

```rust
for p in &patterns.values {
    let p_str = p.as_str();

    match p_str {
        p if is_rust_word_pattern(p)
            && matches_word_token(t, p, matches!(p, "unwrap" | "expect"))
            && !(p == "unwrap" && t.contains(".unwrap_or")) =>
        {
            let vo = match Self::classify_token(p) {
                ViolationKind::UnwrapExpect => {
                    AesCodeAnalysisViolation::UnwrapExpect { reason: None }
                }
                ViolationKind::Panic => {
                    AesCodeAnalysisViolation::Panic { reason: None }
                }
                ViolationKind::Todo => {
                    AesCodeAnalysisViolation::Todo { reason: None }
                }
                ViolationKind::Unimplemented => {
                    AesCodeAnalysisViolation::Unimplemented { reason: None }
                }
                ViolationKind::BypassComment => {
                    AesCodeAnalysisViolation::BypassComment { reason: None }
                }
            };

            violations.push(LintResult::new_arch(
                file,
                i + 1,
                "AES304",
                Severity::CRITICAL,
                vo.to_string(),
            ));

            break;
        }

        p if !is_rust_word_pattern(p)
            && !p.is_empty()
            && t_lower.contains(&p.to_lowercase()) =>
        {
            violations.push(LintResult::new_arch(
                file,
                i + 1,
                "AES304",
                Severity::CRITICAL,
                AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
            ));

            break;
        }

        _ => {}
    }
}
```

This is the most direct fix for the reported `collapsible_match` issue.

---

# 11. Final Recommendation

The codebase has a strong architectural intention, but the `code-analysis` crate currently mixes orchestration, filesystem mechanics, and rule execution too heavily. The most valuable improvements are:

1. Fix the AES304 bypass checker correctness bugs.
2. Fix the clippy violation.
3. Honor AES305 configuration.
4. Make duplication detection memory-safe for large repositories.
5. Remove global state and filesystem I/O from the agent layer.
6. Introduce a checker registry for future AES rules.
7. Add explicit scan diagnostics instead of silent skips.

If you want, I can next provide a concrete unified patch for:

```text
capabilities_check_bypass_checker.rs
agent_code_analysis_orchestrator.rs
capabilities_code_duplication_analyzer.rs
utility_bypass.rs
utility_duplication.rs
```

with the fixes applied in full.
