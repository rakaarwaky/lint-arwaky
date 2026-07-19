# Quick Commands

```bash
# Check forbidden lower-layer imports in all surface files
grep -n "^\s*from\s+.*(capabilities_|infrastructure_|agent_)" modules/*/src/surface_*.py

# Check smart surfaces for forbidden imports
grep -n "^\s*from\s+.*(capabilities_|infrastructure_|agent_)" modules/*/src/surface_*_command.py modules/*/src/surface_*_controller.py

# Check utility surfaces importing smart surfaces
grep -n "surface_.*_command\|surface_.*_controller" modules/*/src/surface_*_hook.py modules/*/src/surface_*_store.py modules/*/src/surface_*_action.py modules/*/src/surface_*_screen.py

# Check possible domain computation in passive surfaces
grep -n "\.len(\|\.sum(\|\.map(\|if.*>" modules/*/src/surface_*_component.py modules/*/src/surface_*_view.py modules/*/src/surface_*_layout.py

# Check syntax
python -c "import <module>"
```
