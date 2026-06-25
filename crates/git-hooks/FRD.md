# Feature Requirement Document (FRD) - Git Hooks

## 1. Feature Goal
Tujuan utama modul `git-hooks` adalah mengimplementasikan sistem git hooks untuk pemeriksaan lint pada tahap pre-commit. Modul ini mendeteksi perubahan file melalui diff git dan menjalankan linter hanya pada file yang dimodifikasi, memastikan kode yang masuk ke repository mematuhi aturan AES.

## 2. Requirements & Scope
Modul `git-hooks` bertanggung jawab untuk git hooks berdasarkan spesifikasi berikut:

### Component Specifications
* **GitHooksOrchestrator**: Mengkoordinasikan proses pre-commit hook execution.
* **DiffChecker**: Menganalisis diff git untuk menentukan file yang berubah.
* **HookManager**: Mengelola instalasi dan konfigurasi git hooks.
* **GitHookAdapter**: Adapter untuk interaksi dengan sistem git (subprocess calls).

### Inputs
* Status git stage (staged files).
* Konfigurasi hook yang didefinisikan dalam YAML.

### Outputs
* Hasil lint pada file yang dimodifikasi.
* Exit code non-zero jika ada pelanggaran, mencegah commit.

---

## 3. Success Indicators
Keberhasilan modul `git-hooks` diukur oleh:
* **Hook Installation**: Hooks terpasang dengan benar di semua jenis sistem (Linux, macOS, Windows).
* **Diff Accuracy**: Hanya file yang benar-benar diubah yang dipindai.
* **Commit Blocking**: Commit yang melanggar aturan AES berhasil diblokir.
* **Self-Audit Conformity**: Modul ini sendiri mematuhi aturan AES dalam kode sumbernya.