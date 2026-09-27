// Unit tests — shared/maintenance taxonomy types.
use shared_lint_arwaky::maintenance::{
    MaintenanceRequest, MaintenanceResponse, SelfUpdateResultVO,
};

// ── SelfUpdateResultVO ─────────────────────────────────────
#[test]
fn self_update_success_marks_up_to_date() {
    let vo = SelfUpdateResultVO::success("3.7.0", "v3.7.0", true);
    assert_eq!(vo.current_version, "3.7.0");
    assert_eq!(vo.latest_version, "v3.7.0");
    assert!(vo.already_up_to_date);
    assert!(!vo.upgraded);
    assert_eq!(vo.status, "Already up to date (v3.7.0)");
}

#[test]
fn self_update_success_marks_upgraded() {
    let vo = SelfUpdateResultVO::success("3.7.0", "v3.8.0", false);
    assert_eq!(vo.current_version, "3.7.0");
    assert_eq!(vo.latest_version, "v3.8.0");
    assert!(!vo.already_up_to_date);
    assert!(vo.upgraded);
    assert_eq!(vo.status, "Upgraded from 3.7.0 to v3.8.0");
}

#[test]
fn self_update_error_clears_latest_version() {
    let vo = SelfUpdateResultVO::error("3.7.0", "network unreachable");
    assert_eq!(vo.current_version, "3.7.0");
    assert!(vo.latest_version.is_empty());
    assert!(!vo.already_up_to_date);
    assert!(!vo.upgraded);
    assert_eq!(vo.status, "Error: network unreachable");
}

#[test]
fn self_update_display_matches_status() {
    let vo = SelfUpdateResultVO::error("3.7.0", "boom");
    assert_eq!(vo.to_string(), vo.status);
}

// ── Request / response round-trip ───────────────────────────
#[test]
fn self_update_request_carries_check_only_flag() {
    let check = MaintenanceRequest::self_update(true);
    let install = MaintenanceRequest::self_update(false);
    assert!(matches!(
        check,
        MaintenanceRequest::SelfUpdate { check_only: true }
    ));
    assert!(matches!(
        install,
        MaintenanceRequest::SelfUpdate { check_only: false }
    ));
}

#[test]
fn self_update_response_extracts_result() {
    let vo = SelfUpdateResultVO::success("3.7.0", "v3.8.0", false);
    let response = MaintenanceResponse::SelfUpdate { result: vo };
    let extracted = response.into_self_update();
    assert_eq!(extracted.current_version, "3.7.0");
    assert_eq!(extracted.latest_version, "v3.8.0");
    assert!(extracted.upgraded);
}

#[test]
fn into_self_update_on_wrong_variant_returns_error() {
    let response = MaintenanceResponse::Update;
    let extracted = response.into_self_update();
    assert!(extracted.latest_version.is_empty());
    assert!(extracted.status.starts_with("Error:"));
}

#[test]
fn into_ran_excludes_self_update() {
    assert!(
        !MaintenanceResponse::SelfUpdate {
            result: SelfUpdateResultVO::success("3.7.0", "v3.7.0", true)
        }
        .into_ran()
    );
    assert!(MaintenanceResponse::Update.into_ran());
}
