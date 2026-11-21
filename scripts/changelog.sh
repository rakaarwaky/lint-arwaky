#!/usr/bin/env bash
# changelog.sh — keep only the newest 2 releases in CHANGELOG.md.
#
# Pattern: current release (full) + 1 previous release (summary).
# Older releases are dropped (not archived). The dropped versions still
# exist as git tags — this only shortens the file.
#
# Usage:
#   bash scripts/changelog.sh
#   bash scripts/changelog.sh --dry-run   # print what would change
#
# Assumes CHANGELOG.md uses the conventional git-cliff format:
#   ## [vX.Y.Z] - YYYY-MM-DD
#   ### Added
#   - ...
#   ### Fixed
#   - ...
#
# Exits 0 on success, 1 if CHANGELOG.md is missing or malformed.

set -euo pipefail
CHANGELOG="${CHANGELOG:-CHANGELOG.md}"
DRY_RUN="${1:-}"

if [ ! -f "$CHANGELOG" ]; then
  echo "changelog: $CHANGELOG not found" >&2
  exit 1
fi

# Count version headings (## [v... or ## <version>)
count=$(grep -cE '^## (\[)?(v|[0-9])' "$CHANGELOG" || true)
if [ "$count" -le 2 ]; then
  echo "changelog: only $count release(s) in $CHANGELOG — nothing to trim"
  exit 0
fi

if [ "$DRY_RUN" = "--dry-run" ]; then
  echo "changelog: would keep newest 2 of $count releases in $CHANGELOG"
  grep -nE '^## (\[)?(v|[0-9])' "$CHANGELOG" | head -n "$count" | tail -n +3
  exit 0
fi

python3 - <<'PY'
from pathlib import Path
import re, sys

path = Path("CHANGELOG.md")
text = path.read_text()
lines = text.splitlines(keepends=True)

# Split into preamble (header) + release blocks
# A release starts at "## [v..." or "## <version>..."
release_re = re.compile(r'^## (\[)?(v|[0-9])')

head = []
blocks = []
cur_block = []
seen_release = False

for line in lines:
    if release_re.match(line):
        seen_release = True
        if cur_block:
            blocks.append(cur_block)
        cur_block = [line]
    elif seen_release:
        cur_block.append(line)
    else:
        head.append(line)

# Flush the last block
if cur_block:
    blocks.append(cur_block)

# Filter: drop empty blocks
blocks = [b for b in blocks if b and any(l.strip() for l in b)]

# Keep only the first 2 release blocks (newest first, as git-cliff prepends)
keep = blocks[:2]
dropped = blocks[2:]

if not dropped:
    print("changelog: nothing to drop (only %d releases)" % len(blocks))
    sys.exit(0)

out = "".join(head) + "".join("".join(b) for b in keep)
path.write_text(out)

# Report what was dropped
dropped_versions = []
for b in dropped:
    for l in b:
        m = release_re.match(l)
        if m:
            v = l.strip().lstrip('#').strip()
            v = v.strip('[]')
            dropped_versions.append(v)
            break

print("changelog: dropped %d release(s): %s" % (len(dropped_versions), ", ".join(dropped_versions)))
print("changelog: kept newest 2 in %s" % path.name)
PY