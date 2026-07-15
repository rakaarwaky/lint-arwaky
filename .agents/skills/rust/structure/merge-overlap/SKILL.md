---
name: merge-overlap-rust
version: 1.0.0
category: refactoring
tags: [aes, merge, overlap, deduplication, consolidation, rust]
triggers:
  - "merge overlap rust"
  - "merge files rust"
  - "consolidate files rust"
  - "deduplicate modules rust"
dependencies: []
related:
  - clean-bloat
  - trait-consolidation
---

# merge-overlap-rust

## Rules

- Keep the file with the most logic as the target
- Move unique functions from source files into target
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

```rust
capabilities_world_to_camera.rs
capabilities_camera_to_world.rs
capabilities_camera_to_viewport.rs
// All do coordinate transforms → merge into capabilities_coordinate_mapper.rs
```

### Same-Feature Files (Merge)

```rust
capabilities_brush_cursor_drawer.rs
capabilities_drag_cursor_drawer.rs
capabilities_cursor_data_renderer.rs
// All render cursors → merge into capabilities_cursor_renderer.rs
```

### Same-Technology Adapters (Merge)

```rust
infrastructure_ffmpeg_adapter.rs
infrastructure_video_ffmpeg_adapter.rs
// Both use FFmpeg → merge into 1 adapter
```

## Workflow

### Step 1: Find Overlaps

Group files by concept/feature/technology.

### Step 2: Pick Target

Select the file with the most logic as merge target.

### Step 3: Merge Functions

Move unique functions from source files into target struct.

### Step 4: Delete Source Files

Remove merged source files.

### Step 5: Update Imports

Fix all references to deleted files.

### Step 6: Verify

Run `cargo check` and `cargo clippy`.
