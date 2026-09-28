// PURPOSE: role-rules thresholds — shared numeric limits for surface role checks (AES406)
// Used by: role-rules (SurfaceRustRoleAuditor, SurfacePythonRoleAuditor,
//          SurfaceTypeScriptRoleAuditor, utility_surface_role_checker)

/// Maximum public methods a passive/utility surface may expose.
pub const MAX_PUBLIC_METHODS: usize = 50;

/// Maximum control flow statements tolerated in a passive/utility surface.
pub const MAX_CONTROL_FLOW: usize = 50;

/// Maximum functions in a smart surface (`_command` / `_controller` / `_page`).
/// Smart surfaces orchestrate sub-commands, so this is generous.
pub const MAX_FN_COUNT_SMART: usize = 50;

/// Maximum functions in a utility surface (`_action` / `_store` / `_hook` /
/// `_screen` / `_router`). Utility surfaces are thin adapters.
pub const MAX_FN_COUNT_UTILITY: usize = 25;

/// Maximum functions in a passive surface (`_component` / `_view` / `_layout`).
/// Passive surfaces only render.
pub const MAX_FN_COUNT_PASSIVE: usize = 25;
