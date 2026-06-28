---
name: Master Audit Patch v1.10.14
about: Validated bug fixes and performance optimizations for code-analysis crate
title: '[AUDIT] code-analysis v1.10.14 — Validated Patches from 6-AI Cross-Review'
labels: bug, performance, code-analysis, audit, priority: high
assignees: ''
---

# Master Audit & Patch Report: code-analysis v1.10.14

## Executive Summary

Validasi patch dari 6 AI (Qwen, Mimo, DeepSeek, Gemini, Kimi, GLM) terhadap source code aktual. Dari 18 fix yang diusulkan, **10 VALID** dan **3 DEBATABLE**. 5 fix INVALID dihapus dari issue ini karena akan introduce regression.

**Catatan untuk reviewer:** 3 bug kritis yang ditemukan saat validasi (brace-depth `#[cfg(test)]`, Python multi-line empty class, JS/TS `rfind('{')`) sudah di-skip. Patch untuk masalah tersebut perlu implementasi baru yang terpisah.

---

## Validation Status

| Fix                             | Module              | Status       | File:Line                                 |
| ------------------------------- | ------------------- | ------------ | ----------------------------------------- |
| 1.1 Remove async/Tokio          | Orchestrator        | ✅ VALID     | `orchestrator.rs:145-149`                 |
| 1.2 Pre-read files              | Orchestrator        | ✅ VALID     | `orchestrator.rs:189`                     |
| 1.3 `lint_path` → `run_scan`    | Orchestrator        | ⚠️ DEBATABLE | `orchestrator.rs:91`                      |
| 1.4 Skip unreadable files       | Orchestrator        | ✅ VALID     | `orchestrator.rs:189`                     |
| 2.1 Add `#[warn(...)]`          | BypassChecker       | ⚠️ DEBATABLE | `bypass_checker.rs:307-315`               |
| 2.2 Support `[lints.clippy]`    | BypassChecker       | ✅ VALID     | `bypass_checker.rs:323`                   |
| 2.4 Brace-depth static Lazy     | BypassChecker       | ✅ VALID     | `bypass_checker.rs:367-376`               |
| 2.5 Hoist `.to_lowercase()`     | BypassChecker       | ✅ VALID     | `bypass_checker.rs:407`                   |
| 3.1 u32 String Interning        | DuplicationAnalyzer | ✅ VALID     | `duplication_analyzer.rs:69`              |
| 3.2 O(1) file_to_others map     | DuplicationAnalyzer | ✅ VALID     | `duplication_analyzer.rs:108-121`         |
| 3.3 Config parsing AES305       | DuplicationAnalyzer | ✅ VALID     | `duplication_analyzer.rs:171-191`         |
| 3.4 Pre-read buffer API         | DuplicationAnalyzer | ✅ VALID     | `duplication_analyzer.rs:56-63`           |
| 4.1 Skip `main.rs`              | MandatoryDefChecker | ✅ VALID     | `mandatory_def_checker.rs:52-57`          |
| 4.2 `pub(crate)` struct         | MandatoryDefChecker | ✅ VALID     | `mandatory_def_checker.rs:73-88`          |
| 4.4 Tuple struct exclusion      | MandatoryDefChecker | ✅ VALID     | `mandatory_def_checker.rs:121`            |
| 5.1 Collapse repeated slashes   | PathVO              | ✅ VALID     | `taxonomy_path_vo.rs:27`                  |
| 5.2 Multi-segment bare patterns | FileCollector       | ✅ VALID     | `taxonomy_file_collector_helper.rs:82-85` |

---

## Module 1: Orchestrator I/O & Async Removal

**File:** `crates/code-analysis/src/agent_code_analysis_orchestrator.rs`

### Fix 1.1: Remove fake async & Tokio runtime ✅

`run_all_checks` didefinisikan `pub async fn` (line 156) tapi TIDAK ada `await` di dalamnya. Tokio runtime dibuat hanya untuk `block_on` — murni overhead.

**Patch:**

- Hapus `async` dari `run_all_checks` signature
- Hapus `tokio::runtime::Runtime::new()` dan `block_on` di `run_lint_at` (line 145-149)
- Langsung panggil `self.run_all_checks(config, &files_str, &root_dir)`

### Fix 1.2: Pre-read files to avoid Double I/O ✅

Line 189: `std::fs::read_to_string(file)` dipanggil di dalam loop. Lalu line 230: `check_file_similarity(files, ...)` re-read semua file lagi.

**Patch:**

- Collect `(path, content)` pairs di awal loop sebelum checker calls
- Skip file yang gagal dibaca (`Err(_) => continue`)
- Pass entries ke `check_file_similarity_entries()` (lihat Fix 3.4)

### Fix 1.3: `lint_path` → `run_scan` ⚠️ DEBATABLE

Line 91: `orchestrator.run_self_lint(&root.value)` memanggil `detect_source_dir` yang mencari `crates/`/`packages/`/`modules/`. Untuk arbitrary path, `run_scan` lebih tepat.

**Assessment:** Behavior change, bukan bug fix. `run_self_lint` punya fallback (line 48: `project_root.to_path_buf()`). Rekomendasi: terapkan, tapi document behavior change.

### Fix 1.4: Skip unreadable files ✅

Line 189: `unwrap_or_default()` mengembalikan string kosong untuk unreadable files, yang bisa trigger false AES302/AES303.

**Patch:** Ganti `unwrap_or_default()` dengan `match` — skip file yang gagal dibaca.

---

## Module 2: Bypass Checker Correctness

**File:** `crates/code-analysis/src/capabilities_check_bypass_checker.rs`

### Fix 2.1: Add `#[warn(...)]` detection ⚠️ DEBATABLE

Line 307-315: `starts_with_allow_attr` hanya punya `allow` dan `expect`.

**Assessment:** `#[warn(...)]` bukan bypass — di context default, `#[warn]` justru memperketat. Hanya bypass jika ada `#[deny]`/`#[forbid]` di parent scope. Rekomendasi: jangan tambah ke bypass list, atau implement context-aware logic.

### Fix 2.2: Support `[lints.clippy]` sections ✅

Line 323: hanya check `[workspace.lints.clippy]`. Rust 1.74+ mendukung `[lints.clippy]` di package level.

**Patch:**

```rust
if t.starts_with("[workspace.lints.clippy]")
    || t.starts_with("[lints.clippy]") {
    in_clippy_section = true;
    continue;
}
```

**CATATAN:** `[package.lints.clippy]` yang diusulkan master report BUKAN section TOML valid. Hanya `[lints.clippy]`.

### Fix 2.4: Brace-depth untuk static Lazy ✅

Line 367-376: `in_static_lazy` flag di-reset saat `});` ditemukan. Tapi miss visibility modifier: `pub static`, `pub(crate) static`.

**Patch:**

```rust
if t.contains("static ") && t.contains("Lazy") {
    in_static_lazy = true;
    lazy_brace_depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
    if lazy_brace_depth <= 0 { in_static_lazy = false; lazy_brace_depth = 0; }
    continue;
}
```

### Fix 2.5: Hoist `.to_lowercase()` ✅

Line 407: `t.to_lowercase().contains(&p_str.to_lowercase())` di dalam loop pattern.

**Patch:**

- Hoist `let t_lower = t.to_lowercase();` sebelum loop pattern
- Pre-compute `p_str.to_lowercase()` untuk setiap pattern (di constructor)

---

## Module 3: Duplication Analyzer Performance

**File:** `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs`

### Fix 3.1: u32 String Interning ✅

Line 69: `HashMap<String, Vec<(usize, usize)>>` — setiap window key adalah String yang di-clone millions of times.

**Patch:**

```rust
let mut interner: HashMap<String, u32> = HashMap::new();
let mut get_id = |s: String| -> u32 {
    let len = interner.len() as u32;
    *interner.entry(s).or_insert(len)
};
// Global map jadi: HashMap<Vec<u32>, Vec<(usize, usize)>>
```

### Fix 3.2: O(1) `file_to_others` map ✅

Lines 108-121: nested loop O(N^2) untuk collect `other_files`.

**Patch:**

```rust
let mut file_to_others: Vec<HashSet<usize>> = vec![HashSet::new(); entries.len()];
for locs in global.values() {
    let unique: HashSet<usize> = locs.iter().map(|(fi, _)| *fi).collect();
    if unique.len() > 1 {
        for &fi in &unique {
            for &other in &unique {
                if other != fi { file_to_others[fi].insert(other); }
            }
        }
    }
}
```

### Fix 3.3: Config parsing AES305 ✅

Lines 171-191: `handle_duplicates` menggunakan `config.rules.first()` (arbitrary) dan `r.rule_type.to_string() == "AES305"` (inconsistent field). `min_lines` (integer) digunakan sebagai percentage threshold (type mismatch).

**Patch:**

```rust
let min_lines = config.rules.iter()
    .find(|r| r.name.value == "AES305")
    .map(|r| r.code_analysis.min_lines.value as usize)
    .unwrap_or(10);
let threshold_pct = config.rules.iter()
    .find(|r| r.name.value == "AES305")
    .and_then(|r| r.code_analysis.duplication_threshold)
    .unwrap_or(50.0);
```

### Fix 3.4: Pre-read buffer API ✅

Tambah method baru untuk accept pre-read entries:

```rust
pub fn check_file_similarity_entries(
    &self,
    entries: &[(PathBuf, String)],
    min_dup_lines: usize,
    threshold_pct: f64,
) -> Vec<(String, AesCodeAnalysisViolation)>
```

---

## Module 4: Mandatory Definition Checker

**File:** `crates/code-analysis/src/capabilities_mandatory_definition_checker.rs`

### Fix 4.1: Skip `main.rs` ✅

Line 52-57: skip list tidak ada `main.rs`. Binary entry points tidak perlu mandatory class definition.

**Patch:**

```rust
if matches!(
    basename.as_str(),
    "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs" | "main.rs"
) { return; }
```

### Fix 4.2: `pub(crate)` struct detection ✅

Lines 73-88: `has_class` hanya cek `pub struct`, `struct` — miss `pub(crate) struct`, `pub(super) struct`.

**Patch:** Gunakan helper function:

```rust
fn rust_declares_type(line: &str) -> bool {
    let keywords = ["struct", "enum", "trait"];
    for kw in keywords {
        if line.contains(kw) && !line.contains('(') {
            return true;
        }
    }
    false
}
```

### Fix 4.4: Tuple struct exclusion ✅

Line 121: `t.starts_with("struct ") && t.ends_with(';')` — Tuple structs `struct Foo(i32);` ikut terdeteksi.

**Patch:**

```rust
if t.starts_with("struct ") && t.ends_with(';') && !t.contains('(') {
    // only unit structs, not tuple structs
}
```

---

## Module 5: Path & Collection Fixes

**File:** `crates/shared/src/common/taxonomy_path_vo.rs` dan `taxonomy_file_collector_helper.rs`

### Fix 5.1: Collapse repeated slashes ✅

`taxonomy_path_vo.rs:27`: `value.replace('\\', "/")` tidak collapse `//`.

**Patch (single pass untuk performance):**

```rust
let mut normalized = String::with_capacity(value.len());
let mut prev_slash = false;
for c in value.chars() {
    if c == '/' || c == '\\' {
        if !prev_slash { normalized.push('/'); prev_slash = true; }
    } else {
        normalized.push(c);
        prev_slash = false;
    }
}
value = normalized;
```

### Fix 5.2: Multi-segment bare patterns ✅

`taxonomy_file_collector_helper.rs:82-85`: `segments.contains(&pat.as_str())` hanya match single segments.

**Patch:**

```rust
let pat_segments: Vec<&str> = pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
if pat_segments.len() == 1 {
    if segments.contains(&pat_segments[0]) { return true; }
} else if pat_segments.len() > 1 {
    let n_pat = pat_segments.len();
    let n_seg = segments.len();
    if n_seg >= n_pat {
        for start in 0..=(n_seg - n_pat) {
            if segments[start..start + n_pat] == pat_segments[..] { return true; }
        }
    }
}
```

---

## Implementation Plan

### Phase 1: Safe Patches (no behavior change)

1. Fix 1.1 — Remove async/Tokio
2. Fix 1.4 — Skip unreadable files
3. Fix 2.5 — Hoist `.to_lowercase()` (both `t` and `p`)
4. Fix 3.1 — u32 String Interning
5. Fix 3.2 — O(1) file_to_others map
6. Fix 3.3 — Config parsing AES305
7. Fix 3.4 — Pre-read buffer API
8. Fix 4.1 — Skip `main.rs`
9. Fix 4.4 — Tuple struct exclusion
10. Fix 5.1 — Collapse repeated slashes (single pass)
11. Fix 5.2 — Multi-segment bare patterns

### Phase 2: Correctness Fixes (with proper implementation)

12. Fix 2.2 — Support `[lints.clippy]`
13. Fix 2.4 — Brace-depth static Lazy (with visibility modifier)
14. Fix 4.2 — `pub(crate)` struct detection

### Phase 3: Debatable Changes (needs discussion)

15. Fix 1.3 — `lint_path` → `run_scan`
16. Fix 2.1 — `#[warn(...)]` detection

---

## Test Criteria

### Unit Tests

```bash
cargo test -p code_analysis_lint_arwaky
```

### Integration Tests

```bash
cargo run --bin lint-arwaky-cli -- scan test-workspaces
```

### Manual Verification

- [ ] `[lints.clippy]` section di Cargo.toml terdeteksi sebagai bypass
- [ ] `static Lazy<Regex>` multi-line dengan `pub static` terdeteksi
- [ ] File yang tidak readable di-skip tanpa error
- [ ] Duplication check tidak re-read files (single I/O)
- [ ] `struct Foo(i32);` (tuple struct) TIDAK terdeteksi sebagai dead inheritance
- [ ] `pub(crate) struct Bar {}` terdeteksi sebagai valid definition
- [ ] `main.rs` di-skip dari mandatory definition check
- [ ] `//path//with//slashes` di-normalize ke `/path/with/slashes`
- [ ] Bare pattern `"foo/bar"` match path `src/foo/bar/baz.rs`

### Performance Benchmark

```bash
# Before: cargo run --bin lint-arwaky-cli -- scan large-project/ --duration
# After:  cargo run --bin lint-arwaky-cli -- scan large-project/ --duration
# Expected: 30-50% reduction for large projects
```

---

## Deferred Fixes (not in this issue)

Patch berikut diidentifikasi INVALID dan perlu implementasi baru:

- Brace-depth `#[cfg(test)]` — perlu `pending_test_attr` flag
- Python empty class multi-line — pertahankan branch yang sudah ada
- JS/TS empty class — gunakan `find('{')` bukan `rfind`
- `matches_word_token` rewrite — perlu implementasi lengkap

Issues terpisah akan dibuat untuk fix-fix ini.
