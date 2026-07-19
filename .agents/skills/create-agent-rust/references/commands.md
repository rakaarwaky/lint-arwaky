# Quick Commands

```bash
# List structs in agent files
rg -n "^\s*pub struct" crates/<crate>/src/agent_*.rs

# List aggregate trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Aggregate\s+for" crates/<crate>/src/agent_*.rs

# Check possible computation/transformation patterns
rg "\.sum\(\)|\.len\(\)|\.map\(|\.fold\(|\.collect\(" crates/<crate>/src/agent_*.rs

# Check possible I/O in agents
rg "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/<crate>/src/agent_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(capabilities_|infrastructure_|surface_)" crates/<crate>/src/agent_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/agent_*.rs

# Find possible magic numbers
rg "[0-9]+\.[0-9]+" crates/<crate>/src/agent_*.rs

# Check object safety issues
cargo check -p <crate-name> 2>&1 | rg "cannot be made into an object"
```

## Check Wrong Block Order

```bash
for file in crates/<crate>/src/agent_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }
    /^impl (Default|Clone|Debug|Display)/ { if (!std) std = FNR }
    /^impl I[A-Z].*Aggregate/ { if (!proto) proto = FNR }
    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before aggregate (line " proto ")"
      }
    }
  ' "$file"
done
```
