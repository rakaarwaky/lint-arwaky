# Quick Commands

```bash
# List classes in agent files
grep -n "^class " modules/*/src/agent_*.py

# List aggregate ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Aggregate" modules/*/src/agent_*.py

# Check computation patterns
grep -n "sum(\|len(\|\.iter\(\)\|\.map(" modules/*/src/agent_*.py

# Check I/O in agents
grep -n "open(\|Path(\|os\.\|requests\.\|httpx\." modules/*/src/agent_*.py

# Check forbidden imports
grep -n "^\s*from\s+.*(capabilities_|infrastructure_)" modules/*/src/agent_*.py

# Check syntax
python -c "import <module>"
```

## Check Wrong Block Order

```bash
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_agg = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_agg is None:
            first_agg = i + 1
    if first_dunder and first_agg and first_dunder < first_agg:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before aggregate method (line {first_agg})')
" modules/*/src/agent_*.py
```
