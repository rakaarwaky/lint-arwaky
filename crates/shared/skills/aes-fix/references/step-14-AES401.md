# Step 14: Import: Primitive-Type Closure Annotation

## What the rule checks

AES401 (rule code `AES401`, checker `capabilities_taxonomy_role_auditor` in the `role-rules` crate) flags explicit `char` primitive type annotations on closure parameters — e.g. `|c: char|` — in taxonomy-layer code.

## Common violation causes

- Closure parameter typed explicitly as `char`: `|c: char| ...`
- `trim_matches(|c: char| ...)` with explicit type annotation
- `filter(char::is_ascii_digit)` used where `Pattern`/`Fn(char)` is expected — `char::is_ascii_digit` has signature `fn(&char) -> bool`, not `fn(char) -> bool`, so it fails the `Pattern` bound.

## How to fix (concrete steps)

1. Identify the violated rule code (AES401) from the scan report.
2. Replace explicit `char` type annotations with `char` methods called on the character directly:
   - `|c: char| c.is_ascii_digit()` → `.chars().take_while(|c| !c.is_ascii_digit() && ...).collect::<String>()`
   - `|c: char| *c != '('` → `.chars().take_while(|c| *c != '(').collect::<String>()`
   - `trim_matches(|c: char| c == '{')` → `.trim_matches('{')`
   - `.filter(char::is_ascii_digit)` → `.filter(|c| c.is_ascii_digit()).collect::<String>()`
3. Re-run `lint-arwaky-cli scan .` and confirm AES401 no longer fires.

## Verify

```bash
lint-arwaky-cli scan . --filter AES401
# Confirm no AES401 finding remains.
```
