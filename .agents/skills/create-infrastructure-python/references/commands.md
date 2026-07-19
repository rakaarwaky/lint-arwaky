# Quick Commands

```bash
# List classes in infrastructure files
grep -n "^class " modules/*/src/infrastructure_*.py

# List port ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Port" modules/*/src/infrastructure_*.py

# Check business logic keywords
grep -n "is_orphan\|analyze\|validate\|calculate\|compute\|business" modules/*/src/infrastructure_*.py

# Check forbidden imports
grep -n "^\s*from\s+.*(capabilities_|agent_)" modules/*/src/infrastructure_*.py

# Check syntax
python -c "import <module>"
```

## Check Wrong Block Order

```bash
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_port = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_port is None:
            first_port = i + 1
    if first_dunder and first_port and first_dunder < first_port:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before port method (line {first_port})')
" modules/*/src/infrastructure_*.py
```
