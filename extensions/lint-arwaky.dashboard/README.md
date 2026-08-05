# Lint Arwaky Dashboard

Real-time visualization panel for lint-arwaky scan results. View violations by severity, rule group, and file with full AES architecture compliance tracking.

## Features

- **Severity Breakdown** — Errors, warnings, and info counts with visual indicators
- **Rule Group Analysis** — Violations grouped by AES rule categories (naming, import, orphan, quality, role)
- **File Overview** — Per-file violation counts with expandable details
- **Theme Support** — Automatic light/dark mode matching Cate's theme
- **Persistent History** — Scan results stored across sessions

## Usage

1. Run a scan via CLI: `cargo run --bin lint-arwaky-cli -- scan . --json`
2. Store results: paste JSON into the dashboard or use `cate.storage`
3. View real-time updates as you fix violations

## Storage Format

Results are stored in `cate.storage` under the key `scan-results`:

```json
{
  "timestamp": "2026-08-05T10:00:00Z",
  "summary": { "total": 42, "errors": 5, "warnings": 20, "info": 17 },
  "violations": [...]
}
```
