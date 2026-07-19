#!/bin/bash
# Refactor BooleanVO → bool
# Usage: ./scripts/refactor-booleanvo-to-bool.sh [--dry-run]

set -euo pipefail

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "=== DRY RUN MODE ==="
fi

# Find all Rust files that use BooleanVO
FILES=$(grep -rl "BooleanVO" crates/ --include="*.rs" 2>/dev/null || true)

if [[ -z "$FILES" ]]; then
    echo "No files found with BooleanVO"
    exit 0
fi

echo "Found $(echo "$FILES" | wc -l) files to refactor"
echo ""

for file in $FILES; do
    echo "Processing: $file"
    
    if [[ "$DRY_RUN" == "false" ]]; then
        # 1. Type annotations: BooleanVO → bool
        sed -i 's/BooleanVO/bool/g' "$file"
        
        # 2. Constructor: BooleanVO::new(x) → x
        sed -i 's/BooleanVO::new(\([^)]*\))/\1/g' "$file"
        
        # 3. From: BooleanVO::from(x) → x  
        sed -i 's/BooleanVO::from(\([^)]*\))/\1/g' "$file"
        
        # 4. Struct literal: BooleanVO { value: x } → x
        sed -i 's/BooleanVO { value: \([^}]*\) }/\1/g' "$file"
        
        # 5. Remove unused imports
        sed -i '/use.*BooleanVO/d' "$file"
    fi
    
    echo "  ✓ Done"
done

echo ""
echo "=== Summary ==="
echo "Files processed: $(echo "$FILES" | wc -l)"

if [[ "$DRY_RUN" == "true" ]]; then
    echo ""
    echo "Run without --dry-run to apply changes"
else
    echo ""
    echo "Changes applied. Run 'cargo build' to verify"
    echo "Run 'git diff' to review changes"
fi
