#!/usr/bin/env python3
import os
import re
import sys
from pathlib import Path

def main():
    print("=== Lint Arwaky Feature Exporter ===")
    
    # 1. Resolve workspace root
    # Since this script is placed in scripts/, parent is the workspace root.
    workspace_root = Path(__file__).resolve().parent.parent
    crates_dir = workspace_root / "crates"
    docs_finding_dir = workspace_root / "docs" / "finding"
    
    if not crates_dir.exists():
        print(f"Error: 'crates' directory not found at {crates_dir}", file=sys.stderr)
        sys.exit(1)
        
    # 2. Get list of feature crates
    feature_crates = []
    for entry in crates_dir.iterdir():
        if entry.is_dir() and entry.name != "shared":
            if (entry / "Cargo.toml").exists():
                feature_crates.append(entry.name)
    feature_crates = sorted(feature_crates)
    
    if not feature_crates:
        print("Error: No feature crates found in 'crates' directory.", file=sys.stderr)
        sys.exit(1)
        
    # 3. Interactive Menu
    print("Available feature crates:")
    for i, name in enumerate(feature_crates, 1):
        print(f"{i:2d}) {name}")
    print()
    
    while True:
        try:
            choice = input(f"Select a crate (1-{len(feature_crates)}) or 'q' to quit: ").strip()
            if choice.lower() == 'q':
                print("Exiting.")
                sys.exit(0)
            idx = int(choice) - 1
            if 0 <= idx < len(feature_crates):
                selected_crate = feature_crates[idx]
                break
            else:
                print(f"Error: Please choose a number between 1 and {len(feature_crates)}.")
        except ValueError:
            print("Error: Invalid input. Please enter a valid number or 'q'.")
            
    print(f"\nProcessing crate: {selected_crate}...")
    
    # 4. Read package version
    crate_path = crates_dir / selected_crate
    cargo_toml_path = crate_path / "Cargo.toml"
    version = "0.1.0"
    if cargo_toml_path.exists():
        try:
            content = cargo_toml_path.read_text(errors='ignore')
            match = re.search(r'(?m)^version\s*=\s*"([^"]+)"', content)
            if match:
                version = match.group(1)
        except Exception as e:
            print(f"Warning: Could not parse version from Cargo.toml ({e}). Defaulting to {version}.")
            
    print(f"Version resolved: {version}")
    
    # 5. Index shared crate files
    shared_src_dir = crates_dir / "shared" / "src"
    module_to_files = {}
    symbol_to_files = {}
    
    if shared_src_dir.exists():
        print("Indexing shared crate for resolving dependencies...")
        shared_files = list(shared_src_dir.rglob("*.rs"))
        for f in shared_files:
            # Index modules by file name / stem and parent directory if mod.rs
            if f.name == "mod.rs":
                mod_name = f.parent.name.replace('-', '_')
            else:
                mod_name = f.stem.replace('-', '_')
                
            if mod_name not in module_to_files:
                module_to_files[mod_name] = []
            module_to_files[mod_name].append(f)
            
            # Read and index public symbols in each file
            try:
                content = f.read_text(errors='ignore')
                # Pattern to match: pub struct/enum/trait/type/fn/const/static Name
                decl_pattern = r'\bpub(?:\([^)]+\))?\s+(struct|enum|trait|type|fn|const|static|mod)\s+([a-zA-Z0-9_]+)'
                for match in re.finditer(decl_pattern, content):
                    decl_type = match.group(1)
                    decl_name = match.group(2)
                    if decl_type == 'mod':
                        if decl_name not in module_to_files:
                            module_to_files[decl_name] = []
                        module_to_files[decl_name].append(f)
                    else:
                        if decl_name not in symbol_to_files:
                            symbol_to_files[decl_name] = []
                        symbol_to_files[decl_name].append(f)
                
                # Capture macro_rules! definitions
                macro_pattern = r'\bmacro_rules!\s+([a-zA-Z0-9_]+)'
                for match in re.finditer(macro_pattern, content):
                    macro_name = match.group(1)
                    if macro_name not in symbol_to_files:
                        symbol_to_files[macro_name] = []
                    symbol_to_files[macro_name].append(f)
            except Exception as e:
                print(f"Warning: Failed to index file {f} ({e})")
    else:
        print("Warning: 'crates/shared/src' directory not found. Shared dependencies cannot be resolved.")
        
    # Helper to resolve dependency files based on parsed components
    def resolve_dependency_files(components):
        resolved = set()
        for comp in components:
            # Look up in module map
            if comp in module_to_files:
                for f in module_to_files[comp]:
                    resolved.add(f)
                    
            # Look up in symbol map
            if comp in symbol_to_files:
                files = symbol_to_files[comp]
                if len(files) == 1:
                    resolved.add(files[0])
                elif len(files) > 1:
                    # Score and rank candidates based on matching path components
                    scored = []
                    for f in files:
                        score = 0
                        f_parts = [p.replace('-', '_') for p in f.parts]
                        for c in components:
                            if c in f_parts:
                                score += 1
                        scored.append((score, f))
                    scored.sort(key=lambda x: x[0], reverse=True)
                    best_score = scored[0][0]
                    if best_score > 0:
                        for score, f in scored:
                            if score == best_score:
                                resolved.add(f)
                    else:
                        resolved.update(files)
        return resolved

    # 6. Gather target files
    files_to_export = set()
    
    # Add files from the selected crate itself
    for f in crate_path.rglob("*"):
        if f.is_file():
            # Include Cargo.toml and any file under src/ (like .rs files)
            if f.name == "Cargo.toml" or (crate_path / "src") in f.parents:
                files_to_export.add(f)
                
    # Add dedicated shared folder for the selected feature if exists
    # E.g. crates/shared/src/import-rules/ -> map selected_crate name to dashes
    feature_dashed = selected_crate.replace('_', '-')
    shared_feature_dir = shared_src_dir / feature_dashed
    if shared_feature_dir.exists() and shared_feature_dir.is_dir():
        for f in shared_feature_dir.rglob("*.rs"):
            if f.is_file():
                files_to_export.add(f)
                
    # Scan feature crate files to resolve imported shared dependencies
    print("Scanning source files for imported shared dependencies...")
    shared_imports_found = set()
    for f in list(files_to_export):
        if f.suffix == ".rs":
            try:
                content = f.read_text(errors='ignore')
                # Find all shared::path::to::...
                matches = re.findall(r'\bshared::([a-zA-Z0-9_:]+)', content)
                for path_str in matches:
                    components = path_str.split("::")
                    resolved_files = resolve_dependency_files(components)
                    shared_imports_found.update(resolved_files)
            except Exception as e:
                print(f"Warning: Failed to read file {f} for dependency analysis ({e})")
                
    files_to_export.update(shared_imports_found)
    
    # 7. Write Markdown file
    docs_finding_dir.mkdir(parents=True, exist_ok=True)
    output_filename = f"{selected_crate}_v{version}.md"
    output_path = docs_finding_dir / output_filename
    
    print(f"Writing export to {output_path}...")
    
    sorted_files = sorted(list(files_to_export))
    
    with open(output_path, "w", encoding="utf-8") as out:
        out.write(f"# Crate: {selected_crate} (v{version})\n\n")
        out.write(f"This document contains the source code for feature crate `{selected_crate}` ")
        out.write("along with its corresponding and imported definitions from the `shared` crate.\n\n")
        
        out.write("## File List\n\n")
        for f in sorted_files:
            rel = f.relative_to(workspace_root)
            # Clickable file:// URL scheme
            file_url = f.as_uri()
            out.write(f"- [{rel}]({file_url})\n")
        
        out.write("\n---\n\n")
        
        for f in sorted_files:
            rel = f.relative_to(workspace_root)
            out.write(f"## File: {rel}\n\n")
            
            # Syntax highlighting
            lang = "rust"
            if f.name == "Cargo.toml":
                lang = "toml"
            elif f.suffix == ".py":
                lang = "python"
            elif f.suffix in (".js", ".ts"):
                lang = "javascript"
                
            out.write(f"```{lang}\n")
            try:
                content = f.read_text(errors='ignore')
                out.write(content)
                if not content.endswith('\n'):
                    out.write('\n')
            except Exception as e:
                out.write(f"/* Error reading file: {e} */\n")
            out.write("```\n\n")
            out.write("---\n\n")
            
    print(f"Success! Consolidate markdown file created: {output_path}")

if __name__ == "__main__":
    main()
