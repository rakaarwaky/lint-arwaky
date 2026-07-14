
### Check & Fix

```bash
# Scan directory
lint-arwaky-cli scan modules/

# Auto-fix
lint-arwaky-cli fix modules/

# Preview fixes
lint-arwaky-cli fix modules/ --dry-run

# CI mode (exit 1 if score < 80)
lint-arwaky-cli check modules/ --threshold 80
```

### Git Integration

```bash
# Install pre-commit hook
lint-arwaky-cli install-hook

# Remove hook
lint-arwaky-cli uninstall-hook
```

### Watch Mode

```bash
# Watch and re-lint on changes
lint-arwaky-cli watch modules/
```

### Setup

```bash
# Initialize config
lint-arwaky-cli setup init

# Install dependencies
lint-arwaky-cli setup install

# Show config
lint-arwaky-cli config show

# Show adapters
lint-arwaky-cli adapters
```

### Diagnostics

```bash

# Check environment
lint-arwaky-cli doctor

# Show version
lint-arwaky-cli version
```