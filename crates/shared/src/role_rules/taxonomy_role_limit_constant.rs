// PURPOSE: role-rules thresholds — shared numeric limits for surface role checks (AES406)
// Used by: role-rules (SurfaceRoleChecker)

/// Maximum public methods a passive/utility surface may expose.
pub const MAX_PUBLIC_METHODS: usize = 50;

/// Maximum control flow statements tolerated in a passive/utility surface.
pub const MAX_CONTROL_FLOW: usize = 50;
