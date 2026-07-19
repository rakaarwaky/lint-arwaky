# Quick Commands

```bash
# Check forbidden lower-layer imports in all surface files
grep -n "^\s*from\s+.*capabilities_\|^\s*from\s+.*infrastructure_\|^\s*from\s+.*agent_" packages/*/src/surface_*.ts

# Check smart surfaces for forbidden imports
grep -n "^\s*from\s+.*capabilities_\|^\s*from\s+.*infrastructure_\|^\s*from\s+.*agent_" packages/*/src/surface_*_command.ts packages/*/src/surface_*_controller.ts

# Check utility surfaces importing smart surfaces
grep -n "surface_.*_command\|surface_.*_controller" packages/*/src/surface_*_hook.ts packages/*/src/surface_*_store.ts packages/*/src/surface_*_action.ts packages/*/src/surface_*_screen.ts

# Check possible domain computation in passive surfaces
grep -n "\.length\|\.reduce\|\.map\|if.*>" packages/*/src/surface_*_component.ts packages/*/src/surface_*_view.ts packages/*/src/surface_*_layout.ts

# Check TypeScript
npx tsc --noEmit
```
