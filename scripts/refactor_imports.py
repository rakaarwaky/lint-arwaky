#!/usr/bin/env python3
"""
refactor_imports.py — Automated barrel import refactoring for feature crates.

Scans all feature crate files and replaces deep import paths (e.g.
`shared::common::taxonomy_path_vo::FilePath`) with barrel re-exports
(e.g. `shared::common::FilePath`) where available.

Also replaces inline deep type paths in function signatures and bodies.

Usage: python3 scripts/refactor_imports.py
"""

import os
import re


# ─── Step 1: Build barrel re-export map from shared mod.rs files ──────────

SHARED_SRC = "crates/shared/src"
CRATES_DIR = "crates"
EXCLUDE_CRATES = {"shared"}  # skip the shared crate itself

# Module name in shared's lib.rs → directory name
MODULE_DIR_MAP = {
    "auto_fix": "auto-fix",
    "cli_commands": "cli-commands",
    "code_analysis": "code-analysis",
    "config_system": "config-system",
    "external_lint": "external-lint",
    "file_watch": "file-watch",
    "git_hooks": "git-hooks",
    "import_rules": "import-rules",
    "mcp_server": "mcp-server",
    "naming_rules": "naming-rules",
    "orphan_detector": "orphan-detector",
    "project_setup": "project-setup",
    "role_rules": "role-rules",
    "tui": "tui",
    "maintenance": "maintenance",
    "report_formatter": "report-formatter",
}


def build_barrel_map():
    """Parse shared mod.rs files to build {module_name: {type_name: submodule}}"""
    barrel_map = {}
    for mod_name, dir_name in MODULE_DIR_MAP.items():
        mod_rs = os.path.join(SHARED_SRC, dir_name, "mod.rs")
        if not os.path.exists(mod_rs):
            continue
        types = set()
        with open(mod_rs) as f:
            for line in f:
                m = re.match(r'^\s*pub use\s+(\w+)::(\w+);', line)
                if m:
                    submodule, type_name = m.group(1), m.group(2)
                    types.add((type_name, submodule))
        if types:
            barrel_map[mod_name] = types
    return barrel_map


# ─── Step 2: Find deep import patterns in files ──────────────────────────

def find_feature_crate_files():
    """Find all .rs files in feature crates (excluding shared crate)."""
    files = []
    for crate in os.listdir(CRATES_DIR):
        crate_path = os.path.join(CRATES_DIR, crate)
        src_path = os.path.join(crate_path, "src")
        if crate in EXCLUDE_CRATES or not os.path.isdir(src_path):
            continue
        for root, _, filenames in os.walk(src_path):
            for fn in filenames:
                if fn.endswith(".rs"):
                    files.append(os.path.join(root, fn))
    return files


# ─── Step 3: Transform deep imports ──────────────────────────────────────

def transform_imports(content, barrel_map):
    """Transform deep imports to barrel imports in a file's content."""
    lines = content.split('\n')
    result = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check for multi-line imports that need processing
        multi_start = re.match(
            r'^(use\s+shared::[a-z_]+::[a-z_]+)::\{',
            stripped
        )
        if multi_start and stripped.endswith('{'):
            block_lines = [line]
            brace_count = stripped.count('{') - stripped.count('}')
            j = i + 1
            while j < len(lines) and brace_count > 0:
                block_lines.append(lines[j])
                brace_count += lines[j].count('{') - lines[j].count('}')
                j += 1

            new_block = _process_multiline_import(block_lines, barrel_map)
            result.extend(new_block)
            i = j
            continue

        # Single-line import
        new_line = _transform_single_import(line, barrel_map)
        result.append(new_line if new_line != line else line)
        i += 1

    new_content = '\n'.join(result)
    new_content = _transform_inline_paths(new_content, barrel_map)
    return new_content


def _transform_single_import(line, barrel_map):
    """Transform a single-line import statement."""
    # Match: use shared::module_name::submodule::Type;
    m = re.match(
        r'^(use\s+shared::([a-z_]+)::([a-z_]+)::(\w+));?\s*$',
        line.strip()
    )
    if m:
        prefix, module, submodule, type_name = m.groups()
        mod_key = module
        if mod_key in barrel_map:
            types = {t[0] for t in barrel_map[mod_key]}
            if type_name in types:
                indent = re.match(r'^(\s*)', line).group(1)
                return f"{indent}{prefix}::{type_name};\n"

    # Match: use shared::module_name::submodule::{Type1, Type2, ...};
    m = re.match(
        r'^(use\s+shared::([a-z_]+)::([a-z_]+)::)\{([^}]+)\};?\s*$',
        line.strip()
    )
    if m:
        prefix, module, submodule, types_str = m.groups()
        types_in_import = [t.strip() for t in types_str.split(',') if t.strip()]
        mod_key = module
        if mod_key in barrel_map:
            barrel_types = {t[0] for t in barrel_map[mod_key]}
            new_types = [t for t in types_in_import if t in barrel_types]
            deep_types = [t for t in types_in_import if t not in barrel_types]
            indent = re.match(r'^(\s*)', line).group(1)
            if new_types and not deep_types:
                return f"{indent}use shared::{mod_key}::{{{', '.join(new_types)}}};\n"
            elif new_types and deep_types:
                barrel_line = f"use shared::{mod_key}::{{{', '.join(new_types)}}};"
                deep_line = f"use shared::{mod_key}::{submodule}::{{{', '.join(deep_types)}}};"
                return f"{indent}{barrel_line}\n{indent}{deep_line}\n"

    return line


def _process_multiline_import(block_lines, barrel_map):
    """Process a multi-line import block."""
    first = block_lines[0].strip()
    m = re.match(r'^use\s+(shared::([a-z_]+)::([a-z_]+))::\{', first)
    if not m:
        return block_lines

    full_path, module, submodule = m.groups()
    mod_key = module
    if mod_key not in barrel_map:
        return block_lines

    barrel_types = {t[0] for t in barrel_map[mod_key]}

    types = []
    for l in block_lines[1:]:
        stripped = l.strip().rstrip(',')
        if stripped and stripped not in ('}', '};'):
            types.append(stripped)

    barrel_imports = [t for t in types if t in barrel_types]
    deep_imports = [t for t in types if t not in barrel_types]

    indent = re.match(r'^(\s*)', block_lines[0]).group(1)
    result = []
    if barrel_imports:
        result.append(
            f"{indent}use shared::{mod_key}::{{{', '.join(barrel_imports)}}};\n"
        )
    if deep_imports:
        result.append(
            f"{indent}use {full_path}::{{{', '.join(deep_imports)}}};\n"
        )
    return result if result else block_lines


def _transform_inline_paths(content, barrel_map):
    """Replace inline deep paths with barrel paths."""
    for mod_key, types_info in barrel_map.items():
        for type_name, submodule in types_info:
            pattern = re.compile(
                rf'(?<!use\s)(shared::{mod_key}::{submodule}::{type_name})(?!\w)'
            )
            content = pattern.sub(f'shared::{mod_key}::{type_name}', content)
    return content


# ─── Step 4: Fix inline type annotations ────────────────────────────────

def fix_type_annotations(content, barrel_map):
    """
    Fix inline type annotations using flat paths like
    `shared::taxonomy_common_vo::LineNumber` to use barrel module paths.
    """
    common_submodules = {
        'taxonomy_common_vo': 'common',
        'taxonomy_layer_vo': 'common',
        'taxonomy_path_vo': 'common',
        'taxonomy_paths_vo': 'common',
        'taxonomy_severity_vo': 'common',
        'taxonomy_adapter_error': 'common',
        'taxonomy_adapter_name_vo': 'common',
        'taxonomy_adapter_list_vo': 'common',
        'taxonomy_display_content_vo': 'common',
        'taxonomy_git_vo': 'common',
        'taxonomy_message_vo': 'common',
        'taxonomy_response_data_vo': 'common',
        'taxonomy_source_vo': 'common',
        'taxonomy_common_error': 'common',
        'taxonomy_job_vo': 'common',
        'taxonomy_duration_vo': 'common',
        'taxonomy_suggestion_vo': 'common',
        'taxonomy_threshold_vo': 'common',
        'taxonomy_error_vo': 'common',
        'taxonomy_language_vo': 'common',
        'taxonomy_lint_vo': 'common',
        'taxonomy_name_vo': 'common',
        'taxonomy_language_info_vo': 'common',
        'taxonomy_definition_vo': 'common',
        'taxonomy_filesystem_error': 'common',
        'taxonomy_action_vo': 'common',
        'taxonomy_line_count_vo': 'common',
        'taxonomy_suffix_vo': 'common',
    }

    # Handle flat-path pattern: shared::submodule::Type → shared::common::Type
    for submodule, barrel_module in common_submodules.items():
        pattern_text = rf'(shared::{submodule}::(\w+))'
        for m in re.finditer(pattern_text, content):
            type_name = m.group(2)
            if barrel_module in barrel_map:
                for bt, _ in barrel_map[barrel_module]:
                    if bt == type_name:
                        old = m.group(0)
                        new = f'shared::{barrel_module}::{type_name}'
                        content = content.replace(old, new, 1)
                        break

    # Also fix: shared::feature_module::submodule::Type → shared::feature_module::Type
    feature_modules = [
        'code_analysis', 'config_system', 'import_rules', 'naming_rules',
        'orphan_detector', 'role_rules', 'auto_fix', 'cli_commands',
        'external_lint', 'file_watch', 'git_hooks', 'mcp_server',
        'project_setup', 'tui', 'maintenance', 'report_formatter',
    ]
    for mod_key in feature_modules:
        if mod_key not in barrel_map:
            continue
        for type_name, submodule in barrel_map[mod_key]:
            pattern_text = rf'(shared::{mod_key}::{submodule}::{type_name})(?!\w)'
            for m in re.finditer(pattern_text, content):
                content = content.replace(
                    m.group(0), f'shared::{mod_key}::{type_name}', 1
                )

    return content


# ─── Main ─────────────────────────────────────────────────────────────────

def main():
    barrel_map = build_barrel_map()
    print(f"Built barrel map: {len(barrel_map)} modules with re-exports")
    for mod, types in sorted(barrel_map.items()):
        print(f"  {mod}: {len(types)} types")

    files = find_feature_crate_files()
    print(f"\nFound {len(files)} feature crate files to process")

    modified_count = 0
    import_pattern = re.compile(r'use\s+shared::[a-z_]+::[a-z_]+\w*::')

    for filepath in sorted(files):
        with open(filepath) as f:
            original = f.read()

        if not import_pattern.search(original):
            continue

        content = transform_imports(original, barrel_map)
        content = fix_type_annotations(content, barrel_map)

        if content != original:
            with open(filepath, 'w') as f:
                f.write(content)
            modified_count += 1
            print(f"  Modified: {filepath}")

    print(f"\nModified {modified_count} files")


if __name__ == '__main__':
    main()
