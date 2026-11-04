# Workflow

1. **Pre-flight** — the codebase must build/parse in its own language (language HOW-TO). Violations on broken code are unreliable.
2. **Scan** — `lint-arwaky-cli scan <target-path> --format json` (shared flags: [references/lint-commands.md](lint-commands.md)). Triage CRITICAL → HIGH → MEDIUM/LOW.
3. **Diagnose** — per finding: AES code, layer prefix, auto-fixable?, root cause. Full table: [references/lint-routing.md](lint-routing.md).
4. **Fix** — auto: `lint-arwaky-cli fix <path>` (`--dry-run`, optional `--filter`). Manual: route via [references/lint-routing.md](lint-routing.md) to the owning `aes-*` skill.
5. **Verify** — re-scan to 0, then language verify pipeline.
6. **Commit** — only when asked: `fix: resolve <N> AES violations (<rules>)`.
