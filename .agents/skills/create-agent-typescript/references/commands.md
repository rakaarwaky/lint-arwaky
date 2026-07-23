# Quick Commands

```bash
# List classes in agent files
grep -n "^export class " packages/*/src/agent_*.ts

# List aggregate interface implementations
grep -n "implements I[A-Za-z0-9_]*Aggregate" packages/*/src/agent_*.ts

# Check computation patterns
grep -n "\.length\|\.reduce\|\.map\|\.filter" packages/*/src/agent_*.ts

# Check I/O in agents
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/agent_*.ts

# Check forbidden imports
grep -n "^\s*from.*capabilities_|from.*agent_|from.*surface_*.ts

# Check TypeScript
npx tsc --noEmit
```

## Check Wrong Block Order

```bash
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!agg_line) agg_line = NR }
    END { if (util_line && agg_line && util_line < agg_line) print "VIOLATION: utility method (line " util_line ") before aggregate method (line " agg_line ")" }
' packages/*/src/agent_*.ts
```
