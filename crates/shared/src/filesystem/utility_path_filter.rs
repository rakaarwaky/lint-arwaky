// Utility: stateless path filtering shared by all feature crates.
// AES404 allows utility files to import the taxonomy/shared layer.
//
// Extracted from filesystem::utility_filesystem_io and
// filesystem::utility_workspace_detection to eliminate AES305 duplication.

/// Path segments split on `/` or `\`.
fn path_segments(rel_path: &str) -> Vec<&str> {
    rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect()
}

/// Check if any segment matches a dot-prefixed literal (e.g. `.git`, `.env`).
fn matches_dot_literal(segments: &[&str], pattern: &str) -> bool {
    segments.contains(&pattern)
}

/// Check prefix pattern: `/dir/**` matches path starting with those segments.
fn matches_prefix_pattern(segments: &[&str], pattern: &str) -> bool {
    let stripped = match pattern.strip_prefix('/') {
        Some(s) if !s.is_empty() => s,
        _ => return false,
    };
    let pat_segs: Vec<&str> = stripped
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    let n_pat = pat_segs.len();
    let n_seg = segments.len();
    if n_seg < n_pat {
        return false;
    }
    for start in 0..=(n_seg - n_pat) {
        if segments[start..start + n_pat] == pat_segs[..] {
            return true;
        }
    }
    false
}

/// Check `**/*.ext` — recursive glob matching extension on last segment.
fn matches_recursive_ext(segments: &[&str], pattern: &str) -> bool {
    let suffix = match pattern.strip_prefix("**/") {
        Some(s) => s,
        None => return false,
    };
    let ext = match suffix.strip_prefix("*.") {
        Some(e) if !e.is_empty() => e.trim_start_matches('.'),
        _ => return false,
    };
    let basename = segments.last().copied().unwrap_or("");
    basename.ends_with(&format!(".{ext}"))
}

/// Check `prefix/*` — single-level wildcard matches first segment.
fn matches_single_level_wildcard(segments: &[&str], pattern: &str) -> bool {
    let prefix = match pattern.strip_suffix("/*") {
        Some(p) if !p.is_empty() => p,
        _ => return false,
    };
    segments.first() == Some(&prefix)
}

/// Check `*.ext` — extension match on last segment.
fn matches_ext_pattern(segments: &[&str], pattern: &str) -> bool {
    let suffix = match pattern.strip_prefix("*.") {
        Some(s) if !s.is_empty() => s.trim_start_matches('.'),
        _ => return false,
    };
    let basename = segments.last().copied().unwrap_or("");
    basename.ends_with(&format!(".{suffix}"))
}

/// Check literal path match: single segment or multi-segment prefix/substring.
fn matches_literal(segments: &[&str], pattern: &str) -> bool {
    let pat_segs: Vec<&str> = pattern
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    match pat_segs.len() {
        1 => segments.contains(&pat_segs[0]),
        n if n > 1 => {
            let n_seg = segments.len();
            n_seg >= n && {
                for start in 0..=(n_seg - n) {
                    if segments[start..start + n] == pat_segs[..] {
                        return true;
                    }
                }
                false
            }
        }
        _ => false,
    }
}

/// Return true if rel_path should be skipped based on ignored patterns.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    let segments = path_segments(rel_path);
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        // Pattern types checked in priority order:
        if pat.starts_with('/') && !pat.starts_with("**/") {
            // Prefix pattern: /dir/** or /dir/sub
            if matches_prefix_pattern(&segments, pat) {
                return true;
            }
        } else if pat.starts_with("**/") {
            // Recursive glob: **/*.ext
            if matches_recursive_ext(&segments, pat) {
                return true;
            }
        } else if pat.ends_with("/*") {
            // Single-level wildcard: dir/*
            if matches_single_level_wildcard(&segments, pat) {
                return true;
            }
        } else if pat.starts_with("*.") {
            // Extension pattern: *.ext
            if matches_ext_pattern(&segments, pat) {
                return true;
            }
        } else if pat.starts_with('.') {
            // Dot-prefixed literal: .env, .git
            if matches_dot_literal(&segments, pat) {
                return true;
            }
        } else {
            // Literal path match (single or multi-segment)
            if matches_literal(&segments, pat) {
                return true;
            }
        }
    }
    false
}
