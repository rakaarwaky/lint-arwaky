# Verification

## Machine checks

Primary gate:

```bash
lint-arwaky-cli docs .                 # invariant audit of every document (strict; every finding gates)
cargo doc --no-deps                    # Rust rustdoc builds; no public /// missing
cargo test --doc                       # Rust doctests compile
```

## Fallback gate only — language doc build

- Python: `python -c "import <package>"`
- TypeScript: `npx tsc --noEmit`

## Human checks

- No "how" narration in doc comments; the body shows how.
- Summary line first, before any `#` section (Rust) or body text (Python).
- No status words (`implemented`, `shipped`, `N% done`).
