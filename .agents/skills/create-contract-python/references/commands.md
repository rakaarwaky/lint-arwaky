# Quick Commands

```bash
# List contract ABCs
grep -n "^class I[A-Za-z0-9_]*Port\|^class I[A-Za-z0-9_]*Protocol\|^class I[A-Za-z0-9_]*Aggregate" modules/shared/src/**/contract_*.py

# Check forbidden imports in contract files
grep -n "from capabilities_\|from infrastructure_\|from agent_\|from surface_" modules/shared/src/*/contract_*.py

# Check methods without @abstractmethod
grep -n "^class " modules/shared/src/*/contract_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -q "@abstractmethod" "$file" || echo "NO ABSTRACT: $file"
done

# Check unregistered contract files
for file in modules/shared/src/<domain>/contract_*.py; do
  name=$(basename "$file" .py)
  grep -q "from \.$name import" modules/shared/src/<domain>/__init__.py \
    || echo "UNREGISTERED: $name"
done
```
