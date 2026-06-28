---
name: lint-scan
description: Scan project with lint-arwaky CLI and filter results by AES violation code. Use to verify specific rule compliance after fixes.
---

# Lint Scan

Scan codebase with lint-arwaky and filter by specific AES codes.

## Full Scan

```bash
cargo run --bin lint-arwaky-cli -- scan <path> 2>&1
```

## Scan + Count by AES Code

```bash
cargo run --bin lint-arwaky-cli -- scan <path> 2>&1 | grep -oP 'AES[0-9]+' | sort | uniq -c | sort -rn
```

## Filter Specific AES Code

```bash
# AES304 bypass violations
cargo run --bin lint-arwaky-cli -- scan crates/shared/ 2>&1 | grep "AES304"

# AES305 duplication violations
cargo run --bin lint-arwaky-cli -- scan crates/shared/ 2>&1 | grep "AES305"

# AES203 import violations
cargo run --bin lint-arwaky-cli -- scan crates/shared --filter AES203 2>&1 | grep "Import '"
```

## Scan Specific Crate

```bash
cargo run --bin lint-arwaky-cli -- scan crates/code-analysis/ 2>&1 | grep -E "AES[0-9]+" | head -20
```

## Verify Zero Violations

```bash
VIOLATIONS=$(cargo run --bin lint-arwaky-cli -- scan <path> 2>&1 | grep -c "AES")
if [ "$VIOLATIONS" -eq 0 ]; then echo "PASS"; else echo "FAIL: $VIOLATIONS violations"; fi
```

## Test-Workspace Coverage Check

```bash
OUTPUT=$(cargo run --bin lint-arwaky-cli -- scan test-workspaces/ 2>&1)
UNIQUE_AES=$(echo "$OUTPUT" | grep -oP 'Total Unique AES Codes:\s*\K[0-9]+' || echo "0")
echo "Unique AES codes: $UNIQUE_AES"
if [ "$UNIQUE_AES" -lt 24 ]; then echo "FAIL: Only $UNIQUE_AES (min 24)"; exit 1; fi
```

## Common AES Codes

| Code       | Meaning              | Crate           |
| ---------- | -------------------- | --------------- |
| AES301-302 | File/line limits     | code-analysis   |
| AES303     | Mandatory definition | code-analysis   |
| AES304     | Bypass detection     | code-analysis   |
| AES305     | Code duplication     | code-analysis   |
| AES201-205 | Import rules         | import-rules    |
| AES101-106 | Naming rules         | naming-rules    |
| AES501-506 | Orphan detection     | orphan-detector |
