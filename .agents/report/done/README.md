# Done — Completed Review Reports

Setiap fitur/crate/module/package yang sudah direview mendapat satu subfolder di sini.

## Struktur

```
.done/
├── README.md              ← file ini
└── <nama-fitur>/          ← folder = nama fitur/crate/package yang direview
    ├── BACKEND_DEVELOPER.md     ← role: Backend Developer
    ├── BUSINESS_ANALYST.md      ← role: Business Analyst
    ├── QUALITY_ANALYSIS.md      ← role: QA/Analyst
    └── ...                      ← file lain sesuai role yang review
```

## Konvensi Penamaan

| Komponen   | Aturan                                | Contoh                                                               |
| ---------- | ------------------------------------- | -------------------------------------------------------------------- |
| **Folder** | Nama fitur / crate / module / package | `cli-commands`, `import-rules`, `code-analysis`                      |
| **File**   | Nama role penuh (dari prompt files)   | `BACKEND_DEVELOPER.md`, `BUSINESS_ANALYST.md`, `QUALITY_ANALYSIS.md` |

## Cara Pakai

1. Buat folder baru: `<nama-fitur>/<ROLE>.md`
2. Isi laporan review sesuai role yang melakukan review
3. Gunakan template dari `.agents/report/todo/TEMPLATE.md` sebagai acuan

## Daftar Fitur yang Sudah Direview

| Folder (Fitur)    | Reports                                               |
| ----------------- | ----------------------------------------------------- |
| `cli-commands`    | BACKEND_DEVELOPER, BUSINESS_ANALYST, QUALITY_ANALYSIS |
| `code-analysis`   | BACKEND_DEVELOPER, BUSINESS_ANALYST, QUALITY_ANALYSIS |
| `config-sytem`    | BACKEND_DEVELOPER, BUSINESS_ANALYST, QUALITY_ANALYSIS |
| `import-rules`    | BACKEND_DEVELOPER, QUALITY_ANALYSIS                   |
| `orphan-detector` | BACKEND_DEVELOPER, BUSINESS_ANALYST, QUALITY_ANALYSIS |
| `role-rules`      | BACKEND_DEVELOPER, BUSINESS_ANALYST, QUALITY_ANALYSIS |
