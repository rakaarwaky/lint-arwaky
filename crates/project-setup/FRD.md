# Feature Requirement Document (FRD) - Project Setup

## 1. Feature Goal
Tujuan utama modul `project-setup` adalah menyediakan fasilitas scaffolding dan doctor checks untuk proyek baru. Modul ini menginisialisasi proyek dengan layout direktori yang sesuai AES, menyiapkan konfigurasi MCP, dan menyediakan template CI/CD untuk integrasi lint_arwaky.

## 2. Requirements & Scope
Modul `project-setup` bertanggung jawab untuk setup berdasarkan spesifikasi berikut:

### Component Specifications
* **SetupManagementOrchestrator**: Mengkoordinasikan proses inisialisasi proyek.
* **SetupManagementProcessor**: Memproses template dan file-file yang perlu dibuat.
* **SetupInstallerAdapter**: Adapter untuk operasi filesystem dan template installation.

### Features
* **init**: Membuat struktur direktori AES (taxonomy, contract, capabilities, infrastructure, agent, surface, root).
* **doctor**: Memeriksa apakah proyek sudah siap untuk lint_arwaky.
* **mcp-config**: Membuat konfigurasi MCP untuk integrasi AI.
* **ci-templates**: Menyediakan template GitHub Actions atau scripts CI.

### Inputs
* Path target untuk inisialisasi.
* Parameter setup (bahasa, framework, template).

### Outputs
* File-file template yang telah dibuat.
* Status inisialisasi dan pesan error bila diperlukan.

---

## 3. Success Indicators
Keberhasilan modul `project-setup` diuki oleh:
* **Structure Correctness**: Direktori dan file terbuat sesuai pola AES.
* **Template Accuracy**: Template yang dibuat siap pakai dan sesuai standar.
* **CI Integration**: Workflow CI dapat langsung dipakai tanpa modifikasi.
* **Self-Audit Conformity**: Modul ini sendiri lulus pemeriksaan aturan AES.