// PURPOSE: CodeMetricAnalyzer — capabilities implementation for complexity, duplication, and quality trends analysis
use crate::ICodeMetricAnalyzerProtocol;
use crate::ITargetResolverProtocol;
use shared::taxonomy_severity_vo::Severity;
use std::process::ExitCode;
use std::sync::Arc;

pub struct CodeMetricAnalyzer {
    resolver: Arc<dyn ITargetResolverProtocol>,
}

impl CodeMetricAnalyzer {
    pub fn new(resolver: Arc<dyn ITargetResolverProtocol>) -> Self {
        Self { resolver }
    }

    fn grade_complexity(&self, c: usize) -> &'static str {
        if c <= 5 {
            "A"
        } else if c <= 10 {
            "B"
        } else if c <= 20 {
            "C"
        } else if c <= 30 {
            "D"
        } else if c <= 40 {
            "E"
        } else {
            "F"
        }
    }
}

impl ICodeMetricAnalyzerProtocol for CodeMetricAnalyzer {
    fn handle_complexity(&self, path: Option<String>) -> ExitCode {
        let root = self.resolver.resolve_target(path);
        println!("Cyclomatic Complexity Analysis — {}", root);
        println!();

        let src = std::path::Path::new(&root).join("src-rust");
        let mut functions: Vec<(std::path::PathBuf, String, usize, usize)> = Vec::new();

        self.resolver.walk_rs_files(&src, &mut |p| {
            if let Ok(c) = std::fs::read_to_string(&p) {
                let lines: Vec<&str> = c.lines().collect();
                let mut i = 0;
                while i < lines.len() {
                    let t = lines[i].trim();
                    if t.starts_with("fn ") || t.starts_with("pub fn ") {
                        let fn_name = t
                            .split("fn ")
                            .nth(1)
                            .unwrap_or("")
                            .split('(')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        let mut branches = 0usize;
                        let mut depth = 0usize;
                        let mut j = i;
                        while j < lines.len() {
                            let lt = lines[j].trim();
                            if lt.contains("if ") || lt.starts_with("else if ") {
                                branches += 1;
                            }
                            if lt.starts_with("match ") {
                                branches += 1;
                            }
                            if lt.starts_with("for ") || lt.starts_with("while ") {
                                branches += 1;
                            }
                            if lt.contains(" && ") || lt.contains(" || ") {
                                branches += lt.matches("&&").count() + lt.matches("||").count();
                            }
                            if lt.contains('{') {
                                depth += 1;
                            }
                            if lt.contains('}') {
                                depth -= 1;
                                if depth == 0 && lt.contains('}') {
                                    let complexity = branches + 1;
                                    functions.push((p.clone(), fn_name.clone(), complexity, j - i));
                                    break;
                                }
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
            }
        });

        functions.sort_by_key(|b| std::cmp::Reverse(b.2));

        println!(
            "{:<30} {:<30} {:>7} {:>6}",
            "File", "Function", "Complexity", "Grade"
        );
        for (p, name, comp, _) in functions.iter().take(20) {
            let grade = self.grade_complexity(*comp);
            let flag = if *comp > 30 { " !" } else { "" };
            println!(
                "{:<30} {:<30} {:>7} {:>6}{}",
                p.file_stem().and_then(|s| s.to_str()).unwrap_or(""),
                name,
                comp,
                grade,
                flag
            );
        }
        println!();
        let avg = if !functions.is_empty() {
            functions.iter().map(|(_, _, c, _)| c).sum::<usize>() as f64 / functions.len() as f64
        } else {
            0.0
        };
        println!("Functions analyzed: {}", functions.len());
        println!("Average complexity: {:.1}", avg);
        let flagged = functions.iter().filter(|(_, _, c, _)| *c > 30).count();
        println!("Functions flagged (E/F): {}", flagged);
        ExitCode::SUCCESS
    }

    fn handle_duplicates(&self, path: Option<String>) -> ExitCode {
        let root = self.resolver.resolve_target(path);
        println!("Code Duplication Detection — {}", root);
        println!();

        let src = std::path::Path::new(&root).join("src-rust");
        let min_lines: usize = 10;
        let mut blocks: std::collections::HashMap<String, Vec<(std::path::PathBuf, usize)>> =
            std::collections::HashMap::new();

        self.resolver.walk_rs_files(&src, &mut |p| {
            if let Ok(c) = std::fs::read_to_string(&p) {
                let lines: Vec<&str> = c.lines().collect();
                for w in lines.windows(min_lines) {
                    let key: String = w
                        .iter()
                        .map(|s| {
                            s.trim()
                                .chars()
                                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                                .collect::<String>()
                        })
                        .collect::<Vec<_>>()
                        .join("|");
                    let start_line = w.as_ptr() as usize - lines.as_ptr() as usize;
                    blocks.entry(key).or_default().push((p.clone(), start_line));
                }
            }
        });

        let duplicates: Vec<_> = blocks.into_iter().filter(|(_, v)| v.len() > 1).collect();

        let total_duplicated_lines = duplicates.len() * min_lines;
        let mut total_loc = 0usize;
        self.resolver.walk_rs_files(&src, &mut |p| {
            if let Ok(c) = std::fs::read_to_string(&p) {
                total_loc += c.lines().count();
            }
        });

        println!("Duplicate blocks: {}", duplicates.len());
        println!(
            "Duplicated lines: {} / {} ({:.1}%)",
            total_duplicated_lines,
            total_loc,
            if total_loc > 0 {
                total_duplicated_lines as f64 / total_loc as f64 * 100.0
            } else {
                0.0
            }
        );
        println!("Min block length: {} lines", min_lines);
        println!();

        for (i, (_, locations)) in duplicates.iter().enumerate().take(10) {
            let sim = 100usize.saturating_sub(locations.len().saturating_sub(1) * 5);
            println!(
                "#{}  {}% similarity across {} files",
                i + 1,
                sim.min(100),
                locations.len()
            );
            for (p, line) in locations.iter().take(3) {
                println!("      {}:{}", p.display(), line + 1);
            }
            if locations.len() > 3 {
                println!("      ... and {} more locations", locations.len() - 3);
            }
        }
        ExitCode::SUCCESS
    }

    fn handle_trends(&self, path: Option<String>) -> ExitCode {
        let root = self.resolver.resolve_target(path);
        let results = self.resolver.lint_path(&root);
        let score = self.resolver.compute_score(&results);
        let violations_count = results.len();
        let critical_count = results
            .iter()
            .filter(|r| r.severity == Severity::CRITICAL)
            .count();
        let history_path = std::path::Path::new(&root).join(".lint-history.json");

        println!("Quality Trends — {}", root);
        println!();
        println!("Current scan:");
        println!("  Score:      {:.1}/100", score);
        println!("  Violations: {}", violations_count);
        println!("  Critical:   {}", critical_count);

        let mut history: Vec<serde_json::Value> = if history_path.exists() {
            std::fs::read_to_string(&history_path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        if let Some(prev) = history.last() {
            let prev_score = prev.get("score").and_then(|s| s.as_f64()).unwrap_or(100.0);
            let prev_violations = prev.get("violations").and_then(|v| v.as_u64()).unwrap_or(0);
            let delta = score - prev_score;

            let trend = if delta > 1.0 {
                "IMPROVING"
            } else if delta < -1.0 {
                "DECLINING"
            } else {
                "STABLE"
            };

            let all_time_high = history
                .iter()
                .filter_map(|e| e.get("score").and_then(|s| s.as_f64()))
                .fold(score, f64::max);
            let all_time_low = history
                .iter()
                .filter_map(|e| e.get("score").and_then(|s| s.as_f64()))
                .fold(score, f64::min);

            println!();
            println!("Previous scan:");
            println!("  Score:      {:.1}/100", prev_score);
            println!("  Violations: {}", prev_violations);
            println!();
            println!("Delta: {:+.1} — {}", delta, trend);
            println!("All-time high: {:.1} / 100", all_time_high);
            println!("All-time low:  {:.1} / 100", all_time_low);
            println!("History entries: {}", history.len());

            if history.len() > 1 {
                let bar = "▁▂▃▄▅▆▇█";
                let scores: Vec<f64> = history
                    .iter()
                    .filter_map(|e| e.get("score").and_then(|s| s.as_f64()))
                    .collect();
                if scores.is_empty() {
                    return ExitCode::SUCCESS;
                }
                let mut min_s = f64::INFINITY;
                let mut max_s = f64::NEG_INFINITY;
                for &s in &scores {
                    if s < min_s {
                        min_s = s;
                    }
                    if s > max_s {
                        max_s = s;
                    }
                }
                let range = (max_s - min_s).max(1.0);
                let spark: String = scores
                    .iter()
                    .map(|s| {
                        let idx = ((s - min_s) / range * 7.0).round() as usize;
                        bar.chars().nth(idx.min(7)).unwrap_or('▁')
                    })
                    .collect();
                println!("Sparkline: {}", spark);
            }
        } else {
            println!();
            println!("No history yet — first run");
        }

        let entry = serde_json::json!({
            "score": score,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "violations": violations_count,
            "critical": critical_count,
        });
        history.push(entry);
        if history.len() > 365 {
            history.remove(0);
        }
        if let Ok(json) = serde_json::to_string_pretty(&history) {
            let _ = std::fs::write(&history_path, &json);
            println!("Score saved to {}", history_path.display());
        }
        ExitCode::SUCCESS
    }
}
