use shared_quality_rules::contract_quality_protocol::ICodeMetricAnalyzerProtocol;
use shared_quality_rules::taxonomy_quality_rules_vo::AesCodeAnalysisViolation;

use shared_common::LintMessage;
use shared_config_system::ArchitectureConfig;
use std::sync::Arc;

// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Accept pre-fetched (path, content) entries from caller
//   2. For each file, tokenize content into lines
//   3. Slide a window of `min_lines` over lines; two keys: raw (identical) and normalized (similar)
//   4. Two global maps keyed on raw / normalized window text; store file indices
//   5. For each file, compute identical_pct (raw shared) and similar_pct (normalized shared)
//   6. identical_pct > identical_threshold → "identical" violation; else similar_pct > similar_threshold → "similar"
//   7. One violation per file with per-block line ranges (top 5, +N more)

use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CodeDuplicationAnalyzer {
    /// P1.6 fix: carry injected config instead of calling default_aes_config()
    config: Arc<ArchitectureConfig>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates_entries(
        &self,
        entries: &[(std::path::PathBuf, String)],
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let config = self.config.as_ref();
        let aes305 = config.rules.iter().find(|r| r.name.value == "AES305");
        let min_lines = aes305
            .map(|r| r.code_analysis.min_lines.value as usize)
            .filter(|&v| v > 0)
            .unwrap_or(10);

        // Three-tier fallback: explicit new field → duplication_threshold → built-in default
        let dup = aes305.and_then(|r| r.code_analysis.duplication_threshold);
        let identical_threshold = aes305
            .and_then(|r| r.code_analysis.identical_threshold)
            .or(dup)
            .unwrap_or(75.0);
        let similar_threshold = aes305
            .and_then(|r| r.code_analysis.similar_threshold)
            .or(dup)
            .unwrap_or(50.0);

        // Borrow path/content as &str instead of cloning into (String, String)
        let borrowed: Vec<(&str, &str)> = entries
            .iter()
            .map(|(p, c)| (p.to_str().unwrap_or_default(), c.as_str()))
            .collect();
        self.check_file_similarity_entries(
            &borrowed,
            min_lines,
            identical_threshold,
            similar_threshold,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CodeDuplicationAnalyzer {
    pub fn from_config(config: Arc<ArchitectureConfig>) -> Self {
        Self { config }
    }
}

/// Per-file window position record: 1-based start/end lines plus the set of
/// other files that share this window.
struct DupWindow {
    start_line: usize,
    end_line: usize,
    others: HashSet<usize>,
}

/// Collapse per-window positions into maximal consecutive line ranges.
/// 1-based, inclusive.
pub(crate) fn collapse_ranges(positions: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for &(s, e) in positions {
        if let Some(last) = ranges.last_mut() {
            if s <= last.1 + 1 {
                last.1 = e.max(last.1);
                continue;
            }
        }
        ranges.push((s, e));
    }
    ranges
}

fn is_import_only(w: &[&str]) -> bool {
    w.iter().all(|l| {
        let t = l.trim();
        !t.is_empty() && shared_quality_rules::utility_code_duplication_detector::is_import_line(t)
    })
}

/// Keys in `map` that appear in 2+ files. String keys make this collision-free.
fn shared_keys(map: &HashMap<String, HashSet<usize>>) -> HashSet<String> {
    map.iter()
        .filter(|(_, file_indices)| file_indices.len() > 1)
        .map(|(id, _)| id.clone())
        .collect()
}

/// Other files (excluding `fi`) that share `key` with the current file.
fn others_for(map: &HashMap<String, HashSet<usize>>, key: &str, fi: usize) -> HashSet<usize> {
    map.get(key)
        .map(|set| {
            set.iter()
                .copied()
                .filter(|&o| o != fi)
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
}

impl CodeDuplicationAnalyzer {
    /// File-level similarity analysis using pre-read entries.
    /// Instead of one violation per sliding-window match, calculates what % of a file's
    /// normalized windows also appear in other files. Only files exceeding the threshold
    /// are flagged — one violation per file.
    /// Returns (file_path, violation) tuples so the caller can attach the file path.
    pub fn check_file_similarity_entries(
        &self,
        entries: &[(&str, &str)],
        min_dup_lines: usize,
        identical_threshold_pct: f64,
        similar_threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        if entries.is_empty() {
            return Vec::new();
        }

        // Per-file preprocessed lines, reused across every pass below.
        let file_lines: Vec<Vec<&str>> = entries
            .iter()
            .map(|(_, content)| content.lines().collect())
            .collect();

        // Two global maps: raw-key (identical, trimmed-per-line) and normalized-key
        // (similar, alphanumeric/whitespace-only). Windows that consist entirely of
        // import/`use`/`from` lines are scaffolding, not logic, so they are excluded
        // from both maps and the per-file shared count (AES305 counts logic duplication).
        let mut raw_global: HashMap<String, HashSet<usize>> = HashMap::with_capacity(entries.len());
        let mut norm_global: HashMap<String, HashSet<usize>> =
            HashMap::with_capacity(entries.len());

        // Per-file window records for range reporting (identical and similar separately).
        let mut raw_windows: Vec<Vec<DupWindow>> = (0..entries.len()).map(|_| Vec::new()).collect();
        let mut norm_windows: Vec<Vec<DupWindow>> =
            (0..entries.len()).map(|_| Vec::new()).collect();

        // Per-file window keys, computed once and reused in the second pass
        // (Issue 19) so the raw/normalized join happens a single time.
        let mut file_keys: Vec<Vec<(String, String)>> = vec![Vec::new(); entries.len()];

        for (fi, lines) in file_lines.iter().enumerate() {
            if lines.len() < min_dup_lines {
                continue;
            }
            for w in lines.windows(min_dup_lines) {
                if is_import_only(w) {
                    continue;
                }
                // Compute both keys once; the owned strings below move into
                // the global maps (Issue 20), and a second pair is kept for the
                // per-file key buffer.
                let raw_key: String = w.iter().map(|l| l.trim()).collect::<Vec<_>>().join("\n");
                let norm_key =
                    shared_quality_rules::utility_code_duplication_detector::normalize_window(w);
                let raw_key_clone = raw_key.clone();
                let norm_key_clone = norm_key.clone();
                raw_global.entry(raw_key).or_default().insert(fi);
                norm_global.entry(norm_key).or_default().insert(fi);
                file_keys[fi].push((raw_key_clone, norm_key_clone));
            }
        }

        let shared_norm_ids: HashSet<String> = shared_keys(&norm_global);
        let shared_raw_ids: HashSet<String> = shared_keys(&raw_global);

        // Per-file counts and shared-window records (for range reporting).
        let mut norm_shared_counts: Vec<usize> = vec![0; entries.len()];
        let mut norm_total_counts: Vec<usize> = vec![0; entries.len()];
        let mut raw_shared_counts: Vec<usize> = vec![0; entries.len()];

        // Reuse the per-file keys built in the first pass instead of
        // recomputing the joins (Issue 19).
        for (fi, keys) in file_keys.iter().enumerate() {
            for (_win_i, (raw_key, norm_key)) in keys.iter().enumerate() {
                norm_total_counts[fi] += 1;
                let start_line = _win_i + 1;
                let end_line = _win_i + min_dup_lines;

                if shared_norm_ids.contains(&norm_key[..]) {
                    norm_shared_counts[fi] += 1;
                    norm_windows[fi].push(DupWindow {
                        start_line,
                        end_line,
                        others: others_for(&norm_global, norm_key.as_str(), fi),
                    });
                }
                if shared_raw_ids.contains(raw_key.as_str()) {
                    raw_shared_counts[fi] += 1;
                    raw_windows[fi].push(DupWindow {
                        start_line,
                        end_line,
                        others: others_for(&raw_global, raw_key.as_str(), fi),
                    });
                }
            }
        }

        // Generate violations (pre-allocate with capacity hint)
        const TOP_K: usize = 5;
        let mut violations = Vec::with_capacity(entries.len());
        for (fi, (file_path, _)) in entries.iter().enumerate() {
            if file_lines[fi].len() < min_dup_lines {
                continue;
            }
            let total = norm_total_counts[fi];
            if total == 0 {
                continue;
            }
            let identical_pct = raw_shared_counts[fi] as f64 / total as f64 * 100.0;
            let similar_pct = norm_shared_counts[fi] as f64 / total as f64 * 100.0;

            // Highest-severity applicable classification wins.
            let (fired, pct, threshold, windows, shared_count) =
                if identical_pct > identical_threshold_pct {
                    (
                        "identical",
                        identical_pct,
                        identical_threshold_pct,
                        &raw_windows[fi],
                        raw_shared_counts[fi],
                    )
                } else if similar_pct > similar_threshold_pct {
                    (
                        "similar",
                        similar_pct,
                        similar_threshold_pct,
                        &norm_windows[fi],
                        norm_shared_counts[fi],
                    )
                } else {
                    continue;
                };

            let mut msg = format!(
                "AES305 ({fired}): {pct:.0}% of this file's non-import windows are shared \
                 (threshold: {threshold:.0}%). {shared_count} of {total} windows are non-unique.",
            );

            // Per-block detail: top K collapsed ranges, each listing the other files
            // (with their matching line ranges) that share this block.
            let ranges = collapse_ranges(
                &windows
                    .iter()
                    .map(|b| (b.start_line, b.end_line))
                    .collect::<Vec<_>>(),
            );
            let mut range_details: Vec<String> = Vec::new();
            for (rs, re) in ranges.iter().take(TOP_K) {
                // Other files whose windows fall inside this collapsed range.
                let mut others: HashSet<usize> = HashSet::new();
                for w in windows
                    .iter()
                    .filter(|w| w.start_line >= *rs && w.end_line <= *re)
                {
                    others.extend(w.others.iter().copied());
                }
                let mut detail: Vec<String> = Vec::new();
                for &ofi in others.iter() {
                    // Line range into `ofi`: the intersection of all stored windows
                    // that overlap the current collapsed range; omit `ofi` when no
                    // stored window overlaps instead of fabricating a default range.
                    let src_windows = if fired == "identical" {
                        &raw_windows
                    } else {
                        &norm_windows
                    };
                    let Some(ofi_windows) = src_windows.get(ofi) else {
                        continue;
                    };
                    let overlapping: Vec<_> = ofi_windows
                        .iter()
                        .filter(|w| w.start_line <= *re && w.end_line >= *rs)
                        .collect();
                    if overlapping.is_empty() {
                        continue;
                    }
                    let (os, oe) = overlapping.iter().fold((usize::MAX, 0), |(s, e), w| {
                        (s.min(w.start_line), e.max(w.end_line))
                    });
                    detail.push(format!("{}:lines {os}-{oe}", entries[ofi].0));
                }
                let joined = detail.join(", ");
                range_details.push(format!("lines {rs}-{re} duplicated in {joined}"));
            }
            let extra = ranges.len().saturating_sub(TOP_K);
            if extra > 0 {
                range_details.push(format!("(+{extra} more)"));
            }
            if !range_details.is_empty() {
                msg.push_str(" Blocks: ");
                msg.push_str(&range_details.join("; "));
            }

            violations.push((
                file_path.to_string(),
                AesCodeAnalysisViolation::CodeDuplication {
                    reason: Some(LintMessage::new(msg)),
                },
            ));
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::collapse_ranges;

    #[test]
    fn collapse_ranges_merges_adjacent_windows() {
        // Windows of size 2 at positions (1,2), (2,3), (3,4) collapse to (1,4).
        let positions = vec![(1, 2), (2, 3), (3, 4)];
        assert_eq!(collapse_ranges(&positions), vec![(1, 4)]);
    }

    #[test]
    fn collapse_ranges_keeps_disjoint_ranges() {
        let positions = vec![(1, 2), (5, 6), (10, 11)];
        assert_eq!(collapse_ranges(&positions), vec![(1, 2), (5, 6), (10, 11)]);
    }

    #[test]
    fn collapse_ranges_merges_overlapping_windows() {
        // (1,5) and (3,8) overlap → (1,8).
        let positions = vec![(1, 5), (3, 8)];
        assert_eq!(collapse_ranges(&positions), vec![(1, 8)]);
    }

    #[test]
    fn collapse_ranges_handles_empty_input() {
        assert!(collapse_ranges(&[]).is_empty());
    }

    #[test]
    fn collapse_ranges_merges_gaps_of_one() {
        // (1,2) and (3,4) are adjacent (2+1 == 3) → merge to (1,4).
        let positions = vec![(1, 2), (3, 4)];
        assert_eq!(collapse_ranges(&positions), vec![(1, 4)]);
    }
}
