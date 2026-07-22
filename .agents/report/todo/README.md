# Review Reports (Todo)

Setiap fitur/crate/module/package yang akan direview mendapat satu subfolder di sini.

## Struktur

```
.todo/
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
2. Copy template dari `.agents/report/todo/TEMPLATE.md` sebagai isi laporan
3. Isi metadata dan tulis temuan sesuai role yang melakukan review

## Severity Convention

- 🔴 **CRITICAL** — Breach of AES layering, data leak, security risk
- 🟡 **WARNING** — Convention deviation, maintainability concern
- 🟢 **INFO** — Suggestion, nice-to-have improvement
