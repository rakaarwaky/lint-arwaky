# Quick Commands

```bash
# List structs in infrastructure files
rg -n "^\s*pub struct" crates/<crate>/src/infrastructure_*.rs

# List port trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Port\s+for" crates/<crate>/src/infrastructure_*.rs

# Check possible business logic keywords
rg "is_orphan|analyze|validate|calculate|compute|business" crates/<crate>/src/infrastructure_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(capabilities_|agent_)" crates/<crate>/src/infrastructure_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/infrastructure_*.rs
```

## Check Wrong Block Order

```bash
for file in crates/<crate>/src/infrastructure_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }
    /^impl (Default|Clone|Debug|Display)/ { if (!std) std = FNR }
    /^impl I[A-Z].*Port/ { if (!proto) proto = FNR }
    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before port (line " proto ")"
      }
    }
  ' "$file"
done
```
