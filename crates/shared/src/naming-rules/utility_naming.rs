// PURPOSE: NamingUtils — shared stem/suffix extraction for naming checkers

/// Extract the file stem using the last dot (rfind), consistent across all checkers.
///
/// For multi-dot filenames like `foo.spec.rs`, this returns `foo.spec`.
/// For single-dot files like `checker.rs`, this returns `checker`.
/// For dotfiles like `.gitignore`, the entire filename is returned.
/// If there is no dot, the entire filename is returned.
pub fn get_stem(filename: &str) -> Option<&str> {
    match filename.rfind('.') {
        Some(pos) if pos > 0 => Some(&filename[..pos]),
        _ => Some(filename),
    }
}

/// Extract the suffix (word after the last underscore) from a stem.
pub fn get_suffix(stem: &str) -> Option<&str> {
    stem.rfind('_').map(|pos| &stem[pos + 1..])
}
