//! Contract tests for the `report` command's arithmetic and key derivation.
//!
//! The delta math has no I/O, so it is tested directly rather than through the
//! CLI: a test that shells out to the binary would need a scanned workspace and
//! a snapshot directory, and would fail for reasons unrelated to the arithmetic
//! it meant to check.

use shared_cli_commands::ReportSnapshot;
use shared_cli_commands::utility_report_delta::compare;
use shared_cli_commands::utility_report_snapshot::target_key;
use std::collections::BTreeMap;

fn snapshot(members: &[(&str, usize)]) -> ReportSnapshot {
    ReportSnapshot {
        taken_at: 1_700_000_000,
        members: members
            .iter()
            .map(|(m, c)| (m.to_string(), *c))
            .collect::<BTreeMap<_, _>>(),
    }
}

#[test]
fn no_baseline_leaves_every_change_unknown() {
    let delta = compare(&snapshot(&[("crates/shared", 12)]), None);
    assert_eq!(delta.total_now, 12);
    assert_eq!(delta.total_before, None);
    assert_eq!(delta.total_change(), None);
    assert_eq!(delta.since, None);
    assert!(delta.members[0].change().is_none());
}

#[test]
fn an_unchanged_member_shows_no_change() {
    let now = snapshot(&[("crates/shared", 12)]);
    let delta = compare(&now, Some(&now));
    assert_eq!(delta.total_change(), Some(0));
    assert_eq!(delta.members[0].change(), Some(0));
    assert_eq!(delta.fixed(), 0);
    assert_eq!(delta.introduced(), 0);
}

#[test]
fn a_drop_is_negative_and_counts_as_fixed() {
    let delta = compare(
        &snapshot(&[("crates/shared", 5)]),
        Some(&snapshot(&[("crates/shared", 12)])),
    );
    assert_eq!(delta.members[0].change(), Some(-7));
    assert_eq!(delta.total_change(), Some(-7));
    assert_eq!(delta.fixed(), 7);
    assert_eq!(delta.introduced(), 0);
}

#[test]
fn a_rise_is_positive_and_counts_as_introduced() {
    let delta = compare(
        &snapshot(&[("crates/shared", 15)]),
        Some(&snapshot(&[("crates/shared", 12)])),
    );
    assert_eq!(delta.members[0].change(), Some(3));
    assert_eq!(delta.introduced(), 3);
    assert_eq!(delta.fixed(), 0);
}

#[test]
fn a_member_new_to_this_scan_is_measured_from_zero() {
    // A member absent from the baseline has a count but no history, so its
    // change is the whole count — not unknown. Reporting it as unknown would
    // hide a member that appeared in the workspace.
    let delta = compare(
        &snapshot(&[("crates/shared", 12), ("crates/fresh", 4)]),
        Some(&snapshot(&[("crates/shared", 12)])),
    );
    let fresh = delta
        .members
        .iter()
        .find(|m| m.member == "crates/fresh")
        .expect("a new member must appear in the report");
    assert_eq!(fresh.change(), Some(4));
    assert_eq!(delta.introduced(), 4);
}

#[test]
fn a_member_that_cleared_still_appears() {
    // Dropping it would hide the improvement that just happened.
    let delta = compare(
        &snapshot(&[("crates/shared", 3)]),
        Some(&snapshot(&[("crates/shared", 12), ("crates/cleared", 9)])),
    );
    let cleared = delta
        .members
        .iter()
        .find(|m| m.member == "crates/cleared")
        .expect("a cleared member must still be reported");
    assert_eq!(cleared.now, 0);
    assert_eq!(cleared.change(), Some(-9));
}

#[test]
fn members_are_ordered_heaviest_first() {
    let delta = compare(&snapshot(&[("a", 3), ("b", 30), ("c", 12)]), None);
    let order: Vec<&str> = delta.members.iter().map(|m| m.member.as_str()).collect();
    assert_eq!(order, vec!["b", "c", "a"]);
}

#[test]
fn an_empty_baseline_is_treated_as_no_baseline() {
    // A store written by a run that found nothing has no members. Comparing
    // against it would report "everything appeared", which misreads a first run.
    let delta = compare(&snapshot(&[("a", 4)]), Some(&ReportSnapshot::default()));
    assert_eq!(delta.total_before, None);
    assert_eq!(delta.total_change(), None);
}

#[test]
fn totals_sum_every_member() {
    let delta = compare(&snapshot(&[("a", 4), ("b", 6), ("c", 10)]), None);
    assert_eq!(delta.total_now, 20);
}

#[test]
fn two_targets_derive_different_keys() {
    // Both targets live in the same snapshot file, so a shared key would make
    // one target's baseline overwrite the other's.
    assert_ne!(target_key("."), target_key("crates"));
    assert_ne!(target_key("crates"), target_key("crates/shared"));
}

#[test]
fn the_same_target_always_derives_the_same_key() {
    // The key is derived at read time as well as at write time. A derivation
    // that depended on anything but the target would make every report a
    // first run — which is the bug this file's first version could not catch.
    assert_eq!(target_key("crates/shared"), target_key("crates/shared"));
}

#[test]
fn a_key_carries_only_characters_usable_in_json_and_paths() {
    for target in [".", "/home/me/proj", "crates/shared", "a b/c\\d"] {
        let key = target_key(target);
        assert!(
            key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'),
            "key for {target:?} has an unusable character: {key:?}"
        );
        assert!(!key.is_empty(), "key for {target:?} is empty");
    }
}

#[test]
fn a_snapshot_survives_a_write_and_read_round_trip() {
    // The store's on-disk shape is its own contract. Writing the map while
    // reading the struct produced a file that never parsed, and every report
    // then claimed it had no baseline without saying why. This writes through
    // the real writer and reads through the real reader.
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("snapshots.json");

    let written = ReportSnapshot {
        taken_at: 1_700_000_000,
        members: BTreeMap::from([("crates/shared".to_string(), 12usize)]),
    };
    shared_cli_commands::utility_report_snapshot::write_store_at(&path, "crates", written.clone())
        .expect("write_store");

    let store = shared_cli_commands::utility_report_snapshot::read_store_at(&path);
    let read = store
        .entries
        .get(&target_key("crates"))
        .expect("entry written");
    assert_eq!(read, &written);
}

#[test]
fn writing_a_second_target_keeps_the_first() {
    // Both targets share one file, so a writer that replaced the whole store
    // would leave the other target with no baseline on its next run.
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("snapshots.json");
    let snap = |n: usize| ReportSnapshot {
        taken_at: 1_700_000_000,
        members: BTreeMap::from([("crates/shared".to_string(), n)]),
    };

    shared_cli_commands::utility_report_snapshot::write_store_at(&path, "crates", snap(12))
        .expect("first write");
    shared_cli_commands::utility_report_snapshot::write_store_at(&path, "packages", snap(3))
        .expect("second write");

    let store = shared_cli_commands::utility_report_snapshot::read_store_at(&path);
    assert_eq!(
        store
            .entries
            .get(&target_key("crates"))
            .expect("first target survives")
            .members
            .get("crates/shared"),
        Some(&12)
    );
    assert_eq!(
        store
            .entries
            .get(&target_key("packages"))
            .expect("second target present")
            .members
            .get("crates/shared"),
        Some(&3)
    );
}

#[test]
fn an_absent_file_reads_as_an_empty_store() {
    let dir = tempfile::tempdir().expect("temp dir");
    let store = shared_cli_commands::utility_report_snapshot::read_store_at(
        &dir.path().join("nothing-here.json"),
    );
    assert!(store.entries.is_empty());
}
