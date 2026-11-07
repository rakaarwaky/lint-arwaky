// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Accept pre-fetched (path, content) entries from caller
//   2. For each file, tokenize content into lines; precompute trimmed (raw) and normalized lines
//   3. Slide a window of `min_lines` over lines; hash each window to a u64 key (raw and normalized)
//   4. Two global maps keyed on u64 hash; store file indices
//   5. For each file, compute identical_pct (raw shared) and similar_pct (normalized shared)
//   6. identical_pct > identical_threshold → "identical" violation; else similar_pct > similar_threshold → "similar"
//   7. One violation per file with per-block line ranges (top 5, +N more)
//   8. `others` (matching files) resolved at output time only for flagged files,
//      avoiding per-window HashSet cloning and String key allocations.

use shared_common::LintMessage;
use shared_config_system::ArchitectureConfig;
use shared_quality_rules::contract_quality_protocol::ICodeMetricAnalyzerProtocol;
use shared_quality_rules::taxonomy_quality_rules_vo::AesCodeAnalysisViolation;
use shared_quality_rules::utility_code_duplication_detector::{is_import_line, normalize_line};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

// ─── Block 1: Module Declaration ────────────────────────────

pub struct CodeDuplicationAnalyzer {
    config: Arc<ArchitectureConfig>,
}

// ─── Block 2: Module Contract ────────────────

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates_entries(
        &self,
        entries: &[(PathBuf, String)],
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
            .unwrap_or(75.0);

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

// ─── Block 3: Module Helpers ───────

impl CodeDuplicationAnalyzer {
    pub fn from_config(config: Arc<ArchitectureConfig>) -> Self {
        Self { config }
    }

    /// File-level similarity analysis using pre-read entries.
    /// Optimized: u64 window hashes replace String keys; `others` resolved at
    /// output time only for flagged files (no per-window HashSet cloning).
    pub fn check_file_similarity_entries(
        &self,
        entries: &[(&str, &str)],
        min_dup_lines: usize,
        identical_threshold_pct: f64,
        similar_threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        if entries.is_empty() || min_dup_lines == 0 {
            return Vec::new();
        }

        let n = entries.len();

        // Per-file preprocessed lines: raw (trimmed) and normalized.
        let mut file_raw: Vec<Vec<&str>> = Vec::with_capacity(n);
        let mut file_norm: Vec<Vec<String>> = Vec::with_capacity(n);
        for (_, content) in entries {
            let lines: Vec<&str> = content.lines().collect();
            let raw: Vec<&str> = lines.iter().map(|l| l.trim()).collect();
            let norm: Vec<String> = lines.iter().map(|l| normalize_line(l)).collect();
            file_raw.push(raw);
            file_norm.push(norm);
        }

        // Global maps: u64 window hash -> set of file indices sharing that window.
        let mut raw_global: HashMap<u64, HashSet<usize>> = HashMap::with_capacity(n);
        let mut norm_global: HashMap<u64, HashSet<usize>> = HashMap::with_capacity(n);

        // Per-file window records: (start, end, raw_hash, norm_hash) per position.
        let mut file_windows: Vec<Vec<(usize, usize, u64, u64)>> = vec![Vec::new(); n];

        for fi in 0..n {
            let raw = &file_raw[fi];
            let norm = &file_norm[fi];
            let len = raw.len();
            if len < min_dup_lines {
                continue;
            }
            let mut wh: Vec<(usize, usize, u64, u64)> = Vec::new();
            for start in 0..=(len - min_dup_lines) {
                let end = start + min_dup_lines;
                let w = &raw[start..end];
                if is_import_only(w) {
                    continue;
                }
                let raw_hash = hash_window(w.iter().copied());
                let norm_hash = hash_window(norm[start..end].iter().map(|s| s.as_str()));
                raw_global.entry(raw_hash).or_default().insert(fi);
                norm_global.entry(norm_hash).or_default().insert(fi);
                wh.push((start, end, raw_hash, norm_hash));
            }
            file_windows[fi] = wh;
        }

        // Keys (hashes) shared by 2+ files.
        let shared_raw: HashSet<u64> = raw_global
            .iter()
            .filter(|(_, set)| set.len() > 1)
            .map(|(k, _)| *k)
            .collect();
        let shared_norm: HashSet<u64> = norm_global
            .iter()
            .filter(|(_, set)| set.len() > 1)
            .map(|(k, _)| *k)
            .collect();

        // Per-file shared-window counts and positions. `others` is intentionally
        // NOT cloned here — it is resolved at output time only for flagged files.
        let mut raw_shared_counts = vec![0usize; n];
        let mut norm_shared_counts = vec![0usize; n];
        let mut raw_total = vec![0usize; n];
        let mut norm_total = vec![0usize; n];
        let mut raw_positions: Vec<Vec<WindowPos>> = vec![Vec::new(); n];
        let mut norm_positions: Vec<Vec<WindowPos>> = vec![Vec::new(); n];

        for fi in 0..n {
            for (start, end, raw_hash, norm_hash) in &file_windows[fi] {
                raw_total[fi] += 1;
                norm_total[fi] += 1;
                let start_line = start + 1;
                let end_line = *end;
                if shared_norm.contains(norm_hash) {
                    norm_shared_counts[fi] += 1;
                    norm_positions[fi].push(WindowPos {
                        start_line,
                        end_line,
                        raw_hash: *raw_hash,
                        norm_hash: *norm_hash,
                    });
                }
                if shared_raw.contains(raw_hash) {
                    raw_shared_counts[fi] += 1;
                    raw_positions[fi].push(WindowPos {
                        start_line,
                        end_line,
                        raw_hash: *raw_hash,
                        norm_hash: *norm_hash,
                    });
                }
            }
        }

        // Generate violations (pre-allocate with capacity hint).
        const TOP_K: usize = 5;
        let mut violations = Vec::with_capacity(n);
        for (fi, (file_path, _)) in entries.iter().enumerate() {
            if file_raw[fi].len() < min_dup_lines {
                continue;
            }
            let total = norm_total[fi];
            if total == 0 {
                continue;
            }
            let identical_pct = raw_shared_counts[fi] as f64 / total as f64 * 100.0;
            let similar_pct = norm_shared_counts[fi] as f64 / total as f64 * 100.0;

            // Highest-severity applicable classification wins.
            let (fired, pct, threshold, positions, shared_count) =
                if identical_pct > identical_threshold_pct {
                    (
                        "identical",
                        identical_pct,
                        identical_threshold_pct,
                        &raw_positions[fi],
                        raw_shared_counts[fi],
                    )
                } else if similar_pct > similar_threshold_pct {
                    (
                        "similar",
                        similar_pct,
                        similar_threshold_pct,
                        &norm_positions[fi],
                        norm_shared_counts[fi],
                    )
                } else {
                    continue;
                };

            let mut msg = format!(
                "AES305 ({fired}): {pct:.0}% of this file's non-import windows are shared \
                 (threshold: {threshold:.0}%). {shared_count} of {total} windows are non-unique.",
            );

            let range_positions: Vec<(usize, usize)> = positions
                .iter()
                .map(|p| (p.start_line, p.end_line))
                .collect();
            let ranges = collapse_ranges(&range_positions);
            let global_map = if fired == "identical" {
                &raw_global
            } else {
                &norm_global
            };
            let src_positions = if fired == "identical" {
                &raw_positions
            } else {
                &norm_positions
            };
            let mut range_details: Vec<String> = Vec::new();
            for (rs, re) in ranges.iter().take(TOP_K) {
                // Resolve other files at output time, deduped by window hash.
                let mut seen_hashes = HashSet::new();
                let mut others: HashSet<usize> = HashSet::new();
                for p in positions
                    .iter()
                    .filter(|p| p.start_line >= *rs && p.end_line <= *re)
                {
                    let h = if fired == "identical" {
                        p.raw_hash
                    } else {
                        p.norm_hash
                    };
                    if seen_hashes.insert(h) {
                        if let Some(file_set) = global_map.get(&h) {
                            for &ofi in file_set {
                                if ofi != fi {
                                    others.insert(ofi);
                                }
                            }
                        }
                    }
                }
                let mut detail: Vec<String> = Vec::new();
                for &ofi in &others {
                    // Line range into `ofi`: the intersection of all stored windows
                    // that overlap the current collapsed range.
                    let ofi_windows = &src_positions[ofi];
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

// ─── Block 3: Module Helpers ───────

/// Per-file window position record: 1-based start/end lines plus the u64 hashes.
#[derive(Clone, Copy)]
struct WindowPos {
    start_line: usize,
    end_line: usize,
    raw_hash: u64,
    norm_hash: u64,
}

/// Hash a window of lines to a u64 key using whole-string SipHash per line
/// (no per-character hashing, no String join allocation).
/// Uses UFCS to avoid unused-import lint on the Hash/Hasher traits.
fn hash_window<'a, I>(lines: I) -> u64
where
    I: Iterator<Item = &'a str>,
{
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for line in lines {
        std::hash::Hash::hash(line, &mut hasher);
    }
    std::hash::Hasher::finish(&hasher)
}

/// Collapse per-window positions into maximal consecutive line ranges.
/// 1-based, inclusive. Public so the crate's tests can cover it directly
/// (capability sources must stay free of inline test modules, AES403).
pub fn collapse_ranges(positions: &[(usize, usize)]) -> Vec<(usize, usize)> {
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
        !t.is_empty() && is_import_line(t)
    })
}
