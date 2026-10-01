// Unit tests for FR-Filesystem-005: project language detection
// (`detect_project_languages` / `ProjectLanguagesVO`).
//
// The walk is a raw recursion rather than a `WalkBuilder` walk, so it needs its
// own guard tests: a directory symlink inside the project must not let files
// from outside the root decide the answer, and a symlink cycle must terminate.
use shared_filesystem::taxonomy_filesystem_vo::ProjectLanguagesVO;
use shared_filesystem::utility_workspace_detection::detect_project_languages;
use std::fs;
use tempfile::TempDir;

fn write(path: &std::path::Path, name: &str, body: &str) {
    fs::write(path.join(name), body).unwrap();
}

#[test]
fn detects_each_language_group() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "main.rs", "fn main() {}");
    write(tmp.path(), "app.py", "x = 1");
    write(tmp.path(), "index.ts", "export const x = 1;");
    write(tmp.path(), "README.md", "# hi");
    let flags = detect_project_languages(tmp.path());
    assert!(flags.has_rust && flags.has_python && flags.has_js && flags.has_markdown);
}

#[test]
fn detects_nested_files() {
    let tmp = TempDir::new().unwrap();
    let deep = tmp.path().join("crates").join("a").join("src");
    fs::create_dir_all(&deep).unwrap();
    write(&deep, "lib.rs", "");
    assert!(detect_project_languages(tmp.path()).has_rust);
}

#[test]
fn single_file_root_is_classified() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "only.rs", "");
    let flags = detect_project_languages(&tmp.path().join("only.rs"));
    assert!(
        flags.has_rust,
        "a file root classifies by its own extension"
    );
    assert!(!flags.has_python);
}

#[test]
fn empty_project_reports_no_languages() {
    let tmp = TempDir::new().unwrap();
    assert!(detect_project_languages(tmp.path()).is_empty());
}

#[test]
fn ignored_directories_are_skipped() {
    // `target/` is in DEFAULT_IGNORED_PATHS: a build artifact tree must not
    // make a project look like it has the languages it happens to contain.
    let tmp = TempDir::new().unwrap();
    let target = tmp.path().join("target");
    fs::create_dir_all(&target).unwrap();
    write(&target, "generated.rs", "");
    write(&target, "generated.py", "");
    let flags = detect_project_languages(tmp.path());
    assert!(
        flags.is_empty(),
        "files under an ignored directory must not set a language flag"
    );
}

#[cfg(unix)]
#[test]
fn symlinked_directory_outside_root_is_not_classified() {
    // A symlink inside the project pointing at a directory elsewhere must not
    // decide the project's languages — otherwise a link to, say, a sibling
    // checkout silently reports languages the project does not contain.
    let outer = TempDir::new().unwrap();
    let outside = outer.path().join("outside");
    fs::create_dir_all(&outside).unwrap();
    write(&outside, "leak.rs", "fn leak() {}");
    write(&outside, "leak.md", "# leak");

    let project = TempDir::new().unwrap();
    std::os::unix::fs::symlink(&outside, project.path().join("escape")).unwrap();

    let flags = detect_project_languages(project.path());
    assert!(
        flags.is_empty(),
        "a symlink must not pull files from outside the root into the flags: {flags:?}"
    );
}

#[cfg(unix)]
#[test]
fn symlink_cycle_terminates() {
    // `loop -> .` makes the tree infinitely deep if the walk follows symlinks.
    // This test asserts termination by finishing at all; a cycle would recurse
    // until the stack is exhausted.
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "main.rs", "");
    std::os::unix::fs::symlink(tmp.path(), tmp.path().join("loop")).unwrap();
    let flags = detect_project_languages(tmp.path());
    assert!(flags.has_rust, "the real file is still found");
}

#[cfg(unix)]
#[test]
fn symlinked_file_inside_root_is_not_classified() {
    let outer = TempDir::new().unwrap();
    write(outer.path(), "leak.py", "");
    let project = TempDir::new().unwrap();
    std::os::unix::fs::symlink(outer.path().join("leak.py"), project.path().join("leak.py"))
        .unwrap();
    let flags = detect_project_languages(project.path());
    assert!(
        !flags.has_python,
        "a symlinked file must not set a language flag: {flags:?}"
    );
}

#[test]
fn project_languages_vo_default_is_empty() {
    let flags = ProjectLanguagesVO::default();
    assert!(flags.is_empty());
    assert!(!flags.has_rust && !flags.has_python && !flags.has_js && !flags.has_markdown);
}

#[test]
fn project_languages_vo_is_empty_tracks_any_flag() {
    let mut flags = ProjectLanguagesVO::default();
    assert!(flags.is_empty());
    flags.has_markdown = true;
    assert!(
        !flags.is_empty(),
        "any single flag set means the project is not empty"
    );
}
