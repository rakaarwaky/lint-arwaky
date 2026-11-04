# Anti-patterns

| Mistake | Correct approach |
| ------- | ---------------- |
| Skipping pre-flight; trusting findings on broken code | Build/import first — violations on red builds are noise. |
| Auto-fixing AES201/205 with `fix` | Only AES101/203/304 auto-fix; AES201/205 manual via `aes-contract`. |
| AES101 rename without barrel/`mod.rs`/`index.ts` update | Re-export immediately — otherwise AES501–506 orphan. |
| Declaring done at "mostly clean" | Done = re-scan **0** + language verify; exit code is the gate. |
| Parsing stdout instead of exit code for CI | Gate on `0`/`1`/`2`; stdout is not the contract. |
| Re-guessing fix rows from memory | Consult `references/lint-routing.md` every time. |
| Treating `tests/`/`benches/` as AES layers | They follow `aes-testing-suite` naming, not AES101–102. |
