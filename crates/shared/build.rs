// build.rs — copy config YAML files from workspace root into OUT_DIR at build time
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = match std::env::var("OUT_DIR") {
        Ok(v) => v,
        Err(e) => panic!("OUT_DIR not set: {e}"),
    };
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(v) => v,
        Err(e) => panic!("CARGO_MANIFEST_DIR not set: {e}"),
    };

    // CARGO_MANIFEST_DIR = .../lint-arwaky/crates/shared
    // Workspace root = 2 parent levels up (shared → crates → lint-arwaky)
    let workspace_root = match Path::new(&manifest_dir).ancestors().nth(2) {
        Some(p) => p,
        None => panic!("cannot reach workspace root from manifest dir"),
    };

    for name in &[
        "lint_arwaky.config.rust.yaml",
        "lint_arwaky.config.python.yaml",
        "lint_arwaky.config.javascript.yaml",
    ] {
        let src = workspace_root.join(name);
        let dst = Path::new(&out_dir).join(name);

        if !src.exists() {
            panic!(
                "Config file not found at {:?}. Check that config files are in the workspace root.",
                src
            );
        }

        if let Err(e) = fs::copy(&src, &dst) {
            panic!("Failed to copy config file {name}: {e}");
        }
    }

    // Re-run on config changes
    println!("cargo:rerun-if-changed=lint_arwaky.config.rust.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.python.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.javascript.yaml");
}
