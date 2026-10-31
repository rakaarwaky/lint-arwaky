// PURPOSE: build script — stage publishable assets into OUT_DIR and generate
// build-time constants for the packages that invoke this script.
//
// Three tasks run here, keyed off the invoking manifest:
//
//   1. Config YAML + skill markdown staged into `OUT_DIR` for
//      `shared-project-setup-lint-arwaky` (`include_str!` of `skills/**`).
//   2. Release version emitted into `OUT_DIR/release_version.rs` for
//      `shared-common-lint-arwaky`.
//   3. Comrak codegen of the doc-linter shape constants into
//      `OUT_DIR/doc_generated.rs` for `shared-doc-rules-lint-arwaky`.
//
// All three asset groups must resolve for the workspace build *and* for the
// isolated build of a published `.crate` tarball (crates.io extracts the
// package to an isolated directory, so any path above the crate root is
// missing there). `resolve_assets` walks up from the manifest dir to the
// ancestor that carries both `config/` and `skills/`; the three manifest
// layouts (`crates/shared/src/{project_setup,common,doc_rules}`) all land
// on `crates/shared/` in a workspace build.
use std::fs;
use std::path::{Path, PathBuf};

use comrak::nodes::NodeValue;
use comrak::{Arena, Options, parse_document};

fn main() {
    let out_dir = match std::env::var("OUT_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("OUT_DIR not set: {e}");
            std::process::exit(1);
        }
    };
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("CARGO_MANIFEST_DIR not set: {e}");
            std::process::exit(1);
        }
    };

    let manifest_path = Path::new(&manifest_dir);
    let out_path = Path::new(&out_dir);
    let assets = resolve_assets(manifest_path);
    stage_config(&assets, out_path);
    stage_skills(&assets, out_path);
    stage_release_version(manifest_path, out_path);
    if is_doc_rules(manifest_path) {
        stage_doc_rules(manifest_path, out_path);
    }
}

/// Locate the directory that carries both `config/` and `skills/`.
///
/// The three manifest layouts this script is invoked from all sit under
/// `crates/shared/src/<pkg>`; in a workspace build the assets live one level
/// up at `crates/shared/`. In a published tarball the layout is flatter and
/// the manifest dir itself carries the assets. Walk up from the manifest
/// dir until the assets are found.
fn resolve_assets(manifest_dir: &Path) -> PathBuf {
    let mut current = manifest_dir.to_path_buf();
    loop {
        if current.join("config").is_dir() && current.join("skills").is_dir() {
            return current;
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break,
        }
    }
    eprintln!(
        "Assets not found: no ancestor of {} carries config/ and skills/",
        manifest_dir.display()
    );
    std::process::exit(1);
}

/// True when this script is being invoked for the doc-rules package. The
/// Comrak codegen runs only for doc_rules; the other packages share the
/// script but their manifests carry no doc-rules templates.
fn is_doc_rules(manifest_dir: &Path) -> bool {
    manifest_dir.file_name().is_some_and(|n| n == "doc_rules")
}

/// Copy `config/lint_arwaky.config.yaml` into `OUT_DIR`.
fn stage_config(manifest_dir: &Path, out_dir: &Path) {
    let name = "lint_arwaky.config.yaml";
    let src = manifest_dir.join("config").join(name);
    let dst = out_dir.join(name);

    if !src.exists() {
        eprintln!(
            "Config file not found at {}. Check that config/ is in the crate root.",
            src.display()
        );
        std::process::exit(1);
    }

    if let Err(e) = fs::copy(&src, &dst) {
        eprintln!("Failed to copy config file {name}: {e}");
        std::process::exit(1);
    }

    println!("cargo:rerun-if-changed={}", src.display());
}

/// Recursively copy skills source into `OUT_DIR/skills/` so the
/// `include_str!` sites in `taxonomy_project_setup_constant.rs` resolve at compile
/// time.
///
/// The source of truth is `<manifest>/skills/` in both workspace builds and
/// published crate builds. `[package] include` keeps the directory in the
/// crates.io tarball.
fn stage_skills(manifest_dir: &Path, out_dir: &Path) {
    let skills_src = manifest_dir.join("skills");
    if !skills_src.is_dir() {
        eprintln!(
            "Skills directory not found at {}. Check that skills/ is in the crate root.",
            skills_src.display()
        );
        std::process::exit(1);
    }

    let skills_dst = out_dir.join("skills");
    if let Err(e) = fs::create_dir_all(&skills_dst) {
        eprintln!("Failed to create {}: {e}", skills_dst.display());
        std::process::exit(1);
    }
    copy_dir_recursive(&skills_src, &skills_dst);

    println!("cargo:rerun-if-changed={}", skills_src.display());
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    let entries = match fs::read_dir(src) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to read skills dir {}: {e}", src.display());
            std::process::exit(1);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read entry in {}: {e}", src.display());
                std::process::exit(1);
            }
        };
        let path = entry.path();
        let target = dst.join(entry.file_name());

        if path.is_dir() {
            let sub = dst.join(entry.file_name());
            if let Err(e) = fs::create_dir_all(&sub) {
                eprintln!("Failed to create {}: {e}", sub.display());
                std::process::exit(1);
            }
            copy_dir_recursive(&path, &sub);
        } else if let Err(e) = fs::copy(&path, &target) {
            eprintln!("Failed to copy skill file {}: {e}", path.display());
            std::process::exit(1);
        }
    }
}

/// Emit the workspace `VERSION` file contents into `OUT_DIR/release_version.rs`
/// so every crate that prints a version reads one number. Walks up from the
/// invoking manifest dir to the ancestor carrying `VERSION`.
fn stage_release_version(manifest_dir: &Path, out_dir: &Path) {
    let version_file = find_version_file(manifest_dir);
    let Some(version_file) = version_file else {
        eprintln!(
            "VERSION file not found at or above {}",
            manifest_dir.display()
        );
        std::process::exit(1);
    };

    let raw = match fs::read_to_string(&version_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read {}: {e}", version_file.display());
            std::process::exit(1);
        }
    };
    let version = raw.trim();
    if version.is_empty() {
        eprintln!("{} is empty", version_file.display());
        std::process::exit(1);
    }

    let dest = out_dir.join("release_version.rs");
    let contents = format!(
        "// Generated by crates/shared/build.rs from the workspace\n\
         // VERSION file. Do not edit.\n\
         /// The lint-arwaky release version, identical in every binary and\n\
         /// every output format.\n\
         pub const RELEASE_VERSION: &str = \"{version}\";\n"
    );
    if let Err(e) = fs::write(&dest, &contents) {
        eprintln!("failed to write {}: {e}", dest.display());
        std::process::exit(1);
    }

    println!("cargo:rerun-if-changed={}", version_file.display());
}

/// Walk up from `start` to the directory carrying `VERSION`. The crate sits
/// at `crates/shared/src/common`, so the workspace root is three levels up;
/// the loop also covers the packaged layout where the file is copied in
/// beside the manifest.
fn find_version_file(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);
    while let Some(dir) = current {
        let candidate = dir.join("VERSION");
        if candidate.is_file() {
            return Some(candidate);
        }
        current = dir.parent();
    }
    None
}

/// Generate the doc-linter shape constants into `OUT_DIR/doc_generated.rs`.
///
/// Parse each `references/*/template.md` with Comrak, extract the required
/// H2s, table column headers, and fixed row labels, and emit them as
/// `pub const NAME: &[&str]` items. The generated module is included into
/// the doc-rules crate via `mod doc_generated { include!(concat!(env!(
/// "OUT_DIR"), "/doc_generated.rs")); }`. When a template changes, this
/// build script re-runs and the linter's shape constants track the
/// template automatically.
fn stage_doc_rules(manifest_dir: &Path, out_dir: &Path) {
    // The templates live under `crates/shared/skills/aes-docs/references/`.
    // From the doc_rules manifest dir that is two levels up.
    let shared_dir = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(manifest_dir);
    let skills_dir = shared_dir.join("skills/aes-docs/references");

    println!("cargo:rerun-if-changed={}", skills_dir.display());
    for doc in DOCS {
        let p = skills_dir.join(doc).join("template.md");
        println!("cargo:rerun-if-changed={}", p.display());
    }

    let mut out = String::from(
        "// AUTO-GENERATED by crates/shared/build.rs (stage_doc_rules).\n\
         // Source: crates/shared/skills/aes-docs/references/*/template.md\n\
         // Do not edit this file; edit the reference templates instead.\n\n",
    );

    for doc in DOCS {
        let path = skills_dir.join(doc).join("template.md");
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {} : {e}", path.display()));
        out = extract_doc_constants(doc, &raw, out);
    }

    let dest = out_dir.join("doc_generated.rs");
    if let Err(e) = fs::write(&dest, &out) {
        eprintln!("failed to write {}: {e}", dest.display());
        std::process::exit(1);
    }
}

/// The 10 reference template directories, in the order their constants are
/// emitted into the generated module.
const DOCS: &[&str] = &[
    "agents",
    "architecture",
    "backlog",
    "contributing",
    "data",
    "design",
    "frd",
    "prd",
    "readme",
    "roadmap",
];

/// Strip the outer `# <X> Template` title line and the wrapping
/// ` ```markdown ... ``` ` fence, returning only the inner document body
/// that the linter actually checks.
///
/// The outer template fence is the one written by the reference author; any
/// inner fences (e.g. a ` ```text ` tree block inside the template) are kept
/// verbatim. The outer closing fence is a plain ` ``` ` with no language tag,
/// reached at nesting depth zero.
fn extract_fenced_markdown(raw: &str) -> String {
    let open = raw
        .find("```markdown")
        .expect("template must open a ```markdown fence");
    let after_open = &raw[open + "```markdown".len()..];
    let mut out = String::new();
    // Tracks nesting: a language-tagged fence (```text) opens an inner block
    // (depth += 1); a plain ``` closes one (depth -= 1). The outer
    // ```markdown template closes when a plain ``` brings depth from 0 to 0,
    // i.e. the very first plain ``` we meet at depth 0 after any inner blocks
    // have opened and closed. We stop there.
    let mut depth: i32 = 0;
    for line in after_open.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            let tagged = trimmed.len() > 3;
            if tagged {
                depth += 1;
                out.push_str(line);
                out.push('\n');
            } else {
                // Plain ``` closes the most recent inner block if depth > 0;
                // at depth 0 it is the outer template boundary, so stop.
                if depth > 0 {
                    depth -= 1;
                    out.push_str(line);
                    out.push('\n');
                } else {
                    break;
                }
            }
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.trim_end().to_string()
}

fn parse_options<'o>() -> Options<'o> {
    let mut opts = Options::default();
    opts.extension.table = true;
    opts.extension.tasklist = true;
    opts
}

/// Collect the H2 heading titles of the parsed document in source order.
fn collect_h2s<'a>(root: comrak::nodes::Node<'a>) -> Vec<String> {
    let mut h2s = Vec::new();
    for node in root.children() {
        if let NodeValue::Heading(h) = node.data().value {
            if h.level == 2 {
                let text = node.collect_text().trim().to_string();
                if !text.is_empty() {
                    h2s.push(text);
                }
            }
        }
    }
    h2s
}

/// Collect the H4 heading titles in document order. Used for the PRD,
/// whose requirement-entry "row labels" are the `#### <Aspect>` sub-headings.
fn collect_h4s<'a>(root: comrak::nodes::Node<'a>) -> Vec<String> {
    let mut h4s = Vec::new();
    for node in root.descendants() {
        if let NodeValue::Heading(h) = node.data().value {
            if h.level == 4 {
                let text = node.collect_text().trim().to_string();
                if !text.is_empty() {
                    h4s.push(text);
                }
            }
        }
    }
    h4s
}

/// Return the first-column labels of every data row in the first table whose
/// header row contains a cell matching *marker*.
fn table_row_labels<'a>(root: comrak::nodes::Node<'a>, marker: &str) -> Option<Vec<String>> {
    for node in root.descendants() {
        if !matches!(node.data().value, NodeValue::Table(_)) {
            continue;
        }
        let rows: Vec<_> = node.children().collect();
        if rows.is_empty() {
            continue;
        }
        let header = &rows[0];
        let header_cells: Vec<String> = header
            .children()
            .map(|c| c.collect_text().trim().to_string())
            .collect();
        if !header_cells.iter().any(|c| c.contains(marker)) {
            continue;
        }
        let mut labels: Vec<String> = Vec::new();
        for row in &rows[1..] {
            let cell = row.children().next();
            let label = cell.map(|c| c.collect_text().trim().to_string());
            if let Some(label) = label {
                if !label.is_empty() {
                    labels.push(label);
                }
            }
        }
        if !labels.is_empty() {
            return Some(labels);
        }
    }
    None
}

/// Return the header cells of the first table whose header row contains a
/// cell matching *marker*.
fn table_header_columns<'a>(root: comrak::nodes::Node<'a>, marker: &str) -> Option<Vec<String>> {
    for node in root.descendants() {
        if !matches!(node.data().value, NodeValue::Table(_)) {
            continue;
        }
        let rows: Vec<_> = node.children().collect();
        if rows.is_empty() {
            continue;
        }
        let header = &rows[0];
        let header_cells: Vec<String> = header
            .children()
            .map(|c| c.collect_text().trim().to_string())
            .collect();
        if header_cells.iter().any(|c| c.contains(marker)) {
            return Some(header_cells.into_iter().filter(|c| !c.is_empty()).collect());
        }
    }
    None
}

/// Append a `pub const <NAME>: &[&str] = &[...];` line to the emitted
/// module source, returning the updated source.
fn push_const(out: String, name: &str, values: &[String]) -> String {
    let items: Vec<String> = values.iter().map(|v| format!("{v:?}")).collect();
    let body = if items.is_empty() {
        "&[]".to_string()
    } else {
        format!("&[{}]", items.join(", "))
    };
    format!("{out}\npub const {name}: &[&str] = {body};\n")
}

/// Append a `pub const <NAME>: &[&str] = <ALIAS>;` line to the emitted
/// module source, returning the updated source.
fn push_alias(out: String, name: &str, alias: &str) -> String {
    format!("{out}\npub const {name}: &[&str] = {alias};\n")
}

/// Per-doc extractor: walk the parsed template body and emit the
/// shape constants for that doc.
fn extract_doc_constants(doc: &str, raw: &str, mut out: String) -> String {
    let body = extract_fenced_markdown(raw);
    let arena = Arena::new();
    let opts = parse_options();
    let root = parse_document(&arena, &body, &opts);

    let h2s = collect_h2s(root);

    match doc {
        "agents" => {
            out = push_const(out, "AGENTS_REQUIRED_H2S", &h2s);
            // The full canonical template body, line-by-line, so the AES601
            // line-drift check tracks the reference template automatically.
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "AGENTS_TEMPLATE_LINES", &template_lines);
            if let Some(labels) = table_row_labels(root, "Aspect") {
                out = push_const(out, "AGENTS_RUNTIME_ROW_LABELS", &labels);
            }
            if let Some(cols) = table_header_columns(root, "Category") {
                out = push_const(out, "AGENTS_COMMAND_COLUMNS", &cols);
            }
            // The Project Structure section carries a fenced `text` tree block
            // rather than a `Path | Purpose` table, so no column constant is
            // generated for it. The tree shape is enforced by the line-drift
            // check above against `AGENTS_TEMPLATE_LINES`.
        }
        "architecture" => {
            out = push_const(out, "ARCHITECTURE_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "ARCHITECTURE_TEMPLATE_LINES", &template_lines);
        }
        "backlog" => {
            out = push_const(out, "BACKLOG_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "BACKLOG_TEMPLATE_LINES", &template_lines);
            if let Some(labels) = table_row_labels(root, "Aspect") {
                out = push_const(out, "BACKLOG_ROW_LABELS", &labels);
            }
        }
        "contributing" => {
            out = push_const(out, "CONTRIBUTING_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "CONTRIBUTING_TEMPLATE_LINES", &template_lines);
            if let Some(cols) = table_header_columns(root, "Step") {
                out = push_const(out, "CONTRIBUTING_WORKFLOW_COLUMNS", &cols);
            }
            if let Some(cols) = table_header_columns(root, "Type") {
                out = push_const(out, "CONTRIBUTING_COMMIT_COLUMNS", &cols);
            }
        }
        "data" => {
            out = push_const(out, "DATA_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "DATA_TEMPLATE_LINES", &template_lines);
            if let Some(cols) = table_header_columns(root, "Type") {
                out = push_const(out, "DATA_TYPE_DESCRIPTION_COLUMNS", &cols);
            }
        }
        "design" => {
            out = push_const(out, "DESIGN_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "DESIGN_TEMPLATE_LINES", &template_lines);
            if let Some(cols) = table_header_columns(root, "Variant") {
                out = push_const(out, "DESIGN_VARIANT_VALUE_COLUMNS", &cols);
            }
        }
        "frd" => {
            out = push_const(out, "FRD_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "FRD_TEMPLATE_LINES", &template_lines);
            out = push_alias(out, "FRD_SECTION_ORDER", "FRD_REQUIRED_H2S");
            if let Some(labels) = table_row_labels(root, "Aspect") {
                out = push_const(out, "FR_FIELDS", &labels);
            }
            out = push_alias(out, "FRD_ROW_LABELS", "FR_FIELDS");
            if let Some(cols) = table_header_columns(root, "Aspect") {
                out = push_const(out, "ASPECT_DETAIL_COLUMNS", &cols);
            }
            if let Some(cols) = table_header_columns(root, "Artifact") {
                out = push_const(out, "REFERENCE_COLUMNS", &cols);
            }
        }
        "prd" => {
            out = push_const(out, "PRD_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "PRD_TEMPLATE_LINES", &template_lines);
            if let Some(cols) = table_header_columns(root, "As a") {
                out = push_const(out, "PRD_USER_STORIES_COLUMNS", &cols);
            }
            let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
            let mut labels = Vec::new();
            for l in collect_h4s(root) {
                if seen.insert(l.clone()) {
                    labels.push(l);
                }
            }
            out = push_const(out, "PRD_ROW_LABELS", &labels);
        }
        "readme" => {
            out = push_const(out, "README_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "README_TEMPLATE_LINES", &template_lines);
            if let Some(cols) = table_header_columns(root, "Step") {
                out = push_const(out, "README_QUICK_START_COLUMNS", &cols);
            }
        }
        "roadmap" => {
            out = push_const(out, "ROADMAP_REQUIRED_H2S", &h2s);
            let template_lines: Vec<String> = body.lines().map(String::from).collect();
            out = push_const(out, "ROADMAP_TEMPLATE_LINES", &template_lines);
            if let Some(labels) = table_row_labels(root, "Aspect") {
                out = push_const(out, "ROADMAP_ROW_LABELS", &labels);
            }
        }
        _ => {
            unreachable!("unknown doc: {doc}");
        }
    }

    out
}
