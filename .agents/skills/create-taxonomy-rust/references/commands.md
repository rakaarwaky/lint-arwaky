# Quick Commands

```bash
# Find possible dataclasses in layer files
rg -n "^\s*pub struct|^\s*pub enum" crates/<crate>/src --glob '!**/shared/**'

# Check forbidden imports in taxonomy files
rg -n "^\s*use\s+.*(capabilities_|agent_|surface_)" crates/shared/src/**/taxonomy_*.rs

# Check possible I/O in taxonomy files
rg -n "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/shared/src/**/taxonomy_*.rs

# List registered taxonomy modules
rg -n "^pub mod taxonomy_" crates/shared/src/*/mod.rs

# Find magic constants in layer files
rg "[0-9]+\.[0-9]+" crates/<crate>/src --glob '!**/shared/**'
```

## Check Unregistered Taxonomy Files

```bash
for file in crates/shared/src/<domain>/taxonomy_*.rs; do
  basename=$(basename "$file" .rs)
  rg -q "^pub mod $basename;" crates/shared/src/<domain>/mod.rs \
    || echo "UNREGISTERED: $basename"
done
```
