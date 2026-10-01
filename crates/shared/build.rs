// PURPOSE: build script — stage publishable assets into OUT_DIR
//
// Both asset groups must resolve for the workspace build *and* for the isolated
// build of a published `.crate` tarball (crates.io extracts the package to an
// isolated directory, so any path above the crate root is missing there):
//
//   1. Config YAML — committed at `crates/shared/config/`.
//   2. Skill markdown — committed at `crates/shared/skills/`.
//
// The script now lives at `crates/shared/build.rs` but is invoked by the
// `project-setup` package nested two levels below (`crates/shared/src/
// project_setup/`), so `CARGO_MANIFEST_DIR` points there and the assets are
// reached through its `../../` ancestor. `resolve_assets` picks whichever of
// the two layouts actually carries them.
use std::fs;
use std::path::{Path, PathBuf};

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

    let assets = resolve_assets(Path::new(&manifest_dir));
    stage_config(&assets, Path::new(&out_dir));
    stage_skills(&assets, Path::new(&out_dir));
}

/// Locate the directory that carries both `config/` and `skills/`.
///
/// Historically that was the manifest dir itself; since the monolith was split
/// into per-folder packages the manifest sits two levels deeper, so fall back
/// to the `../../` ancestor.
fn resolve_assets(manifest_dir: &Path) -> PathBuf {
    for candidate in [manifest_dir.to_path_buf(), manifest_dir.join("../..")] {
        if candidate.join("config").is_dir() && candidate.join("skills").is_dir() {
            return candidate;
        }
    }
    eprintln!(
        "Assets not found: neither {} nor {} carries config/ and skills/",
        manifest_dir.display(),
        manifest_dir.join("../..").display()
    );
    std::process::exit(1);
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
