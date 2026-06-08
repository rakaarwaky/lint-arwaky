# 📄 Feature Requirements Document (FRD)
**Feature Name:** File Size Limit Checker (AES004)  
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
Dokumen ini mendefinisikan aturan **AES004 (FILE_TOO_LARGE)** yang memeriksa batas maksimum jumlah baris per file. Aturan diimplementasikan di method `check_line_counts()` dalam `ArchMetricChecker`. AES004 memastikan file tidak melebihi batas maksimum baris (global default: 700).

### 2.2 Scope
**In-Scope:**
- Pengecekan jumlah baris per file terhadap threshold `max_lines`
- Pengecualian (exception) untuk barrel files (`__init__.py`, `mod.rs`) dan daftar exceptions dari YAML
- Pesan kustom dari konfigurasi YAML (`max_lines_violation_message`)
- Severity HIGH

**Out-of-Scope:**
- Pengecekan minimum baris (AES005 — FRD terpisah)
- Pengecekan berdasarkan layer dengan threshold berbeda (saat ini global)
- Auto-fixing violations

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES004** | Rule code untuk file melebihi batas maksimum baris |
| **check_line_counts()** | Method utama di `ArchMetricChecker` yang menjalankan aturan |
| **max_lines** | Konfigurasi batas maksimum baris per file (default: 700) |
| **count_lines()** | Helper method untuk menghitung jumlah baris dari file |

## 3. Feature Overview
### 3.1 Background & Problem
Sebelum AES004, tidak ada batasan ukuran file dalam proyek. File bisa menjadi sangat besar (ribuan baris) yang melanggar Single Responsibility Principle dan sulit di-maintain. Arsitektur AES membutuhkan file yang fokus dan terukur.

### 3.2 Business Goals
- Mencegah file yang terlalu besar yang sulit di-maintain dan di-test
- Memberikan pesan violation yang jelas dan actionable
- Threshold yang dapat dikonfigurasi via YAML

### 3.3 Target Users
- **Developers**: Mendapat feedback otomatis ketika file terlalu besar
- **Architects**: Mengatur threshold `max_lines` per proyek via YAML

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** Sebagai developer, saya ingin diperingatkan ketika file saya melebihi batas baris maksimum, sehingga saya bisa memecahnya menjadi modul yang lebih kecil.
- **US-002:** Sebagai architect, saya ingin threshold `max_lines` bisa diatur via YAML, sehingga saya bisa menyesuaikan dengan standar proyek.

### 4.2 Use Cases & Workflow
**Detection Pipeline:**
```
File: capabilities/my_checker.rs

1. Dapatkan basename file
2. Apakah basename == "__init__.py" atau "mod.rs"? → SKIP (barrel)
3. Apakah basename ada di exceptions list? → SKIP
4. count = count_lines(file) → hitung jumlah baris
5. Jika count > max_lines (700) → AES004 HIGH
```

### 4.3 Business Rules
- Severity: HIGH
- Barrel files (`__init__.py`, `mod.rs`) tidak dicek
- Exception list dari YAML: `["main.rs", "lib.rs", "mod.rs", "python_taxonomy_bridge.rs", "js_taxonomy_bridge.rs"]`
- Jika `max_lines` <= 0, aturan dilewati
- Pesan kustom dari YAML didahulukan; jika kosong pakai default

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |
| NFR-002 | False positive rate | 0% untuk file valid |
| NFR-003 | False negative rate | 0% untuk file yang melanggar |

## 6. UI/UX Requirements
```
AES004 HIGH - src-rust/capabilities/massive_file.rs
  AES004 FILE_TOO_LARGE: File exceeds the 700-line limit.
  WHY? Large files violate the Single Responsibility Principle.
  FIX: Split the module into smaller, more focused files (max: 700).
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File dengan 504 baris (massive_domain_entity.rs) | `check_line_counts()` jalan | Tidak ada AES004 (504 < 700) | ✅ |
| AC-002 | File > 700 baris (belum ada fixture) | `check_line_counts()` jalan | AES004 HIGH flagged | ✅ (logic benar) |
| AC-003 | File `__init__.py` atau `mod.rs` | `check_line_counts()` jalan | Skip (barrel) | ✅ |
| AC-004 | File di exception list | `check_line_counts()` jalan | Skip | ✅ |
| AC-005 | File 0 baris (tidak bisa dibaca) | `count_lines()` jalan | Return 0 — tidak overflow | ⚠️ OK untuk AES004 |

## 8. Temuan Empiris (Code Audit)

### 8.1 Implementasi Saat Ini
- **Lokasi**: `src-rust/capabilities/architecture_metric_checker.rs:75-128`
- **Bagian AES004**: `architecture_metric_checker.rs:114-127`
- **Status**: **SUDAH TERIMPLEMENTASI PENUH** — bukan stub
- Method `check_line_counts()` dipanggil di `lint_checking_coordinator.rs:98`

### 8.2 Bug yang Ditemukan
1. **Threshold global tanpa per-layer override** (`lint_arwaky.config.rust.yaml:131`)
   - `max_lines: 700` berlaku untuk SEMUA layer
   - Tidak bisa membedakan threshold untuk taxonomy vs capabilities vs surfaces
   - **Fix**: dukung `max_lines` per scope di YAML

2. **Tidak ada test unit Rust** untuk `ArchMetricChecker`
   - Tidak ada `#[cfg(test)]` module di `architecture_metric_checker.rs`
   - Test hanya mengandalkan test-project fixtures
   - **Fix**: tambah unit test untuk `count_lines()`, `check_line_counts()`

### 8.3 Apa yang Perlu Ditambahkan
- **Per-layer threshold**: dukung konfigurasi `max_lines` spesifik per scope/layer
- **Unit tests**: minimal untuk `count_lines()` dan `check_line_counts()`
- **Test fixture untuk Rust AES004**: TEST.md menyebut `extremely_large_vo` (line 59) tapi file itu **tidak ada** di test-project-rust. Buat file dengan > 700 baris untuk Rust.

### 8.4 Apa yang Perlu Dipertahankan
- **Logika pengecualian barrel files** ✅ (line 83-86)
- **Pesan default yang jelas dan actionable** ✅ (line 119-124)
- **Dukungan pesan kustom dari YAML** ✅ (line 116-117)
- **Integrasi dengan coordinator pipeline** ✅ (lint_checking_coordinator.rs:98)

### 8.5 Bukti Empiris Test Project
- `test-project-rust/src-rust/taxonomy/massive_domain_entity.rs` (504 baris) → tidak overflow karena < 700 ✅
- `test-project-python/src-python/taxonomy/large_domain_entity.py` — perlu dicek jumlah barisnya
- **AES004 di TEST.md**: disebut `extremely_large_vo` tapi **file tidak ditemukan** → fixture hilang ❌

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Membaca konten file | File tidak bisa dibaca | count_lines return 0 (OK untuk AES004) |
| Config YAML | Threshold dari config | Threshold tidak wajar (0 atau negatif) | Skip jika <= 0 ✅ |
| Test fixtures | File test > 700 baris | `extremely_large_vo` tidak ada | Buat fixture baru |

## 10. Appendices
- `src-rust/capabilities/architecture_metric_checker.rs:75` — `check_line_counts()`
- `src-rust/capabilities/architecture_metric_checker.rs:38` — `count_lines()`
- `src-rust/taxonomy/layer_definition_vo.rs` — `max_lines` field
- `src-rust/agent/lint_checking_coordinator.rs:98` — Invocation
- `lint_arwaky.config.rust.yaml:131` — Konfigurasi max_lines
- `test-project-rust/src-rust/taxonomy/massive_domain_entity.rs` — Fixture (504 baris)
