# Quick Commands

These commands are rough heuristic checks. Final validation should use `cargo check`, clippy, or AST-based tooling.

```bash
# Check possible I/O in capabilities (AES404)
rg "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/<crate>/src/capabilities_*.rs

# Check forbidden imports
rg "^\s*use\s+.*agent_" crates/<crate>/src/capabilities_*.rs

# List structs in capabilities files
rg -n "^\s*pub struct" crates/<crate>/src/capabilities_*.rs

# List protocol trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Protocol\s+for" crates/<crate>/src/capabilities_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/capabilities_*.rs

# Find possible magic numbers
rg "[0-9]+\.[0-9]+" crates/<crate>/src/capabilities_*.rs

# Check object safety issues
cargo check -p <crate-name> 2>&1 | rg "cannot be made into an object"
```

## Check Wrong Block Order

```bash
for file in crates/<crate>/src/capabilities_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }
    /^impl (Default|Clone|Debug|Display)/ { if (!std) std = FNR }
    /^impl I[A-Z].*Protocol/ { if (!proto) proto = FNR }
    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before protocol (line " proto ")"
      }
    }
  ' "$file"
done
```
