# Quick Commands

```bash
# Find possible dataclasses in layer files
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ --exclude-dir=shared

# Check forbidden imports in taxonomy files
grep -n "from capabilities_\|from infrastructure_\|from agent_\|from surface_" modules/shared/src/*/taxonomy_*.py

# Check possible I/O in taxonomy files
grep -n "open(\|Path(\|os\.\|requests\.\|httpx\.\|sqlite3\.\|asyncpg\." modules/shared/src/*/taxonomy_*.py

# List registered taxonomy modules
grep -n "^from \.taxonomy_" modules/shared/src/*/__init__.py
```

## Check Unregistered Taxonomy Files

```bash
for file in modules/shared/src/<domain>/taxonomy_*.py; do
  name=$(basename "$file" .py)
  grep -q "from \.$name import" modules/shared/src/<domain>/__init__.py \
    || echo "UNREGISTERED: $name"
done
```
