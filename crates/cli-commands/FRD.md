# Feature Requirement Document (FRD) - CLI Commands

## 1. Feature Goal
Tujuan utama modul `cli-commands` adalah menyediakan antarmuka command-line interface (CLI) yang menyatu bagi pengguna untuk menjalankan seluruh pipeline linting. Modul ini mengimplementasikan surfaces untuk perintah-perintah utama seperti `check`, `scan`, `fix`, `git`, `config`, `setup`, `tui`, dan `watch`.

## 2. Requirements & Scope
Modul `cli-commands` bertanggung jawab untuk menyediakan command surfaces berdasarkan spesifikasi berikut:

### Command Specifications
* **check**: Memeriksa satu file atau direktori terhadap aturan AES.
* **scan**: Memindai seluruh workspace dan menghasilkan laporan lengkap.
* **fix**: Menerapkan perbaikan otomatis pada file yang melanggar aturan.
* **git**: Mengimplementasikan git hooks dan pemeriksaan diff untuk pre-commit.
* **config**: Mengelola konfigurasi lint_arwaky (inisialisasi, validasi, pembaruan).
* **setup**: Menyiapkan proyek baru dengan struktur direktori AES.
* **tui**: Memulai antarmuka terminal pengguna (TUI) untuk linting interaktif.
* **watch**: Memantau perubahan file secara real-time dan menjalankan scan otomatis.

### Inputs
* Argumen baris perintah (`&[String]`) yang diparse oleh clap.
* Konfigurasi proyek dari berbagai sumber (YAML, environment variables).

### Outputs
* Laporan linter dalam format terminal (colored output via console crate).
* Exit code yang sesuai (0 untuk sukses, non-zero untuk pelanggaran).

---

## 3. Success Indicators
Keberhasilan modul `cli-commands` diukur oleh:
* **UX Consistency**: Semua perintah mengikuti pola input/output yang konsisten.
* **Performance**: CLI responsif dengan output yang di-buffer untuk workspace besar.
* **Help Documentation**: Setiap perintah memiliki dokumentasi `--help` yang jelas.
* **Self-Audit Conformity**: Modul ini sendiri lulus pemeriksaan aturan AES.