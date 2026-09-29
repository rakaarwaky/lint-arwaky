// PURPOSE: build script — stage publishable assets into OUT_DIR
//
// Both asset groups must resolve for the workspace build *and* for the isolated
// build of a published `.crate` tarball (crates.io extracts the package to an
// isolated directory, so any path above the crate root is missing there):
//
//   1. Config YAML — committed at `<manifest>/config/`, in-package already.
//   2. Skill markdown — committed at `<manifest>/skills/`, in-package already.
//
// `cargo package` stages `<manifest>/skills/` into the tarball via the
// `[package] include` list in Cargo.toml, so `include_str!(concat!(env!("OUT_DIR"),
// "/skills/..."))` stays valid for a consumer that builds straight from the
// registry.
use std::fs;
use std::path::Path;

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

    stage_config(Path::new(&manifest_dir), Path::new(&out_dir));
    stage_skills(Path::new(&manifest_dir), Path::new(&out_dir));
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

    println!("cargo:rerun-if-changed=config/lint_arwaky.config.yaml");
}

/// Recursively copy skills source into `OUT_DIR/skills/` so the
/// `include_str!` sites in `taxonomy_project_setup_constant.rs` resolve at compile
/// time.
///
/// In a workspace build the source of truth is
/// `../../crates/skills/` (from the shared crate root, two levels up gets to
/// the workspace root, then into `crates/skills/`). In a published tarball
/// build the source is `<manifest>/skills/` (staged there by the preceding
/// workspace build and packaged via `[package] include`).
///
/// Neither case is fatal: a binary without embedded skills still compiles;
/// init-time skill installation simply yields an empty catalog.
fn stage_skills(manifest_dir: &Path, out_dir: &Path) {
    let skills_src = if manifest_dir.join("../../crates/skills").is_dir() {
        manifest_dir.join("../../crates/skills")
    } else if manifest_dir.join("skills").is_dir() {
        manifest_dir.join("skills")
    } else {
        eprintln!(
            "Skills directory not found. Skipping embedding: neither \
             ../../crates/skills/ nor skills/ exists under {}",
            manifest_dir.display()
        );
        println!("cargo:warning=skills directory missing — EMBEDDED_SKILLS will be empty");
        return;
    };

    let skills_dst = out_dir.join("skills");
    if let Err(e) = fs::create_dir_all(&skills_dst) {
        eprintln!("Failed to create {}: {e}", skills_dst.display());
        std::process::exit(1);
    }
    copy_dir_recursive(&skills_src, &skills_dst);

    println!("cargo:rerun-if-changed=../../crates/skills");
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
