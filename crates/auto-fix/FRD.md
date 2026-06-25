# Feature Requirement Document (FRD) - Auto Fix

## 1. Feature Goal
Tujuan utama modul `auto-fix` adalah menyediakan mekanisme perbaikan otomatis untuk pelanggaran aturan AES yang dapat diperbaiki secara mekanis. Modul ini mengambil hasil linting dan menerapkan koreksi otomatis pada file sumber yang bersangkutan, mengurangi beban kerja manual developer dalam memperbaiki kesalahan yang dapat dideteksi dan diperbaiki secara deterministik.

## 2. Requirements & Scope
Modul `auto-fix` bertanggung jawab untuk menerapkan koreksi otomatis berdasarkan spesifikasi berikut:

### Rules Specifications
* **AES Fix: Perbaikan Import yang Tidak Digunakan**
  * **Requirement**: Menghapus baris import yang tidak direferensikan dalam file secara otomatis.
  * **Scope**: Rust, Python, JavaScript, dan TypeScript.

* **AES Fix: Perbaikan Penamaan File**
  * **Requirement**: Mengganti nama file yang melanggar konvensi snake_case menjadi format yang benar secara otomatis.
  * **Scope**: Semua bahasa yang didukung.

* **AES Fix: Perbaikan Bypass Warning**
  * **Requirement**: Menambahkan atau memperbaiki komentar bypass (seperti `noqa`, `type: ignore`) yang tidak valid menjadi format yang benar, atau menghapusnya bersama perbaikan kode.
  * **Scope**: Python (ruff, mypy) dan JavaScript/TypeScript (eslint).

* **AES Fix: Perbaikan Format Kode**
  * **Requirement**: Mengaplikasikan format otomatis menggunakan rustfmt, prettier, atau formatter bawaan.
  * **Scope**: Rust, JavaScript/TypeScript.

### Inputs
* Daftar hasil linting (`Vec<LintResult>`) yang mengandung pelanggaran yang dapat diperbaiki.
* Konfigurasi proyek (`ArchitectureConfig`) untuk menentukan aturan yang berlaku.

### Outputs
* File sumber yang telah diperbaiki.
* Laporan perubahan yang berisi jumlah perbaikan yang diterapkan per kategori.

---

## 3. Success Indicators
Keberhasilan modul `auto-fix` diukur oleh:
* **Akurasi Perbaikan**: Perbaikan yang diterapkan tidak merusak fungsionalitas kode.
* **Coverage Rules**: Persentase pelanggaran yang dapat diperbaiki secara otomatis mencapai target.
* **Idempotensi**: Menjalankan auto-fix berulang kali pada file yang sama tidak menimbulkan perubahan tambahan.
* **Self-Audit**: Modul ini sendiri harus mematuhi aturan AES dan lulus pemeriksaan linting.