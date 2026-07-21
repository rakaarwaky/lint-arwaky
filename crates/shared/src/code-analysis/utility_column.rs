// PURPOSE: Stateless utility functions for column position computation
// Pure functions only — no struct, no &self, no I/O

/// Compute 1-indexed column position of `pattern` in `line`.
/// Returns 0 if pattern not found.
pub fn compute_column(line: &str, pattern: &str) -> usize {
    line.find(pattern).map(|pos| pos + 1).unwrap_or(0)
}

/// Compute 1-indexed column position of a byte offset in a line.
pub fn byte_offset_to_column(line: &str, offset: usize) -> usize {
    line[..offset.min(line.len())].chars().count() + 1
}
