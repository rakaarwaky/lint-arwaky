# 📄 Feature Requirements Document (FRD)
**Feature Name:** File Minimum Size Checker (AES005)  
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
Dokumen ini mendefinisikan aturan **AES005 (FILE_TOO_SHORT)** yang memeriksa batas minimum jumlah baris per file. Aturan diimplementasikan di method `check_line_counts()` dalam `ArchMetricChecker`. AES005 memastikan file tidak terlalu kecil (global default: minimal 10 baris).

### 2.2 Scope
**In-Scope:**
- Pengecekan jumlah baris per file terhadap threshold `min_lines`
- Pengecualian (exception) untuk barrel files (`__init__.py`, `mod.rs`) dan daftar exceptions dari YAML
- Pesan kustom dari konfigurasi YAML (`min_lines_violation_message`)
- Severity HIGH

**Out-of-Scope:**
- Pengecekan maksimum baris (AES004 — FRD terpisah)
- Pengecekan berdasarkan layer dengan threshold berbeda (saat ini global)
- Auto-fixing violations

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES005** | Rule code untuk file kurang dari batas minimum baris |
| **check_line_counts()** | Method utama di `ArchMetricChecker` yang menjalankan aturan |
| **min_lines** | Konfigurasi batas minimum baris per file (default: 10) |
| **count_lines()** | Helper method untuk menghitung jumlah baris dari file |

## 3. Feature Overview
### 3.1 Background & Problem
Sebelum AES005, file bisa dibuat sangat kecil (1-2 baris) tanpa ada peringatan. File yang terlalu kecil mengotori struktur proyek dan menunjukkan bahwa logika seharusnya digabungkan dengan modul terkait. Arsitektur AES membutuhkan komponen yang bermakna dan terukur.

### 3.2 Business Goals
- Mencegah file yang terlalu kecil yang mengotori struktur proyek
- Mendorong penggabungan logika kecil ke modul yang relevan
- Threshold yang dapat dikonfigurasi via YAML

### 3.3 Target Users
- **Developers**: Mendapat feedback otomatis ketika file terlalu kecil
- **Architects**: Mengatur threshold `min_lines` per proyek via YAML

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** Sebagai developer, saya ingin diperingatkan ketika file saya terlalu kecil, sehingga saya bisa menggabungkannya dengan modul terkait.
- **US-002:** Sebagai architect, saya ingin threshold `min_lines` bisa diatur via YAML, sehingga saya bisa menyesuaikan dengan standar proyek.

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: infrastructure/tiny.py

1. Dapatkan basename file
2. Apakah basename == "__init__.py" atau "mod.rs"? → SKIP (barrel)
3. Apakah basename ada di exceptions list? → SKIP
4. count = count_lines(file) → hitung jumlah baris
5. Jika count < min_lines (10) → AES005 HIGH
```

### 4.3 Business Rules
- Severity: HIGH
- Barrel files (`__init__.py`, `mod.rs`) tidak dicek
- Exception list dari YAML: `["main.rs", "lib.rs", "mod.rs", "python_taxonomy_bridge.rs", "js_taxonomy_bridge.rs"]`
- Jika `min_lines` <= 0, aturan dilewati
- Pesan kustom dari YAML didahulukan; jika kosong pakai default

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | 0% untuk file valid |
| NFR-003 | False negative rate | 0% untuk file yang melanggar |

## 6. UI/UX Requirements
```
AES005 HIGH - src-rust/infrastructure/tiny.py
  AES005 FILE_TOO_SHORT: File contains fewer than 10 lines of code.
  WHY? Excessively small files clutter the project structure.
  FIX: Merge this logic into a related module (min: 10).
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File dengan 1 baris (tiny.py) | `check_line_counts()` jalan | AES005 HIGH flagged | ✅ |
| AC-002 | File dengan 10+ baris | `check_line_counts()` jalan | Tidak ada AES005 | ✅ |
| AC-003 | File `__init__.py` atau `mod.rs` | `check_line_counts()` jalan | Skip (barrel) | ✅ |
| AC-004 | File di exception list | `check_line_counts()` jalan | Skip | ✅ |
| AC-005 | File 0 baris (tidak bisa dibaca) | `count_lines()` jalan | Return 0 → **false positive AES005** | ❌ **BUG** |

## 8. Temuan Empiris (Code Audit)

### 8.1 Implementasi Saat Ini
- **Lokasi**: `src-rust/capabilities/architecture_metric_checker.rs:75-128`
- **Bagian AES005**: `architecture_metric_checker.rs:99-112`
- **Status**: **SUDAH TERIMPLEMENTASI PENUH** — bukan stub
- Method `check_line_counts()` dipanggil di `lint_checking_coordinator.rs:98`

### 8.2 Bug yang Ditemukan

1. **`count_lines()` return 0 saat file tidak bisa dibaca** (KRITIS — `architecture_metric_checker.rs:38-42`)
   - **Lokasi**: `architecture_metric_checker.rs:41` — `unwrap_or(0)`
   - Jika `fs::read_to_string` gagal (file tidak ada, permission denied, dll), return 0
   - Karena 0 < `min_lines` (10), ini menyebabkan **false positive AES005**
   - **Dampak**: file yang tidak bisa dibaca akan di-flag sebagai terlalu kecil, padahal mungkin file valid
   - **Fix**: return -1 dan skip check jika return value negatif, atau return `Option<i64>` bukan `i64`

2. **Threshold global tanpa per-layer override** (`lint_arwaky.config.rust.yaml:130`)
   - `min_lines: 10` berlaku untuk SEMUA layer
   - Tidak bisa membedakan threshold untuk taxonomy vs infrastructure vs surfaces
   - **Fix**: dukung `min_lines` per scope di YAML

3. **Tidak ada test unit Rust** untuk `ArchMetricChecker`
   - Tidak ada `#[cfg(test)]` module di `architecture_metric_checker.rs`
   - **Fix**: tambah unit test untuk `count_lines()`, `check_line_counts()`, terutama edge case read error

### 8.3 Apa yang Perlu Ditambahkan
- **Error handling**: jangan return 0 saat file tidak bisa dibaca. Gunakan `Option<i64>`
- **Per-layer threshold**: dukung konfigurasi `min_lines` spesifik per scope/layer
- **Unit tests**: minimal untuk `count_lines()` dengan berbagai kondisi (file normal, file tidak ada, file kosong)
- **Skip check jika count_lines gagal**: tambah pengecekan di `check_line_counts()` sebelum evaluasi threshold

### 8.4 Apa yang Perlu Dipertahankan
- **Logika pengecualian barrel files** ✅ (line 83-86)
- **Pengecualian exceptions list** ✅ (line 93-95)
- **Pesan default yang jelas dan actionable** ✅ (line 104-109)
- **Dukungan pesan kustom dari YAML** ✅ (line 101-102)
- **Skip jika min_lines <= 0** ✅ (line 100)
- **Integrasi dengan coordinator pipeline** ✅ (lint_checking_coordinator.rs:98)

### 8.5 Bukti Empiris Test Project
- **AES005**: `test-project-python/src-python/infrastructure/tiny.py` (1 baris) → flagged ✅
- **AES005**: TEST.md line 60 menyebut Rust AES005 dari `invalid_import_vo`, `removal_types`, `missing_import_analyzer`
  - `test-project-rust/src-rust/taxonomy/bare_entity.rs` (2 baris) — juga akan kena AES005 ✅
  - `test-project-rust/src-rust/taxonomy/bypass_comment_entity.rs` (perlu dicek jumlah barisnya)

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Membaca konten file | File tidak bisa dibaca → false positive | Fix error handling — return Option |
| Config YAML | Threshold dari config | `min_lines: 0` atau negatif | Skip jika <= 0 ✅ |
| Test fixtures | File < 10 baris | Rust fixtures sudah ada (bare_entity.rs = 2 baris) | ✅ sudah cukup |

## 10. Appendices
- `src-rust/capabilities/architecture_metric_checker.rs:75` — `check_line_counts()`
- `src-rust/capabilities/architecture_metric_checker.rs:38` — `count_lines()` (BUG: unwrap_or(0))
- `src-rust/taxonomy/layer_definition_vo.rs` — `min_lines` field
- `src-rust/agent/lint_checking_coordinator.rs:98` — Invocation
- `lint_arwaky.config.rust.yaml:130` — Konfigurasi min_lines
- `test-project-python/src-python/infrastructure/tiny.py` — Fixture (1 baris)
- `test-project-rust/src-rust/taxonomy/bare_entity.rs` — Fixture (2 baris)
