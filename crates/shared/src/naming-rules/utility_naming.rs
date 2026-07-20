// PURPOSE: NamingUtils — shared stem/suffix extraction for naming checkers

/// Extract the file stem using the last dot (rfind), consistent across all checkers.
///
/// For multi-dot filenames like `foo.spec.rs`, this returns `foo.spec`.
/// For single-dot files like `checker.rs`, this returns `checker`.
/// If there is no dot, the entire filename is returned.
pub fn get_stem(filename: &str) -> Option<&str> {
    if let Some(pos) = filename.rfind('.') {
        Some(&filename[..pos])
    } else {
        Some(filename)
    }
}

/// Extract the suffix (word after the last underscore) from a stem.
pub fn get_suffix(stem: &str) -> Option<&str> {
    stem.rfind('_').map(|pos| &stem[pos + 1..])
}

