// PURPOSE: str_or — stateless utility to return a fallback string when an Option is None

/// Return the provided string if Some, otherwise return the fallback.
///
/// Common helper for gracefully providing default string values without
/// duplicating `opt.map_or(fallback, |s| s)` throughout the codebase.
pub fn str_or<'a>(opt: Option<&'a str>, fallback: &'a str) -> &'a str {
    opt.map_or(fallback, |s| s)
}
