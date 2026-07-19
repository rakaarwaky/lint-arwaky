# Quick Commands

```bash
# List contract traits
rg -n "^\s*pub trait" crates/shared/src/**/contract_*.rs

# Check forbidden imports in contract files
rg -n "^\s*use\s+.*(capabilities_|infrastructure_|agent_|surface_)" crates/shared/src/**/contract_*.rs

# Check possible raw primitive signatures
rg -n "fn .*\b(String|Vec<String>|Option<String>|usize|u32|i32|u64|i64|f32|f64)\b" crates/shared/src/**/contract_*.rs

# Check async fn without async_trait nearby
rg -n "^\s*async fn" crates/shared/src/**/contract_*.rs

# Check object safety issues
cargo check -p shared 2>&1 | rg "cannot be made into an object"
```

## Check Unregistered Contract Files

```bash
for file in crates/shared/src/<domain>/contract_*.rs; do
  basename=$(basename "$file" .rs)
  rg -q "^pub mod $basename;" crates/shared/src/<domain>/mod.rs \
    || echo "UNREGISTERED: $basename"
done
```
