# Quick Commands (Utility — TypeScript)

## Verify Structure

```bash
# Check for forbidden patterns (class, this)
grep -rn "^class \|this\." packages/shared/src/<domain>/utility_*.ts

# List all utility functions
grep -rn "^export function" packages/shared/src/<domain>/utility_*.ts

# Check imports in utilities (should only use taxonomy)
grep -rn "^import" packages/shared/src/<domain>/utility_*.ts
```

## Verify Purity

```bash
# Check for side effects (Math.random, Date.now, fs.write, fetch)
grep -rn "Math\.random\|Date\.now\|fs\.write\|fetch\|XMLHttpRequest" packages/shared/src/<domain>/utility_*.ts

# Check for business rule knowledge (architecture layer names)
grep -rn "agent_\|capabilities_\|contract_\|surface_" packages/shared/src/<domain>/utility_*.ts
```

## Verify Compilation

```bash
npx tsc --noEmit
```

## Verify Reusability

```bash
# Count usages of a utility function across the workspace
grep -rn "from.*utility_<name>\|utility_<name>\." packages/*/src/ --exclude-dir=shared

# If count == 0, it may be unused (but could be new — verify before removing)
```
