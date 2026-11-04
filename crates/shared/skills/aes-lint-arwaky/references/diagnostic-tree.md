# Diagnostic tree

Scan failed or findings non-zero?

```text
├─ exit 2 → config/parse → lint_arwaky.config.yaml / path → fix config, re-run
├─ exit 1, findings present
│  ├─ CRITICAL 🔴 AES201 / AES205 → structural; route via references/lint-routing.md (aes-contract)
│  ├─ HIGH 🟡 AES101–102, AES202, AES301–303, AES401–403, AES406, AES505–506 → fix / aes-{layer}
│  ├─ MEDIUM/LOW 🟢 AES203–204, AES305, AES404–405, AES501–504 → fix (AES101/203 auto-fix) or aes-{layer} (manual)
│  └─ After each AES101 rename → barrel update (language HOW-TO) or orphan reappears
└─ exit 0 but code unhealthy → language verify pipeline
```
