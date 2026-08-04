// build.rs — copy config YAML files from crate-local config/ dir into OUT_DIR
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

    // Config files live in <manifest>/config/ (within the crate, safe for crates.io)
    let config_dir = Path::new(&manifest_dir).join("config");

    let name = "lint_arwaky.config.yaml";
    let src = config_dir.join(name);
    let dst = Path::new(&out_dir).join(name);

    if !src.exists() {
        panic!(
            "Config file not found at {:?}. Check that config/ is in the crate root.",
            src
        );
    }

    if let Err(e) = fs::copy(&src, &dst) {
        panic!("Failed to copy config file {name}: {e}");
    }

    println!("cargo:rerun-if-changed=config/lint_arwaky.config.yaml");
}
