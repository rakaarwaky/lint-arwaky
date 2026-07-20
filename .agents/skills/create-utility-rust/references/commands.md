# Quick Commands (Utility)

## Verify Structure

```bash
# Check for forbidden patterns (struct, impl, &self)
rg "pub struct|impl\s+.*\{|fn\s+.*&self" crates/shared/src/<domain>/utility_*.rs

# List all utility functions
rg "^pub fn" crates/shared/src/<domain>/utility_*.rs

# Check imports in utilities (should only use shared::taxonomy)
rg "^\s*use\s+" crates/shared/src/<domain>/utility_*.rs
```

## Verify Purity

```bash
# Check for side effects (rand, SystemTime, static mut, lazy_static)
rg "(rand|SystemTime|static mut|lazy_static)" crates/shared/src/<domain>/utility_*.rs

# Check for business rule knowledge (architecture layer names)
rg "(agent_|capabilities_|contract_|surface_)" crates/shared/src/<domain>/utility_*.rs
```

## Verify Compilation

```bash
cargo check -p shared-lint-arwaky
```

## Verify Reusability

```bash
# Count usages of a utility function across the workspace
rg -n "fn\s+<utility_name>" crates/ --glob "!**/utility_*.rs"

# If count == 0, it may be unused (but could be new — verify before removing)
```
