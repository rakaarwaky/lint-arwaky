// build.rs — copy config YAML files from config/ into OUT_DIR at build time
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = match std::env::var("OUT_DIR") {
        Ok(v) => v,
        Err(_) => return,
    };
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(v) => v,
        Err(_) => return,
    };

    // Config files are in workspace root (3 levels up from crates/shared/)
    let config_dir = Path::new(&manifest_dir).join("../../../..");

    for name in &[
        "lint_arwaky.config.rust.yaml",
        "lint_arwaky.config.python.yaml",
        "lint_arwaky.config.javascript.yaml",
    ] {
        let src = config_dir.join(name);
        let dst = Path::new(&out_dir).join(name);
        if src.exists() {
            let _ = fs::copy(&src, &dst);
        }
    }

    // Re-run if config files change
    println!("cargo:rerun-if-changed=lint_arwaky.config.rust.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.python.yaml");
    println!("cargo:rerun-if-changed=lint_arwaky.config.javascript.yaml");
}
