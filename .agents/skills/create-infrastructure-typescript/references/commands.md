# Quick Commands

```bash
# List port interface implementations
grep -n "implements I[A-Za-z0-9_]*Port" packages/*/src/infrastructure_*.ts

# Check business logic keywords
grep -n "is_orphan\|analyze\|validate\|calculate\|compute\|business" packages/*/src/infrastructure_*.ts

# Check forbidden imports
grep -n "^\s*from\s+.*capabilities_\|^\s*from\s+.*agent_" packages/*/src/infrastructure_*.ts

# Check TypeScript
npx tsc --noEmit
```

## Check Wrong Block Order

```bash
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!port_line) port_line = NR }
    END { if (util_line && port_line && util_line < port_line) print "VIOLATION: utility method (line " util_line ") before port method (line " port_line ")" }
' packages/*/src/infrastructure_*.ts
```
