/// semantic_boundary_analyzer — Semantic scope boundary detection.
/// Variant used by semantic_flow_analyzer for scope-aware data flow.

pub struct ScopeBoundaryAnalyzer;

impl ScopeBoundaryAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Find scope bounds (start_line, end_line) around a given line.
    pub fn find_scope_bounds(
        &self,
        content: &str,
        line: Option<usize>,
    ) -> (Option<usize>, Option<usize>) {
        let lines: Vec<&str> = content.lines().collect();
        let target = line.unwrap_or(0);
        if target >= lines.len() {
            return (None, None);
        }

        let mut depth: i32 = 0;
        let mut scope_start: Option<usize> = None;
        let mut scope_end: Option<usize> = None;

        for (i, l) in lines.iter().enumerate() {
            for ch in l.chars() {
                if ch == '{' {
                    if depth == 0 && i <= target {
                        scope_start = Some(i);
                    }
                    depth += 1;
                } else if ch == '}' {
                    depth -= 1;
                    if depth == 0 && i >= target {
                        scope_end = Some(i);
                        return (scope_start, scope_end);
                    }
                }
            }
        }

        (scope_start, Some(lines.len().saturating_sub(1)))
    }
}
