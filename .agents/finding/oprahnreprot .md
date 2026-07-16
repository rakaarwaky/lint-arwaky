Based on a thorough code review of the `orphan-detector` crate, I have identified two functional bugs that compromise the accuracy of the orphan detection logic. Below are the explanations and the fixed code snippets.

### 1. Import Graph Pollution in Python/JS Fallback Resolution

**File:** `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

**Explanation:**
In the `build_graph_context_inner` method, when the resolver processes Python/JS-style imports (e.g., `from shared.common... import X`) that fail to resolve to an absolute file path, it falls through to a "relative imports" block. In this block, the code extracts the root module name (e.g., `"shared"`) and pushes it directly into the `import_graph` and `inbound_links` maps.

Since the graph traversal (BFS) expects **file paths** as keys and values, injecting directory/module names like `"shared"` pollutes the graph. The BFS will never find neighbors for `"shared"` because it's not a valid file path key, causing the reachability trace to silently fail or break for legitimate dependencies.

**Fixed Code:**
Replace the fallback block with logic that only adds resolved file paths to the graph. If the import refers to a workspace module that cannot be resolved to a specific file, it should be safely ignored rather than polluting the graph.

```rust
                 // Python/JS relative imports or local module imports
                 if let Some(resolved_path) = module_to_file.get(&dep) {
                     if resolved_path != f {
                         import_graph.entry(f.clone()).or_default().push(resolved_path.clone());
                         inbound_links.entry(resolved_path.clone()).or_default().push(f.clone());
                     }
                 }
                 // If it's a workspace module (e.g., "shared") but not resolved to a specific file,
                 // we do not add it to the import graph to avoid polluting it with directory names.
```

---

### 2. False Negative in Taxonomy Orphan Detection (Self-Import Bug)

**File:** `crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs`

**Explanation:**
The `check_taxonomy_orphan` function iterates through all project files to see if the taxonomy file's stem (e.g., `taxonomy_foo_vo`) is contained within them. However, the loop does not skip the file itself (`cf == fp`).

Since a file often contains its own stem in comments, module declarations, or struct names, the condition `c.contains(&stem)` will evaluate to `true` when the loop reaches the file itself. This falsely marks the file as "imported," causing valid orphaned taxonomy files to be incorrectly classified as "used" (false negatives).

**Fixed Code:**
Add a self-reference check at the beginning of the loop to ensure a file is not considered "imported" by itself.

```rust
pub fn check_taxonomy_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let stem = basename
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "");
    let suffix = match stem.rfind('_') {
        Some(pos) => &stem[pos + 1..],
        None => "",
    };
    let is_utility_or_helper = matches!(suffix, "utility" | "helper");
    let mut imported = false;
    for cf in files {
        // FIX: Skip self to avoid false negatives where the file matches its own stem
        if cf == fp {
            continue;
        }
        let cb = match cf.split('/').next_back() {
            Some(b) => b,
            None => continue,
        };
        if is_utility_or_helper {
            // utility/helper: can be imported directly by any layer, no contract needed
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&stem) {
                    imported = true;
                    break;
                }
            }
        } else {
            // vo, entity, error, event, constant: must be imported via contract layer
            if !cb.starts_with("contract_") {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&stem) {
                    imported = true;
                    break;
                }
            }
        }
    }
    if !imported {
        let category = if is_utility_or_helper {
            "utility"
        } else {
            "vo"
        };
        let reason = if is_utility_or_helper {
            format!("Taxonomy '{}' is not imported by any file.", stem)
        } else {
            format!("Taxonomy '{}' is not imported by any contract.", stem)
        };
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &AesOrphanViolation::TaxonomyOrphan {
                stem: stem.clone(),
                category,
                reason: Some(reason.into()),
            }
            .to_string(),
            Severity::LOW,
            "AES501",
        ));
    }
}
```

Ya, setelah melakukan inspeksi yang lebih dalam (deep-dive) pada logika analitik dan resolusinya, saya menemukan **3 bug fungsional kritis tambahan** yang sangat memengaruhi akurasi deteksi, terutama untuk proyek multi-bahasa dan multi-crate.

Berikut adalah penjelasan dan perbaikan kodenya:

### 3. False Positive pada Deteksi Implementasi Kontrak (Python/TS)

**File:** `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

**Penjelasan:**
Pada fungsi `is_contract_orphan`, terdapat logika "Check 1" yang memverifikasi apakah sebuah kontrak (trait/interface) telah diimplementasikan oleh layer target (capabilities/infrastructure). Namun, pencocokan string untuk Python dan TypeScript sangat cacat:

- Untuk **Python**, kode mencari string literal `class TraitName(\(` atau `class TraitName:`. Ini hanya akan cocok dengan _definisi_ kontrak itu sendiri, bukan _implementasinya_ (yang seharusnya berbentuk `class MyImpl(TraitName):`).
- Untuk **TypeScript**, kode sama sekali tidak memeriksa kata kunci `implements` atau `extends`.

Akibatnya, implementasi kontrak yang valid di Python dan TS tidak akan pernah terdeteksi oleh Check 1. Fungsi akan langsung return lebih awal dan secara keliru menandai kontrak yang aktif digunakan sebagai orphan (AES502).

**Fixed Code:**
Ganti blok `if let Ok(c) = std::fs::read_to_string(cf)` di dalam loop "Check 1" dengan logika regex dan string matching yang benar untuk Python/TS:

```rust
         if let Ok(c) = std::fs::read_to_string(cf) {
             let has_rust_impl = c.contains(&format!("impl {} for", trait_name))
                 || c.lines().any(|ln| {
                     let t = ln.trim();
                     t.starts_with("impl") && t.contains(&trait_name) && t.contains(" for")
                 });

             // Python: class MyClass(TraitName): atau class MyClass(Base, TraitName):
             let py_pattern = format!(r"class\s+\w+\([^)]*\b{}\b[^)]*\)", regex::escape(trait_name));
             let has_py_impl = regex::Regex::new(&py_pattern)
                 .map(|re| re.is_match(&c))
                 .unwrap_or(false);

             // TypeScript: class MyClass implements TraitName atau extends TraitName
             let has_ts_impl = c.contains(&format!("implements {}", trait_name))
                 || c.contains(&format!("extends {}", trait_name));

             if has_rust_impl || has_py_impl || has_ts_impl {
                 has_impl = true;
                 break;
             }
         }
```

---

### 4. Hardcoded Current Working Directory (CWD) pada Deteksi Surface Orphan

**File:** `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs`

**Penjelasan:**
Di dalam fungsi `is_surface_orphan`, analyzer mencoba mencari _workspace root_ menggunakan `std::path::Path::new(".")`. Ini sangat tidak dapat diandalkan karena bergantung sepenuhnya pada _Current Working Directory_ (CWD) dari mana CLI dieksekusi.

Jika pengguna menjalankan linter dari sub-direktori (misal: `cd crates/my-app && lint-arwaky scan`) atau jika CI/CD pipeline mengatur CWD di luar tree proyek, `find_workspace_root` akan gagal menemukan marker (seperti `crates/` atau `Cargo.toml`). Hal ini menyebabkan analyzer gagal memverifikasi apakah surface diimpor oleh entry point/router, sehingga memicu peringatan orphan palsu (AES506).

**Fixed Code:**
Gunakan _parent directory_ dari file yang sedang dianalisis sebagai titik awal pencarian, alih-alih bergantung pada CWD:

```rust
         // FIX: Gunakan parent directory dari file yang sedang dianalisis
         let file_parent = std::path::Path::new(fp_val).parent().unwrap_or(std::path::Path::new("."));
         if let Ok(workspace_root) =
             crate::capabilities_orphan_capabilities_analyzer::find_workspace_root(file_parent)
         {
             if let Ok(imported) = check_imported_by_entry_or_router(&workspace_root, &stem) {
                 if imported {
                     return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                 }
             }
             // ... (lanjutkan logika existing untuk memeriksa identifiers)
```

---

### 5. Kegagalan Resolusi Modul Direktori Lintas-Crate (Cross-Crate)

**File:** `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

**Penjelasan:**
Saat mereseleksi import lintas-crate (misal: `use shared::code_analysis::...`), resolver secara manual melakukan iterasi pada direktori `src/` crate target dan hanya memeriksa apakah `stem == module_name`.

Logika ini gagal total untuk **modul direktori standar Rust** di mana modul direpresentasikan oleh sebuah direktori yang berisi file `mod.rs` (misal: `shared/src/code-analysis/mod.rs`). Karena resolver hanya mencari file dengan stem `code_analysis` dan mengabaikan sub-direktori, dependensi lintas-crate yang valid tidak dimasukkan ke dalam `import_graph`. Ini merusak tracing reachability (BFS) secara masif untuk proyek Rust multi-crate.

**Fixed Code:**
Perbarui blok `read_dir` pada bagian _Workspace crate import resolution_ untuk juga memeriksa keberadaan `mod.rs` di dalam sub-direktori:

```rust
                         if let Some(src_dir) = crate_src_dirs.get(crate_name) {
                             if let Ok(entries) = std::fs::read_dir(src_dir) {
                                 for entry in entries.flatten() {
                                     let path = entry.path();
                                     if path.is_dir() {
                                         // FIX: Cek modul direktori (misal: code-analysis/mod.rs)
                                         let mod_rs = path.join("mod.rs");
                                         if mod_rs.exists() {
                                             let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
                                             let normalized_dir = dir_name.replace('-', "_");
                                             if normalized_dir == module_name {
                                                 let path_str = mod_rs.to_string_lossy().to_string();
                                                 if path_str != *f {
                                                     import_graph.entry(f.clone()).or_default().push(path_str.clone());
                                                     inbound_links.entry(path_str).or_default().push(f.clone());
                                                 }
                                             }
                                         }
                                     } else if let Some(path_str) = path.to_str() {
                                         let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
                                         if stem == module_name && path_str != *f {
                                             import_graph.entry(f.clone()).or_default().push(path_str.to_string());
                                             inbound_links.entry(path_str.to_string()).or_default().push(f.clone());
                                         }
                                     }
                                 }
                             }
                         }
```
