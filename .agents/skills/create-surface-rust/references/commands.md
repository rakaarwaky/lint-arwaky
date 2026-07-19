# Quick Commands

```bash
# Check forbidden lower-layer imports in all surface files
rg -n "^\s*use\s+.*(capabilities_|infrastructure_|agent_)" crates/*/src/surface_*.rs

# Check smart surfaces for forbidden imports
rg -n "^\s*use\s+.*(capabilities_|infrastructure_|agent_)" crates/*/src/surface_*_command.rs crates/*/src/surface_*_controller.rs

# Check utility surfaces importing smart surfaces
rg -n "surface_.*_(command|controller|page|entry)" crates/*/src/surface_*_hook.rs crates/*/src/surface_*_store.rs crates/*/src/surface_*_action.rs crates/*/src/surface_*_screen.rs

# Check possible domain computation in passive surfaces
rg "\.len\(\)|\.sum\(\)|\.fold\(|\.map\(" crates/*/src/surface_*_component.rs crates/*/src/surface_*_view.rs crates/*/src/surface_*_layout.rs

# Check possible unwrap usage
rg "unwrap\(\)|unwrap_or_default\(\)" crates/*/src/surface_*.rs

# Find possible local dataclasses in surface files
rg -n "^\s*pub struct" crates/*/src/surface_*.rs
```
