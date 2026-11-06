# Step 5: Import Graph: Mandatory Layer Import

## What the rule checks

AES202 (rule code `AES202`, checker `capabilities_import_mandatory_checker` in the `import-rules` crate) enforces mandatory layer imports for contract & aggregate enforcement.

- A capability must import its contract (`contract(*_protocol)`).
- An agent must import its aggregate.

## Common violation causes

- A capability file does not import the protocol it implements.
- An agent file does not import its aggregate.
- Required type/import missing after a rename or move.

## How to fix (concrete steps)

1. Identify the violated rule code (AES202) and the missing import from the scan report.
2. Add the required import so the dependency points in the allowed direction.
3. For a capability: import `contract(*_protocol)` it implements.
4. For an agent: import its aggregate.
5. Re-run `lint-arwaky-cli scan .` and confirm AES202 no longer fires on that file.

## Verify

```bash
lint-arwaky-cli scan . --filter AES202
# Confirm no AES202 finding remains.
```
