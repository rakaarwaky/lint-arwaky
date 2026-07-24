# Quick Commands (Utility — Python)

## Verify Structure

```bash
# Check for forbidden patterns (class, self)
grep -rn "^class \|def.*self" modules/shared/src/<domain>/utility_*.py

# List all utility functions
grep -rn "^def " modules/shared/src/<domain>/utility_*.py

# Check imports in utilities (should only use taxonomy)
grep -rn "^from\|^import" modules/shared/src/<domain>/utility_*.py
```

## Verify Purity

```bash
# Check for side effects (random, time.sleep, open for write)
grep -rn "random\|time\.sleep\|open.*w\|os\.system" modules/shared/src/<domain>/utility_*.py

# Check for business rule knowledge (architecture layer names)
grep -rn "agent_\|capabilities_\|contract_\|surface_" modules/shared/src/<domain>/utility_*.py
```

## Verify Compilation

```bash
python -c "import modules.shared.src.<domain>.utility_<name>"
```

## Verify Reusability

```bash
# Count usages of a utility function across the workspace
grep -rn "from.*utility_<name> import\|utility_<name>\." modules/*/src/ --exclude-dir=shared

# If count == 0, it may be unused (but could be new — verify before removing)
```
