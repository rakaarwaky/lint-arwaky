// PURPOSE: Unit tests — per-skill provisioning semantics for `lint-arwaky init`.
//
// Verifies that `provision_skills` handles each skill directory independently:
//   - a provisioned skill missing from the target is created,
//   - a provisioned skill already present in the target is overwritten in place,
//   - a skill that exists only in the target is left untouched.
//
// Flat prefix naming (create-test-rust): unit_dispatcher_<subject>.rs
use dispatcher_lint_arwaky::surface_setup_action::{SkillProvision, provision_skills};
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use std::path::Path;
use std::sync::Arc;

fn filesystem() -> Arc<dyn IFileSystemIOProtocol> {
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

/// Write `<root>/<skill>/SKILL.md` with the given body.
fn write_skill(root: &Path, skill: &str, body: &str) {
    let dir = root.join(skill);
    std::fs::create_dir_all(&dir).expect("create skill dir");
    std::fs::write(dir.join("SKILL.md"), body).expect("write SKILL.md");
}

fn read_skill(root: &Path, skill: &str) -> String {
    std::fs::read_to_string(root.join(skill).join("SKILL.md")).expect("read SKILL.md")
}

/// Seed a source with two skills, a target with a stale skill plus a local
/// one, then provision. Counts the three buckets and asserts on content.
fn provision(source: &Path, target: &Path) -> (SkillProvision, String, String, String) {
    write_skill(source, "alpha", "alpha upstream");
    write_skill(source, "beta", "beta upstream");
    write_skill(target, "alpha", "STALE local alpha");
    write_skill(target, "local-only", "hand written");

    let fs = filesystem();
    let result = provision_skills(source, target, &*fs);
    let alpha = read_skill(target, "alpha");
    let beta = read_skill(target, "beta");
    let local = read_skill(target, "local-only");
    (result, alpha, beta, local)
}

#[test]
fn provision_overwrites_existing_creates_new_keeps_local() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let source = tmp.path().join("source");
    let target = tmp.path().join("target");
    std::fs::create_dir_all(&source).expect("create source");
    std::fs::create_dir_all(&target).expect("create target");

    let (result, alpha, beta, local) = provision(&source, &target);
    match result {
        SkillProvision::Provisioned {
            copied,
            overwritten,
            kept,
        } => {
            assert_eq!(copied, 1, "beta is new -> copied");
            assert_eq!(overwritten, 1, "alpha exists -> overwritten");
            assert_eq!(kept, 1, "local-only is untouched");
        }
        other => panic!("expected Provisioned, got {other:?}"),
    }
    assert_eq!(
        alpha, "alpha upstream",
        "stale alpha replaced with upstream"
    );
    assert_eq!(beta, "beta upstream", "new skill provisioned");
    assert_eq!(local, "hand written", "local-only skill left untouched");
}

#[test]
fn provision_missing_source_reports_source_missing() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let source = tmp.path().join("does-not-exist");
    let target = tmp.path().join("target");
    std::fs::create_dir_all(&target).expect("create target");

    let result = provision_skills(&source, &target, &*filesystem());
    assert!(matches!(result, SkillProvision::SourceMissing));
}

#[test]
fn provision_empty_source_reports_source_missing() {
    let tmp = tempfile::TempDir::new().expect("tempdir");
    let source = tmp.path().join("source");
    let target = tmp.path().join("target");
    std::fs::create_dir_all(&source).expect("create source");
    std::fs::create_dir_all(&target).expect("create target");

    let result = provision_skills(&source, &target, &*filesystem());
    assert!(matches!(result, SkillProvision::SourceMissing));
}
