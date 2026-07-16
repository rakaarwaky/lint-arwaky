// build.rs — copy config YAML files from workspace root into OUT_DIR at build time
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // CARGO_MANIFEST_DIR = .../lint-arwaky/crates/shared
    // Workspace root = 2 parent levels up (shared → crates → lint-arwaky)
    let workspace_root = Path::new(&manifest_dir)
        .ancestors()
        .nth(2)
        .expect("cannot reach workspace root from manifest dir");

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

        fs::copy(&src, &dst).expect("Failed to copy config file");
    }

    // Re-run on config changes
    println!("cargo:rerun-if-changed=lint_arwaky.config.rust.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.python.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.javascript.yaml");
}
