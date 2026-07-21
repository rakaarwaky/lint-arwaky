Yes — I reviewed the uploaded `shared` crate source as a QA/debugging pass. Below are concrete functional defects and performance bottlenecks I found, with explanations and targeted fixes. I did **not** add new features; all changes are corrective or optimization-only.

---

# 1. Compile / reliability defects

## 1.1 Invalid import of primitive `bool` from a VO module

### Files

- `crates/shared/src/external-lint/contract_external_lint_language_detector_protocol.rs`
- `crates/shared/src/external-lint/contract_external_lint_selector_protocol.rs`
- `crates/shared/src/external-lint/contract_external_lint_utility_protocol.rs`

### Bug

These files import `bool` as if it were a VO:

```rust
use crate::common::taxonomy_common_vo::bool;
```

But `taxonomy_common_vo` defines `BooleanVO`, not `bool`. Primitive `bool` is built-in and cannot be imported from that module. This is at minimum a compile-time defect if the module is compiled as shown.

### Fix

Remove the invalid import and use primitive `bool` directly.

#### `contract_external_lint_language_detector_protocol.rs`

```rust
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub struct DetectedLanguages {
    pub has_rs: bool,
    pub has_py: bool,
    pub has_js: bool,
}

#[async_trait]
pub trait IExternalLintLanguageDetectorProtocol: Send + Sync {
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages;
}
```

#### `contract_external_lint_selector_protocol.rs`

```rust
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintSelectorProtocol: Send + Sync {
    fn select_adapters(
        &self,
        has_rs: bool,
        has_py: bool,
        has_js: bool,
    ) -> Vec<String>;
}
```

#### `contract_external_lint_utility_protocol.rs`

Replace:

```rust
use crate::common::taxonomy_common_vo::{bool, PatternList};
```

with:

```rust
use crate::common::taxonomy_common_vo::PatternList;
```

---

## 1.2 `build.rs` uses `expect` / `panic`, causing abrupt build failures

### File

- `crates/shared/build.rs`

### Bug

The build script uses `expect` and `panic!`:

```rust
let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
...
.expect("cannot reach workspace root from manifest dir");
...
panic!("Config file not found...");
...
fs::copy(&src, &dst).expect("Failed to copy config file");
```

This bypasses structured error handling and produces poor diagnostics. Build scripts can return `Result`.

### Fix

```rust
// build.rs — copy config YAML files from workspace root into OUT_DIR at build time
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let workspace_root = Path::new(&manifest_dir)
        .ancestors()
        .nth(2)
        .ok_or("cannot reach workspace root from manifest dir")?;

    for name in &[
        "lint_arwaky.config.rust.yaml",
        "lint_arwaky.config.python.yaml",
        "lint_arwaky.config.javascript.yaml",
    ] {
        let src = workspace_root.join(name);
        let dst = Path::new(&out_dir).join(name);

        if !src.exists() {
            return Err(format!(
                "Config file not found at {:?}. Check that config files are in the workspace root.",
                src
            )
            .into());
        }

        fs::copy(&src, &dst).map_err(|e| {
            format!("Failed to copy config file from {:?} to {:?}: {}", src, dst, e)
        })?;
    }

    println!("cargo:rerun-if-changed=lint_arwaky.config.rust.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.python.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.javascript.yaml");

    Ok(())
}
```

---

## 1.3 Mutable borrow conflict in config parser

### File

- `crates/shared/src/config-system/utility_config_parser.rs`

### Bug

This block can cause a borrow-checker problem because `arch_json` is mutably borrowed through `rules_obj`, then mutated again by `arch_json["layers"] = layers;` inside the same scope:

```rust
if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
    for (_rule_code, rule_val) in rules_obj.iter_mut() {
        if let Some(layers) = rule_val.get_mut("layers") {
            let layers = std::mem::take(layers);
            arch_json["layers"] = layers;
            break;
        }
    }
}
```

Even if some compiler revisions accept parts of this pattern, it is fragile and semantically wrong to mutate the parent value while iterating a mutable child borrow.

### Fix

Extract first, assign after the borrow ends:

```rust
let mut extracted_layers: Option<serde_json::Value> = None;

if arch_json.get("layers").is_none() {
    if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
        for (_rule_code, rule_val) in rules_obj.iter_mut() {
            if let Some(layers) = rule_val.get_mut("layers") {
                extracted_layers = Some(std::mem::take(layers));
                break;
            }
        }
    }
}

if let Some(layers) = extracted_layers {
    arch_json["layers"] = layers;
}
```

---

# 2. Functional logic bugs

## 2.1 `compute_column` returns byte offset, not character column

### File

- `crates/shared/src/code-analysis/utility_column.rs`

### Bug

```rust
pub fn compute_column(line: &str, pattern: &str) -> usize {
    line.find(pattern).map(|pos| pos + 1).unwrap_or(0)
}
```

`pos` is a byte offset. For non-ASCII lines, this reports the wrong column. The file already has `byte_offset_to_column`, but `compute_column` does not use it.

### Fix

```rust
/// Compute 1-indexed column position of `pattern` in `line`.
/// Returns 0 if pattern not found.
pub fn compute_column(line: &str, pattern: &str) -> usize {
    line.find(pattern)
        .map(|pos| byte_offset_to_column(line, pos))
        .unwrap_or(0)
}

/// Compute 1-indexed column position of a byte offset in a line.
pub fn byte_offset_to_column(line: &str, offset: usize) -> usize {
    line[..offset.min(line.len())].chars().count() + 1
}
```

---

## 2.2 `is_path_ignored` incorrectly matches extension patterns

### File

- `crates/shared/src/common/utility_file.rs`

### Bug

The current pattern logic does this:

```rust
if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
    let suffix = ...;
    if basename.ends_with(suffix) {
        return true;
    }
}
```

For pattern `"*.rs"`, suffix becomes `"rs"`, so it matches:

- `foo.rs` correctly
- but also `bars` incorrectly, because `"bars".ends_with("rs")`

For pattern `".git"`, suffix becomes `"git"`, so it can ignore a file named `git`.

### Fix

Replace that block with stricter matching:

```rust
        if let Some(suffix) = pat.strip_prefix("*.") {
            let suffix = suffix.trim_start_matches('.');
            if suffix.is_empty() {
                continue;
            }

            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(&format!(".{suffix}")) {
                return true;
            }
            continue;
        }

        if pat.starts_with('.') {
            if segments.iter().any(|seg| seg == pat) {
                return true;
            }
            continue;
        }
```

This ensures:

- `*.rs` matches only dotted `.rs` suffixes
- `.git` matches a path segment exactly `.git`

---

## 2.3 Symlinked non-source files are collected as source files

### File

- `crates/shared/src/common/utility_file.rs`

### Bug

In `walk_source_files_inner`, symlinked files are collected without checking extension:

```rust
} else if target_meta.is_file() {
    collect_source_file(&target, files);
}
```

But regular files are filtered:

```rust
} else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
    if is_source_file(ext) {
        collect_source_file(&path, files);
    }
}
```

So a symlink to `README.md` can be incorrectly treated as a source file.

### Fix

```rust
} else if target_meta.is_file() {
    if let Some(ext) = target.extension().and_then(|e| e.to_str()) {
        if is_source_file(ext) {
            collect_source_file(&target, files);
        }
    }
}
```

---

## 2.4 `contains_delimited` can panic on non-ASCII content

### File

- `crates/shared/src/orphan-detector/utility_orphan.rs`

### Bug

```rust
for i in 0..content.len().saturating_sub(token.len()) {
    if content[i..].starts_with(token) {
```

`i` increments by byte. If `i` lands inside a multibyte UTF-8 character, `content[i..]` panics.

Also:

```rust
let before = content.as_bytes()[i - 1] as char;
```

This is incorrect for non-ASCII bytes.

### Fix

Use `match_indices`, which yields valid char-boundary offsets:

```rust
pub fn contains_delimited(content: &str, token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    const DELIMITERS: [char; 12] = [
        ' ', '\t', '\n', '\r', ';', ',', '(', ')', '{', '}', '"', '\'',
    ];

    for (i, _) in content.match_indices(token) {
        let before = content[..i].chars().next_back().unwrap_or(' ');
        let after = content[i + token.len()..].chars().next().unwrap_or(' ');

        let boundary_before = before.is_whitespace() || DELIMITERS.contains(&before);
        let boundary_after = after.is_whitespace() || DELIMITERS.contains(&after);

        if boundary_before && boundary_after {
            return true;
        }
    }

    false
}
```

---

## 2.5 `has_trait_implementation` fails for generic impls

### File

- `crates/shared/src/orphan-detector/utility_orphan.rs`

### Bug

For:

```rust
impl<T> MyTrait for Foo
```

the current parser does:

```rust
let trait_part = trait_part.trim_start_matches('<').split('>').next()...
```

For `<T> MyTrait`, `trim_start_matches('<')` gives `T> MyTrait`, then `split('>').next()` gives `"T"`, not `MyTrait`.

So generic trait implementations are missed.

### Fix

Add a helper to strip leading generic parameter lists correctly:

```rust
fn strip_leading_generics<'a>(s: &'a str) -> &'a str {
    let mut s = s.trim();

    while let Some(rest) = s.strip_prefix('<') {
        let mut depth = 1usize;
        let mut end = None;

        for (idx, ch) in rest.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(idx);
                        break;
                    }
                }
                _ => {}
            }
        }

        match end {
            Some(pos) => s = rest[pos + 1..].trim(),
            None => break,
        }
    }

    s
}
```

Then replace the Rust impl detection portion with:

```rust
        if trimmed.starts_with("impl") && trimmed.contains(" for ") {
            let after_impl = trimmed[4..].trim();

            let trait_part = match after_impl.find(" for ") {
                Some(pos) => after_impl[..pos].trim(),
                None => continue,
            };

            let trait_part = strip_leading_generics(trait_part);
            let trait_base = trait_part.split('<').next().unwrap_or(trait_part).trim();
            let trait_last = trait_base.split("::").last().unwrap_or(trait_base);

            if trait_last == trait_name {
                return true;
            }
        }
```

---

## 2.6 Dummy-import detection mis-handles Python and JS import aliases

### File

- `crates/shared/src/import-rules/utility_dummy_detector.rs`

### Bug 1: Python `import module as alias`

Current code extracts the module name, not the alias:

```rust
let module = trimmed.trim_start_matches("import ")...
let name = module.rsplit('.').next()
```

For:

```python
import numpy as np
```

it records `numpy`, but the used symbol is `np`.

### Bug 2: JS `import { foo as bar }`

Current code takes `foo`, not `bar`.

### Bug 3: JS `const { foo: bar } = require(...)`

Current code takes `foo`, not `bar`.

### Fix

#### Python imported symbols

```rust
fn python_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if let Some(import_part) = trimmed
            .strip_prefix("from ")
            .and_then(|s| s.split_once(" import ").map(|(_, p)| p))
        {
            for name in import_part.split(',') {
                let name = name.trim();
                if name.is_empty() || name == "*" {
                    continue;
                }

                let used_name = match name.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => name.split_whitespace().next().unwrap_or_default(),
                };

                if !used_name.is_empty() && used_name != "*" {
                    symbols.push((SymbolName::new(used_name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("import ") {
            for module in rest.split(',') {
                let module = module.trim();
                if module.is_empty() {
                    continue;
                }

                let used_name = match module.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => module.rsplit('.').next().unwrap_or(module).trim(),
                };

                if !used_name.is_empty() && used_name != "*" {
                    symbols.push((SymbolName::new(used_name), LineNumber::new(idx as i64 + 1)));
                }
            }
        }
    }

    symbols
}
```

#### JS imported symbols

```rust
fn js_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];

                    for part in inside.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }

                        let name = match part.split_once(" as ") {
                            Some((_, alias)) => alias.trim(),
                            None => part.split_whitespace().next().unwrap_or_default(),
                        };

                        if !name.is_empty() && name != "type" {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let before_from = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or_default()
                    .trim();

                let name = match before_from.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => before_from,
                };

                if !name.is_empty() && name != "default" {
                    symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];

                    for part in inside.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }

                        let name = match part.split_once(':') {
                            Some((_, alias)) => alias.trim(),
                            None => part,
                        };

                        if !name.is_empty() {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
        }
    }

    symbols
}
```

---

## 2.7 Unused-symbol detection uses substring matching, causing false negatives

### File

- `crates/shared/src/import-rules/utility_dummy_detector.rs`

### Bug

```rust
if !trimmed.contains(symbol) {
    continue;
}
```

If symbol is `Foo`, then `FooBar` counts as usage. That hides real unused imports.

### Fix

Add an identifier-boundary helper:

```rust
fn contains_ident(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }

    let mut start = 0usize;

    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let end = abs + needle.len();

        let before_ok = abs == 0
            || {
                let b = haystack.as_bytes()[abs - 1];
                !(b.is_ascii_alphanumeric() || b == b'_')
            };

        let after_ok = end == haystack.len()
            || {
                let b = haystack.as_bytes()[end];
                !(b.is_ascii_alphanumeric() || b == b'_')
            };

        if before_ok && after_ok {
            return true;
        }

        start = abs + needle.len();
    }

    false
}
```

Then replace:

```rust
if !trimmed.contains(symbol) {
    continue;
}
```

with:

```rust
if !contains_ident(trimmed, symbol) {
    continue;
}
```

---

## 2.8 `matches_word_token` over-matches safe methods like `unwrap_or`

### File

- `crates/shared/src/code-analysis/utility_bypass.rs`

### Bug

The current underscore-segment logic treats any `token_suffix(` as a match. That means:

```rust
value.unwrap_or(default)
```

can be flagged as forbidden `unwrap`, even though it does not panic.

### Fix

Restrict underscore suffix matching to known panicking/unsafe variants only.

Replace `matches_word_token` with:

```rust
/// Returns true if `line` (already trimmed) contains `token` invoked as a method call or macro.
/// When `requires_method_call` is true, the token must be preceded by a dot (`.`).
pub fn matches_word_token(line: &str, token: &str, requires_method_call: bool) -> bool {
    if token.is_empty() {
        return false;
    }

    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }

    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();

    if bytes.len() < tlen {
        return false;
    }

    let mut i = 0;

    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok = i == 0 || !is_ident_start(bytes[i - 1]);

            if before_ok {
                if requires_method_call {
                    let preceded_by_dot = i > 0 && bytes[i - 1] == b'.';
                    if !preceded_by_dot {
                        i += 1;
                        continue;
                    }
                }

                let mut j = i + tlen;

                if j < bytes.len() && (bytes[j] == b'(' || bytes[j] == b'!') {
                    return true;
                }

                if j < bytes.len() && bytes[j] == b'_' {
                    let seg_start = j + 1;

                    if seg_start < bytes.len() && is_ident_start(bytes[seg_start]) {
                        let mut seg_end = seg_start;

                        while seg_end < bytes.len() && is_ident_continue(bytes[seg_end]) {
                            seg_end += 1;
                        }

                        let seg = &line[seg_start..seg_end];
                        let k = seg_end;

                        if k < bytes.len()
                            && (bytes[k] == b'(' || bytes[k] == b'!')
                            && forbidden_method_suffix(token, seg)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        i += 1;
    }

    false
}

fn forbidden_method_suffix(token: &str, suffix: &str) -> bool {
    matches!(
        (token, suffix),
        ("unwrap", "unchecked") | ("panic", "any")
    )
}
```

Also add comment skipping to `matches_keyword_token`:

```rust
pub fn matches_keyword_token(line: &str, token: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }

    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();

    if bytes.len() < tlen {
        return false;
    }

    let mut i = 0;

    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok =
                i == 0 || (!bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_');

            let after_ok = i + tlen == bytes.len()
                || (!bytes[i + tlen].is_ascii_alphanumeric() && bytes[i + tlen] != b'_');

            if before_ok && after_ok {
                return true;
            }
        }

        i += 1;
    }

    false
}
```

---

## 2.9 `get_relative_path` can strip incorrectly when root is a prefix substring

### File

- `crates/shared/src/import-rules/utility_path_normalizer.rs`

### Bug

```rust
if normalized_file.starts_with(&normalized_root) {
    normalized_file[normalized_root.len()..]
        .trim_start_matches('/')
        .to_string()
}
```

If:

- root = `/foo/bar`
- file = `/foo/barbaz`

then `starts_with` is true, producing an invalid relative path.

### Fix

Use component-wise `Path::strip_prefix`:

```rust
pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = match Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => file_path.replace('\\', "/"),
    };

    let normalized_root = match Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => root_dir.trim_end_matches('/').replace('\\', "/"),
    };

    let file_path = Path::new(&normalized_file);
    let root_path = Path::new(&normalized_root);

    match file_path.strip_prefix(root_path) {
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => normalized_file,
    }
}
```

---

## 2.10 `get_stem` mishandles dotfiles

### File

- `crates/shared/src/naming-rules/utility_naming.rs`

### Bug

```rust
if let Some(pos) = filename.rfind('.') {
    Some(&filename[..pos])
}
```

For `.gitignore`, `pos == 0`, so stem becomes `""`.

### Fix

```rust
pub fn get_stem(filename: &str) -> Option<&str> {
    match filename.rfind('.') {
        Some(pos) if pos > 0 => Some(&filename[..pos]),
        Some(_) => Some(filename),
        None => Some(filename),
    }
}
```

---

## 2.11 Cargo working-directory fallback returns a fake nonexistent directory

### File

- `crates/shared/src/external-lint/taxonomy_external_lint_helper.rs`

### Bug

```rust
FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
```

and

```rust
FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
```

This deliberately produces a bogus path when no Cargo manifest is found. That causes confusing downstream execution errors.

### Fix

Fall back to current directory instead:

```rust
pub fn resolve_cargo_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;

    if path_str.is_empty() {
        return path.clone();
    }

    let current = Path::new(path_str);

    if current.is_dir() {
        if current.join("Cargo.toml").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.toml").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }

        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.toml").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }

    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

pub fn resolve_cargo_lock_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;

    if path_str.is_empty() {
        return path.clone();
    }

    let current = Path::new(path_str);

    if current.is_dir() {
        if current.join("Cargo.lock").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.lock").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }

        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.lock").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }

    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}
```

---

## 2.12 `read_text_within_canonical_root` does not canonicalize the root

### File

- `crates/shared/src/config-system/utility_config_io.rs`

### Bug

```rust
let canonical_path = tokio::fs::canonicalize(path).await?;
if !canonical_path.starts_with(canonical_root) {
```

If `canonical_root` is not already canonical, this check can fail incorrectly.

### Fix

```rust
pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> std::io::Result<String> {
    let path = path.as_ref();

    let canonical_root = tokio::fs::canonicalize(canonical_root).await?;
    let canonical_path = tokio::fs::canonicalize(path).await?;

    if !canonical_path.starts_with(&canonical_root) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }

    let meta = tokio::fs::metadata(&canonical_path).await?;

    if !meta.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "config path is not a regular file",
        ));
    }

    if meta.len() > MAX_CONFIG_FILE_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "config file exceeds maximum allowed size",
        ));
    }

    tokio::fs::read_to_string(&canonical_path).await
}
```

---

## 2.13 `is_binary_available` is not portable

### File

- `crates/shared/src/tui/utility_tui_io.rs`

### Bug

```rust
Command::new("sh").args(["-c", &format!("command -v {} ...", bin_name)])
```

This fails on Windows and depends on a shell.

### Fix

Use PATH lookup directly:

```rust
/// Check if a binary is available in the system PATH.
pub fn is_binary_available(bin_name: &str) -> bool {
    if bin_name.is_empty() {
        return false;
    }

    let Ok(path_var) = std::env::var("PATH") else {
        return false;
    };

    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(bin_name);
        if candidate.is_file() {
            return true;
        }

        #[cfg(windows)]
        {
            for ext in ["exe", "cmd", "bat"] {
                let candidate = dir.join(format!("{bin_name}.{ext}"));
                if candidate.is_file() {
                    return true;
                }
            }
        }
    }

    false
}
```

---

# 3. Performance bottlenecks and algorithmic defects

## 3.1 Duplication detector overcounts overlapping windows

### File

- `crates/shared/src/code-analysis/utility_duplication.rs`

### Bug

`scan_duplicate_blocks` uses sliding windows. If two identical 100-line files are compared with `min_lines = 10`, there are 91 duplicated windows per file. The current logic computes:

```rust
let dup_lines: usize = blocks.iter().map(|b| b.len() * min_dup_lines).sum();
```

That can produce duplicate percentages far above 100%, causing false positives.

### Fix

Count unique duplicated lines instead of overlapping window occurrences.

Add import:

```rust
use std::collections::HashSet;
```

Then replace `build_violations` with:

```rust
/// Build violation list from duplicated blocks.
pub fn build_violations(
    blocks: &[Vec<(PathBuf, usize)>],
    total_loc: usize,
    min_dup_lines: usize,
) -> Vec<AesCodeAnalysisViolation> {
    if blocks.is_empty() || total_loc == 0 {
        return Vec::new();
    }

    let mut duplicated_lines: HashSet<(PathBuf, usize)> = HashSet::new();

    for locs in blocks {
        for (path, start) in locs {
            for line in *start..(*start + min_dup_lines) {
                duplicated_lines.insert((path.clone(), line));
            }
        }
    }

    let dup_lines = duplicated_lines.len();
    let pct = dup_lines as f64 / total_loc as f64 * 100.0;

    if pct < 10.0 {
        return Vec::new();
    }

    let mut locations: Vec<String> = blocks
        .iter()
        .flat_map(|b| {
            b.iter()
                .map(|(path, line)| format!("{}:{}", path.display(), line))
        })
        .collect();

    locations.sort();
    locations.dedup();

    vec![AesCodeAnalysisViolation::CodeDuplication {
        reason: Some(LintMessage::new(format!(
            "AES305: Duplicate block ({} lines) at {} — {:.1}% of total across {} occurrence(s).",
            min_dup_lines,
            locations.join(", "),
            pct,
            blocks.iter().map(|b| b.len()).sum::<usize>()
        ))),
    }]
}
```

This prevents:

- duplicate percentage > 100%
- inflated severity from overlapping windows

---

## 3.2 Repeated synchronous process spawn in `is_bun_available`

### File

- `crates/shared/src/external-lint/taxonomy_external_lint_helper.rs`

### Bottleneck

```rust
fn is_bun_available() -> bool {
    std::process::Command::new("bun")...
}
```

This is called from `resolve_js_cmd`, potentially many times during a scan. Each call spawns a process.

### Fix

Cache the result:

```rust
use std::sync::OnceLock;

static BUN_AVAILABLE: OnceLock<bool> = OnceLock::new();

fn is_bun_available() -> bool {
    *BUN_AVAILABLE.get_or_init(|| {
        std::process::Command::new("bun")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    })
}
```

---

## 3.3 Recursive directory walks without adequate ignore handling

### Files

- `crates/shared/src/orphan-detector/utility_orphan_io.rs`
- `crates/shared/src/orphan-detector/utility_workspace.rs`

### Bottleneck

`scan_directory_recursive` and container-wiring scans can descend into heavy directories such as:

- `node_modules`
- `target`
- `dist`
- `build`
- `.venv`
- `__pycache__`

This creates unnecessary I/O and can dominate scan time.

### Fix

#### `utility_orphan_io.rs`

```rust
/// Recursively scan directory for files, returning vector of file paths.
/// Skips hidden directories and common heavy dependency/build directories.
pub fn scan_directory_recursive(dir_path: &Path) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = dir_path.read_dir() {
        for dir_entry in entries.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if name.starts_with('.') {
                    continue;
                }

                let path = dir_entry.path();

                if path.is_dir() {
                    if matches!(
                        name,
                        "target" | "node_modules" | "dist" | "build" | "__pycache__" | ".venv"
                    ) {
                        continue;
                    }

                    files.extend(scan_directory_recursive(&path));
                } else if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    }

    files
}
```

#### `utility_workspace.rs`

Inside `check_dir_containers`, skip heavy directories:

```rust
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if matches!(
                    name,
                    "target" | ".git" | "node_modules" | "dist" | "build" | "__pycache__" | ".venv"
                ) {
                    continue;
                }

                if check_dir_containers(path, identifiers) {
                    return true;
                }
            }
```

---

## 3.4 Thread-local unbounded file cache

### File

- `crates/shared/src/orphan-detector/utility_file_cache.rs`

### Bottleneck / defect

```rust
thread_local! {
    static FILE_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}
```

Problems:

- cache is not shared across threads
- no eviction policy, so memory can grow unbounded during large scans

### Fix

Use a shared bounded cache:

```rust
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use std::collections::HashMap;
use std::fs;
use std::sync::{Mutex, OnceLock};

static FILE_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<String, String>> {
    FILE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

const MAX_CACHE_ENTRIES: usize = 20_000;

pub fn read_cached(path: &FilePath) -> ContentString {
    let mut cache = cache().lock().unwrap_or_else(|e| e.into_inner());

    if let Some(content) = cache.get(path.value()) {
        return ContentString::new(content.clone());
    }

    let content = fs::read_to_string(path.value()).unwrap_or_default();

    if cache.len() < MAX_CACHE_ENTRIES {
        cache.insert(path.value().to_string(), content.clone());
    }

    ContentString::new(content)
}

pub fn read_dir(dir_path: &FilePath) -> Vec<FilePath> {
    let mut entries = Vec::new();

    if let Ok(read_dir) = fs::read_dir(dir_path.value()) {
        for entry in read_dir.flatten() {
            if let Some(s) = entry.path().to_str() {
                if let Ok(fp) = FilePath::new(s) {
                    entries.push(fp);
                }
            }
        }
    }

    entries
}

pub fn path_exists(path: &FilePath) -> bool {
    std::path::Path::new(path.value()).exists()
}

pub fn is_symlink(path: &FilePath) -> bool {
    std::fs::symlink_metadata(path.value())
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

pub fn clear_cache() {
    let mut cache = cache().lock().unwrap_or_else(|e| e.into_inner());
    cache.clear();
}
```

This improves:

- cache reuse across threads
- memory safety under long-running scans

---

## 3.5 `PathUtils::walk_recursive` does not guard against symlink cycles

### File

- `crates/shared/src/common/taxonomy_path_utils_vo.rs`

### Bottleneck / reliability bug

`path.is_dir()` follows symlinks. A symlink cycle can cause unbounded recursion.

### Fix

Add a visited canonical-path set:

```rust
use std::collections::HashSet;
use std::fs;

impl PathUtils {
    pub fn walk_recursive(dir: &std::path::Path, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        let root = fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
        let mut visited = HashSet::new();
        Self::walk_recursive_internal(&root, &root, ignored, &mut visited)
    }

    fn walk_recursive_internal(
        root: &std::path::Path,
        dir: &std::path::Path,
        ignored: &[&str],
        visited: &mut HashSet<std::path::PathBuf>,
    ) -> Vec<std::path::PathBuf> {
        let mut results = Vec::new();

        if !dir.is_dir() {
            if dir.is_file() {
                if let Some(name_str) = dir.file_name().and_then(|s| s.to_str()) {
                    if !ignored.contains(&name_str) {
                        let rel_path = dir.strip_prefix(root).unwrap_or(dir);
                        let rel_str = rel_path.to_string_lossy();

                        if !Self::matches_any_pattern(&rel_str, ignored) {
                            results.push(dir.to_path_buf());
                        }
                    }
                }
            }
            return results;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let rel_path = path.strip_prefix(root).unwrap_or(&path);
                let rel_str = rel_path.to_string_lossy();

                if Self::matches_any_pattern(&rel_str, ignored) {
                    continue;
                }

                if path.is_dir() {
                    let canonical = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());

                    if !visited.insert(canonical) {
                        continue;
                    }

                    results.extend(Self::walk_recursive_internal(root, &path, ignored, visited));
                } else {
                    results.push(path);
                }
            }
        }

        results
    }

    fn matches_any_pattern(rel_path: &str, ignored: &[&str]) -> bool {
        for pattern in ignored {
            if rel_path == *pattern || rel_path.starts_with(&format!("{}/", pattern)) {
                return true;
            }

            if let Some(file_name) = std::path::Path::new(rel_path).file_name() {
                if file_name == *pattern {
                    return true;
                }
            }
        }

        false
    }

    pub fn collect_paths(start: &str, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        let root = std::path::Path::new(start);
        Self::walk_recursive(root, ignored)
    }
}
```

---

# 4. Additional correctness hardening worth applying

## 4.1 `extract_trait_method_signatures` has a suspicious trait-header condition

### File

- `crates/shared/src/common/utility_signature_parser.rs`

### Bug

```rust
&& line.contains(')').ge(&line.contains('('));
```

This boolean comparison is semantically unclear and can cause false negatives for trait headers with unbalanced parentheses on the same line.

### Safer fix

Use a clearer header check:

```rust
let is_trait_header = (line.starts_with("pub trait ") || line.starts_with("trait "))
    && line.contains('{');
```

If multi-line trait headers must be supported, a more robust parser is needed, but this change removes the questionable boolean ordering logic.

---

# 5. Summary of highest-impact fixes

If you want the fastest stabilization pass, apply these first:

1. **Remove invalid `bool` imports** in `external-lint` contracts.
2. **Fix `build.rs` error handling** to avoid panic-driven build failures.
3. **Fix `contains_delimited` UTF-8 panic** in orphan detection.
4. **Fix symlink file collection bug** in `utility_file.rs`.
5. **Fix duplication overcounting** in `utility_duplication.rs`.
6. **Fix ignore-pattern false positives** in `utility_file.rs`.
7. **Fix alias handling** in Python/JS import detection.
8. **Fix substring-based symbol usage checks** to use identifier boundaries.
9. **Cache `bun` availability** to avoid repeated process spawns.
10. **Add visited-set protection** to recursive path walking.

---

If you want, I can next provide this as a **single consolidated patch** in unified diff format for the entire `shared` crate.
