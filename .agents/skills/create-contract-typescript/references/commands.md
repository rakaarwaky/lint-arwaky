# Quick Commands

```bash
# List contract interfaces
grep -n "^export interface I[A-Za-z0-9_]*Port\|^export interface I[A-Za-z0-9_]*Protocol\|^export interface I[A-Za-z0-9_]*Aggregate" packages/shared/src/**/contract_*.ts

# Check forbidden imports in contract files
grep -n "from.*capabilities_\|from.*infrastructure_\|from.*agent_\|from.*surface_" packages/shared/src/*/contract_*.ts

# Check unregistered contract files
for file in packages/shared/src/<domain>/contract_*.ts; do
  name=$(basename "$file" .ts)
  grep -q "from.*$name" packages/shared/src/<domain>/index.ts \
    || echo "UNREGISTERED: $name"
done
```
