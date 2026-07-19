# Quick Commands

```bash
# Check I/O in capabilities (AES404)
grep -n "open(\|Path(\|os\.\|requests\.\|httpx\.\|sqlite3\.\|asyncpg\." modules/*/src/capabilities_*.py

# Check forbidden imports
grep -n "^\s*from\s+.*(infrastructure_|agent_)" modules/*/src/capabilities_*.py

# List classes in capabilities files
grep -n "^class " modules/*/src/capabilities_*.py

# List protocol ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Protocol" modules/*/src/capabilities_*.py

# Find error swallowing patterns
grep -n "or ''\|or \"\"\|or 0" modules/*/src/capabilities_*.py

# Check syntax
python -c "import <module>"
```

## Check Wrong Block Order

```bash
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_proto = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_proto is None:
            first_proto = i + 1
    if first_dunder and first_proto and first_dunder < first_proto:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before protocol method (line {first_proto})')
" modules/*/src/capabilities_*.py
```
