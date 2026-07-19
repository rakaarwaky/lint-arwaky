# Quick Commands

```bash
# Find possible data types in layer files
grep -rn "^interface\|^type \|^enum " packages/*/src/ --exclude-dir=shared

# Check forbidden imports in taxonomy files
grep -n "from.*capabilities_\|from.*infrastructure_\|from.*agent_" packages/shared/src/*/taxonomy_*.ts

# Check possible I/O in taxonomy files
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios\|sqlite3\|pg" packages/shared/src/*/taxonomy_*.ts

# List registered taxonomy modules
grep -n "^export.*from.*taxonomy_" packages/shared/src/*/index.ts
```

## Check Unregistered Taxonomy Files

```bash
for file in packages/shared/src/<domain>/taxonomy_*.ts; do
  name=$(basename "$file" .ts)
  grep -q "from.*$name" packages/shared/src/<domain>/index.ts \
    || echo "UNREGISTERED: $name"
done
```
