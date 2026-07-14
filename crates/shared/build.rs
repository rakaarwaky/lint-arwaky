// build.rs — copy config YAML files from project root into OUT_DIR at build time
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Config files are at project root (4 levels up from crates/shared/)
    let root = Path::new(&manifest_dir).join("../../..");

    for name in &[
        "lint_arwaky.config.rust.yaml",
        "lint_arwaky.config.python.yaml",
        "lint_arwaky.config.javascript.yaml",
    ] {
        let src = root.join(name);
        let dst = Path::new(&out_dir).join(name);
        if src.exists() {
            fs::copy(&src, &dst).expect(&format!("failed to copy {}", name));
        }
    }

    // Re-run if config files change
    println!("cargo:rerun-if-changed=../../../lint_arwaky.config.rust.yaml");
    println!("cargo:rerun-if-changed=../../../lint_arwaky.config.python.yaml");
    println!("cargo:rerun-if-changed=../../../lint_arwaky.config.javascript.yaml");
}
