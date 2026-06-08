# 📄 Feature Requirements Document (FRD)
**Feature Name:** Primitive Usage Checker (AES006)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v1.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
Dokumen ini mendefinisikan aturan **AES006 (PRIMITIVE_USAGE)** yang melarang penggunaan tipe primitif mentah (seperti `String`, `i32`, `bool`, `str`, `int`, `float`) dalam domain types tertentu. Aturan ini memastikan bahwa entity, error, event, dan contract interface menggunakan Value Objects (_vo) sebagai pengganti primitif, sesuai prinsip Domain-Driven Design.

### 2.2 Scope
**In-Scope:**
- Deteksi primitif di file taxonomy(entity), taxonomy(error), taxonomy(event)
- Deteksi primitif di file contract(port), contract(protocol)
- Pengecualian untuk taxonomy(vo) dan taxonomy(constant) — boleh pakai primitif
- Tiga bahasa: Rust, Python, JavaScript/TypeScript
- Severity HIGH

**Out-of-Scope:**
- File di luar taxonomy dan contract (capabilities, infrastructure, agent, surfaces)
- Auto-fixing (tidak auto-fixable)
- Penggantian primitif dengan VO secara otomatis

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES006** | Rule code untuk primitive usage violation |
| **check_primitive_usage()** | Inline checker method (aktif) di `lint_checking_coordinator.rs` |
| **DomainTypeRuleChecker** | Capability checker (tidak dipakai) di `domain_type_checker.rs` |
| **no_primitives** | Config flag yang menentukan scope mana yang dicek |
| **CORE_PRIMITIVE_TYPES** | Constant daftar primitif di `naming_symbols_constant.rs` |
| **VO (Value Object)** | Tipe domain yang membungkus primitif dengan aturan bisnis |

## 3. Feature Overview
### 3.1 Background & Problem
Sebelum AES006, domain entity, error, dan event bisa menggunakan tipe primitif langsung seperti `String name` atau `int age`. Ini melanggar prinsip Domain-Driven Design di mana semua domain type harus menggunakan Value Objects yang memiliki validasi dan aturan bisnis. Tidak ada mekanisme yang secara otomatis mendeteksi pelanggaran ini.

### 3.2 Business Goals
- Memastikan semua domain entity menggunakan Value Objects
- Memastikan domain error dan event tidak bocor ke primitif
- Contract interface harus menggunakan domain types, bukan primitif
- Memberikan pengecualian untuk VO dan Constant (exempt)

### 3.3 Target Users
- **Developers**: Mendapat feedback ketika menggunakan primitif di domain types
- **Domain Architects**: Memastikan DDD purity dalam taxonomy layer

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** Sebagai developer, saya ingin diperingatkan ketika saya menggunakan `String` atau `i32` di entity, sehingga saya membuat Value Object yang tepat.
- **US-002:** Sebagai developer, saya TIDAK ingin diperingatkan ketika saya menggunakan primitif di VO file, karena VO justru membungkus primitif.
- **US-003:** Sebagai architect, saya ingin mengaktifkan/menonaktifkan aturan ini per scope via YAML.

### 4.2 Use Cases & Workflow
**Detection Pipeline (saat ini - inline checker):**
```
File: taxonomy/massive_domain_entity.rs

1. Apakah path mengandung "/taxonomy/"? → YA ✅
2. Untuk setiap baris:
   a. Apakah baris mengandung ":" dan diakhiri "," atau "}"? → YA (field definition)
   b. Ambil tipe setelah ":"
   c. Apakah tipe termasuk ["String","i32","bool",...]? → YA
   d. Flag AES006 HIGH
```

**Detection Pipeline (seharusnya - berdasarkan config):**
```
File: taxonomy/massive_domain_entity.rs

1. Deteksi layer: taxonomy(entity)
2. Apakah no_primitives = true? → YA ✅ (config line 190)
3. Parse file dengan AST scanner → dapatkan semua field types
4. Apakah ada field dengan tipe primitif? → String, i32 — YA ❌
5. Flag AES006 HIGH
```

### 4.3 Business Rules
- Severity: HIGH
- Scope yang dicek: `taxonomy(entity)`, `taxonomy(error)`, `taxonomy(event)`, `contract`
- Scope yang TIDAK dicek: `taxonomy(vo)`, `taxonomy(constant)`, `capabilities`, `infrastructure`, `agent`, `surfaces`
- Pesan violation dari YAML per scope
- Primitive list untuk Rust: `String, i8-i128, u8-u128, f32, f64, bool, char, Vec<, HashMap<, Option<, Result<`
- Primitive list untuk Python: `str, int, float, bool, list, dict, tuple, set`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 20ms |
| NFR-002 | False positive rate (VO files) | 0% |
| NFR-003 | Cross-language support | Rust, Python, JS/TS |
| NFR-004 | Config-driven scope | Sesuai `no_primitives` di YAML |

## 6. UI/UX Requirements
```
AES006 HIGH - test-project-rust/src-rust/taxonomy/massive_domain_entity.rs:5
  AES006 PRIMITIVE_USAGE: Direct primitive 'String' in taxonomy.

AES006 HIGH - test-project-python/src-python/taxonomy/raw_entity.py:5
  AES006 PRIMITIVE_USAGE: Direct primitive 'str' in taxonomy.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Rust entity file pakai `String`, `i32` | Check primitive usage | AES006 HIGH flagged | ✅ (inline checker) |
| AC-002 | Python entity file pakai `str`, `int` | Check primitive usage | AES006 HIGH flagged | ❌ **Tidak dicek oleh inline checker** |
| AC-003 | JS/TS entity file pakai `string`, `number` | Check primitive usage | AES006 HIGH flagged | ❌ **Tidak dicek sama sekali** |
| AC-004 | Rust VO file pakai `String` | Check primitive usage | **TIDAK** flagged (exempt) | ❌ **Inline checker tetap flag — false positive** |
| AC-005 | Rust constant file pakai primitif | Check primitive usage | **TIDAK** flagged (exempt) | ❌ **Inline checker tetap flag — false positive** |
| AC-006 | File di luar taxonomy | Check primitive usage | **TIDAK** dicek | ✅ (inline checker early return) |
| AC-007 | Config `no_primitives: false` untuk satu scope | Check primitive usage | **TIDAK** flagged untuk scope itu | ❌ **Config flag tidak pernah dibaca** |

## 8. Temuan Empiris (Code Audit)

### 8.1 Ada 4 (Empat) Implementasi, Hanya 1 yang Aktif

#### 8.1.1 Inline Checker (AKTIF) — `lint_checking_coordinator.rs:177-191`
```rust
fn check_primitive_usage(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    if !file.contains("/taxonomy/") { return; }
    let primitives = ["String","i8","i16",...,"Result<"];
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.contains(':') && (t.ends_with(',') || t.ends_with('}')) {
            let ft = t.split(':').nth(1).unwrap_or("").trim().trim_end_matches(',').trim_end_matches('}');
            for p in &primitives { if ft.starts_with(p) || ft == p.trim_end_matches('<') {
                violations.push(...);
            }}
        }
    }
}
```
**Status**: ✅ Aktif dipanggil di `run_all_checks()` line 47.

#### 8.1.2 DomainTypeRuleChecker (ORPHAN) — `domain_type_checker.rs:24-92`
```rust
pub fn find_primitive_violations(&self, file_path: &str, primitive_types: &[&str]) -> Vec<PrimitiveViolation>
```
**Status**: ❌ **Tidak pernah dipanggil** oleh siapa pun. Kode mati (orphan).

#### 8.1.3 AST Scanners (UNUSED for AES006) — `ast_rust_scanner.rs`, `ast_py_scanner.rs`, `ast_js_scanner.rs`
Semua scanner implement `find_primitive_violations()` via `ISourceParserPort`.
**Status**: ❌ **Tidak pernah dipanggil untuk AES006**. Scanners hanya digunakan untuk parsing import, symbol detection, dll.

#### 8.1.4 PythonPrimitiveChecker (STUB) — `python_primitive_checker.rs`
```rust
pub struct PythonPrimitiveChecker;
impl PythonPrimitiveChecker {
    pub fn new() -> Self { Self }
}
```
**Status**: ❌ **Hollow stub** — 9 baris, struct kosong tanpa method. Rencana awal untuk generated Python primitive checker tapi tidak pernah diimplementasi.

### 8.2 Bug yang Ditemukan

1. **`no_primitives` config flag TIDAK PERNAH dibaca oleh checker** (KRITIS)
   - Config YAML mendefinisikan `no_primitives: true/false` per scope (lines 184-242)
   - Tapi inline checker di `lint_checking_coordinator.rs:177-191` punya primitive list hardcoded
   - `LayerDefinition.no_primitives` di-set di `architecture_compliance_orchestrator.rs:385-387` tapi tidak pernah dikonsumsi
   - Ini menyebabkan **false positive untuk VO dan Constant** yang seharusnya exempt

2. **VO dan Constant tetap di-flag** (KRITIS)
   - Config: `taxonomy(vo): no_primitives: false` (line 186)
   - Config: `taxonomy(constant): no_primitives: false` (line 215)
   - Tapi inline checker tetap mengecek SEMUA file di `/taxonomy/` tanpa diskriminasi

3. **Hanya support Rust** (KRITIS)
   - Primitive list hardcoded untuk Rust: `String, i32, Vec<`, dll.
   - Python file di `/taxonomy/` tidak akan terdeteksi primitifnya
   - JS/TS file tidak terdeteksi sama sekali

4. **Heuristic regex rapuh**
   - `line.contains(':') && (line.ends_with(',') || line.ends_with('}'))` — hanya cocok untuk struct field
   - Tidak menangkap: tuple struct fields, function signatures, const/static types, associated types
   - `fn process(&self) -> bool { true }` akan di-flag karena `: bool` dan `}` ✅ (actual match)
   - Tapi `let x: String = ...` tidak akan di-flag ❌ (false negative)

5. **`CORE_PRIMITIVE_TYPES` tidak dipakai**
   - `naming_symbols_constant.rs:10`: `pub const CORE_PRIMITIVE_TYPES: &[&str] = &["str", "int", "float"]`
   - Define untuk Python tapi tidak direferensi oleh checker manapun

6. **`architecture_internal_checker.rs:145` — TODO comment**
   - `// Note: no_primitives check (AES006) requires AST parsing of class attributes. That is delegated to the main ArchitectureRulesEvaluator which has AST access.`
   - Tapi ArchitectureRulesEvaluator tidak ada — delegasi ke entitas yang tidak eksis

### 8.3 Apa yang Perlu Ditambahkan

1. **Checker yang proper dengan konsumsi `no_primitives` config**
   - Gunakan `LayerDefinition.no_primitives` untuk menentukan apakah file perlu dicek
   - Perluas primitive list per bahasa
   - Integrasi dengan `ISourceParserPort::find_primitive_violations()` dari AST scanners

2. **Dukungan Python dan JavaScript**
   - Aktifkan `find_primitive_violations()` di `ast_py_scanner.rs` dan `ast_js_scanner.rs`
   - Panggil melalui `SourceParserOrchestrator` berdasarkan ekstensi file

3. **Unit tests** untuk semua primitive checking logic
   - Test untuk Rust struct fields
   - Test untuk Python class attributes
   - Test untuk JS/TS class properties
   - Test untuk VO exemption
   - Test untuk config `no_primitives: false`

4. **Test fixtures yang lebih komprehensif**
   - Rust: entity dengan primitif (`String`, `i32`, `bool`)
   - Rust: VO dengan primitif (harus exempt — test negative)
   - Python: entity dengan `str`, `int`
   - JS/TS: entity dengan `string`, `number`

### 8.4 Apa yang Perlu Dihapus

1. **`domain_type_checker.rs` — kode mati/orphan**
   - `DomainTypeRuleChecker` tidak pernah dipanggil
   - Hapus file atau wiring agar dipanggil oleh coordinator
   - Atau ganti dengan implementasi yang benar-benar terintegrasi

2. **`python_primitive_checker.rs` — stub kosong**
   - 9 baris, struct kosong, tidak berguna
   - Hapus atau implementasi penuh

### 8.5 Apa yang Perlu Dipertahankan

- **Inline checker sebagai fallback**: selama belum ada implementasi yang proper, inline checker tetap berguna untuk Rust taxonomy meskipun dengan false positive
- **Config YAML structure**: definisi `no_primitives`, scope, dan violation messages sudah benar — hanya perlu wiring
- **`NamingSymbolsConstant::CORE_PRIMITIVE_TYPES`**: constant untuk Python primitives — perlu diperluas dan diintegrasikan

### 8.6 Bukti Empiris Test Project

**AES006 terdeteksi di TEST.md ✅** (line 96 — AES006 ada di 30 unique codes)
Tapi test fixtures yang ada:
- `test-project-python/taxonomy/raw_entity.py` — `str`, `int` ✅
- `test-project-python/taxonomy/raw_error.py` — `int`, `str` ✅
- `test-project-rust/taxonomy/massive_domain_entity.rs` — `String`, `i32` (504 lines, > threshold)

**Yang BELUM ada:**
- Rust test fixture dengan primitif di entity yang jelas (file terpisah)
- JS/TS test fixture dengan primitif
- Rust VO file dengan primitif (harus exempt — untuk test negatif)

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (AST Parsing) | Scanner untuk deteksi tipe per bahasa | Scanner belum diintegrasi untuk AES006 | Integrasi via ISourceParserPort |
| Config YAML `no_primitives` | Flag enable/disable per scope | Flag tidak dikonsumsi → false positive | Wiring config ke checker logic |
| SourceParserOrchestrator | Routing parser by extension | Hanya support Rust di inline checker | Pakai orchestrator untuk multi-language |
| ArchitectureComplianceOrchestrator | Layer definition builder | `no_primitives` sudah di-parse tapi tidak dipakai | Sambungkan ke coordinator |

## 10. Konsep Arsitektur

### 10.1 Alur Data yang Seharusnya
```
lint_arwaky.config.rust.yaml
    ↓ (parsing)
LayerDefinition { no_primitives: true/false }
    ↓ (wiring) ──── harusnya ────→ check_primitive_usage()
                                      ↓
                              SourceParserOrchestrator
                              ├── .rs → ASTRustScanner.find_primitive_violations(fields)
                              ├── .py → ASTPythonScanner.find_primitive_violations(attrs)
                              └── .js → ASTJSScanner.find_primitive_violations(props)
                                      ↓
                              LintResult (AES006 HIGH)
```

### 10.2 Gap Saat Ini
```
Config (no_primitives) ────???───→ Inline Checker (hardcoded, Rust only)
                                        ↓
                               DomainTypeRuleChecker (orphan, Python-focused)
                                        ↓
                               AST Scanners (unused for AES006)
                                        ↓
                               PythonPrimitiveChecker (empty stub)
```

## 11. Appendices
- `src-rust/agent/lint_checking_coordinator.rs:177` — Inline checker (AKTIF)
- `src-rust/capabilities/domain_type_checker.rs:24` — Orphan capability (TIDAK DIPAKAI)
- `src-rust/infrastructure/python_primitive_checker.rs` — Stub kosong
- `src-rust/infrastructure/ast_rust_scanner.rs:368` — Rust scanner (UNUSED untuk AES006)
- `src-rust/infrastructure/ast_py_scanner.rs:394` — Python scanner (UNUSED untuk AES006)
- `src-rust/infrastructure/ast_js_scanner.rs:465` — JS scanner (UNUSED untuk AES006)
- `src-rust/infrastructure/source_parser_adapter.rs:80` — Orchestrator (UNUSED untuk AES006)
- `src-rust/contract/source_parser_port.rs:11` — ISourceParserPort::find_primitive_violations
- `src-rust/taxonomy/naming_symbols_constant.rs:10` — CORE_PRIMITIVE_TYPES
- `src-rust/taxonomy/layer_definition_vo.rs:33` — no_primitives field
- `src-rust/taxonomy/architecture_rule_vo.rs:25` — no_primitives in rule
- `lint_arwaky.config.rust.yaml:184-242` — Config per scope
- `test-project-python/src-python/taxonomy/raw_entity.py` — Fixture
- `test-project-python/src-python/taxonomy/raw_error.py` — Fixture
- `test-project-rust/src-rust/taxonomy/massive_domain_entity.rs` — Fixture
