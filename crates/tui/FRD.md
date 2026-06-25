# Feature Requirement Document (FRD) - TUI

## 1. Feature Goal
Tujuan utama modul `tui` adalah menyediakan antarmuka terminal pengguna (Text User Interface) berbasis Ratatui yang interaktif untuk menjelajahi hasil linting, mempratinjau file, dan menerapkan perbaikan. Interface 3-panel memungkinkan navigasi intuitif pada struktur proyek.

## 2. Requirements & Scope
Modul `tui` bertanggung jawab untuk antarmuka TUI berdasarkan spesifikasi berikut:

### Component Specifications
* **TuiOrchestrator**: Mengkoordinasikan state management dan user interactions.
* **FileBrowser**: Menampilkan daftar file dalam hierarki tree.
* **LayerDetector**: Mendeteksi layer AES dari file yang dipilih.
* **LintExecutor**: Menjalankan linting pada demand user.
* **ActionHandler**: Menangani aksi perbaikan dan navigasi.

### View Components
* **FileListScreen**: Panel kiri - daftar file dan folder.
* **PreviewScreen**: Panel tengah - preview isi file dengan highlighting.
* **TreeScreen**: Panel kanan - hierarki tree hasil linting.
* **HelpScreen**: Menampilkan shortcut dan bantuan.
* **ShortcutComponent**: Komponen shortcut keyboard.
* **StatusComponent**: Komponen status bar.

### Inputs
* Key press dari pengguna (navigation, action commands).
* File path yang dipilih untuk preview/lint.

### Outputs
* Tampilan terminal yang diperbarui secara real-time.
* Hasil lint atau aksi yang dipilih user.

---

## 3. Success Indicators
Keberhasilan modul `tui` diukur oleh:
* **Responsiveness**: UI responsif dengan update cepat (< 50ms).
* **Navigation Smoothness**: Navigasi file dan folder lancar tanpa lag.
* **Feature Completeness**: Semua perintah lint dapat diakses via TUI.
* **Self-Audit Conformity**: Modul ini sendiri lulus pemeriksaan aturan AES.