# Quick Commands

```bash
# Check I/O in capabilities (AES404)
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/capabilities_*.ts

# Check forbidden imports
grep -n "^\s*from.*capabilities_|from.*agent_|from.*surface_*.ts

# List protocol interface implementations
grep -n "implements I[A-Za-z0-9_]*Protocol" packages/*/src/capabilities_*.ts

# Check capability filename follows role naming capabilities_<domain>_<role>.ts
grep -rnE "capabilities_[a-z_]+_[a-z_]+\.ts$" packages/*/src/capabilities_*.ts || echo "FILENAME VIOLATION"

# Check for inter-capability dependency (forbidden)
grep -n "^\s*from.*capabilities_|from.*agent_|from.*surface_*.ts

# Check for orchestration anti-patterns in capabilities (No Orchestration, §8)
grep -nE "for \(.* of|while \(|\.escalate\(|error_escalation" packages/*/src/capabilities_*.ts

# Find error swallowing patterns
grep -n "?? ''\|?? \"\"\||| 0" packages/*/src/capabilities_*.ts

# Check TypeScript
npx tsc --noEmit
```

## Check Wrong Block Order

```bash
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!proto_line) proto_line = NR }
    END { if (util_line && proto_line && util_line < proto_line) print "VIOLATION: utility method (line " util_line ") before protocol method (line " proto_line ")" }
' packages/*/src/capabilities_*.ts
```
