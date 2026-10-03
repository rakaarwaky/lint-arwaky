//! Issue #560 (UX-3-03): assert `Modifier::BOLD` only appears in
//! `utility_tui_theme.rs`, never as a raw literal in TUI surface call sites.
use std::path::Path;

fn read_src_tree(root: &Path) -> Vec<(String, String)> {
    fn walk(dir: &Path, out: &mut Vec<(String, String)>) {
        let entries = std::fs::read_dir(dir)
            .expect("read_dir failed")
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let content = std::fs::read_to_string(&path)
                    .expect("read_to_string failed")
                    .to_string();
                out.push((path.to_string_lossy().to_string(), content));
            }
        }
    }
    let mut files = Vec::new();
    walk(root, &mut files);
    files
}

#[test]
fn modifier_bold_literal_only_in_theme_module() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tui_src = manifest.join("src");
    let theme_src = manifest
        .join("../shared/src/tui/utility_tui_theme.rs")
        .canonicalize()
        .expect("locate utility_tui_theme.rs");
    let files = read_src_tree(&tui_src);
    for (name, content) in &files {
        if content.contains("Modifier::BOLD") {
            panic!(
                "Modifier::BOLD literal found in {name}. It must be defined \
                 only in utility_tui_theme.rs as EMPHASIS_SELECTED / EMPHASIS_HEADING."
            );
        }
    }

    let theme_content = std::fs::read_to_string(&theme_src).expect("read utility_tui_theme.rs");
    assert!(
        theme_content.contains("EMPHASIS_SELECTED"),
        "EMPHASIS_SELECTED token missing from utility_tui_theme.rs"
    );
    assert!(
        theme_content.contains("EMPHASIS_HEADING"),
        "EMPHASIS_HEADING token missing from utility_tui_theme.rs"
    );
}
