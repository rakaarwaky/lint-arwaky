---
name: merge-overlap
version: 1.0.0
category: refactoring
tags: [aes, merge, overlap, deduplication, consolidation]
triggers:
  - "merge overlap"
  - "merge files"
  - "consolidate files"
  - "deduplicate modules"
dependencies: []
related:
  - clean-bloat
  - fix-class-wrapping
---

# merge-overlap

## Rules

- Keep the file with the most logic as the target
- Move unique methods from source files into target
- Delete source files after merge
- Update all imports after merge

## Purpose

Merge files with overlapping concerns into a single file.

## When to Use

- Multiple files implement the same concept (e.g., 7 coordinate transform files)
- Multiple files handle the same feature (e.g., cursor drawer + cursor renderer)
- Multiple adapter files for the same technology (e.g., 3 FFmpeg adapters)

## The Fundamental Question

> **"Do these files do the same thing?"**

If yes → **Merge them into 1 file**

## Detection Patterns

### Same-Concept Files (Merge)
```
capabilities_world_to_camera.py
capabilities_camera_to_world.py
capabilities_camera_to_viewport.py
# All do coordinate transforms → merge into capabilities_coordinate_mapper.py
```

### Same-Feature Files (Merge)
```
capabilities_brush_cursor_drawer.py
capabilities_drag_cursor_drawer.py
capabilities_cursor_data_renderer.py
# All render cursors → merge into capabilities_cursor_renderer.py
```

### Same-Technology Adapters (Merge)
```
infrastructure_ffmpeg_adapter.py
infrastructure_video_ffmpeg_adapter.py
# Both use FFmpeg → merge into 1 adapter
```

## Workflow

### Step 1: Find Overlaps
Group files by concept/feature/technology.

### Step 2: Pick Target
Select the file with the most logic as merge target.

### Step 3: Merge Methods
Move unique methods from source files into target class.

### Step 4: Delete Source Files
Remove merged source files.

### Step 5: Update Imports
Fix all references to deleted files.

### Step 6: Verify
Run syntax check and lint.
