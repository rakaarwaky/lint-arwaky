/// semantic_boundary_resolver — Resolves enclosing scope for semantic tracer.
/// Used by semantic_tracer_analyzer for scope resolution.

pub struct ScopeBoundaryResolver {}

impl ScopeBoundaryResolver {
    pub fn new() -> Self {
        Self {}
    }

    /// Resolve the enclosing scope name for a given file and line.
    pub fn resolve_enclosing_scope(&self, file_path: &str, line: usize) -> Option<String> {
        let content = std::fs::read_to_string(file_path).ok()?;
        let lines: Vec<&str> = content.lines().collect();

        // Walk backward from `line` looking for function/class declaration
        let func_re = regex::Regex::new(r"(?:function|class|def|fn)\s+(\w+)").ok()?;
        let mut depth: i32 = 0;
        for i in (0..=line.min(lines.len().saturating_sub(1))).rev() {
            let l = lines[i];
            for ch in l.chars().rev() {
                if ch == '}' {
                    depth += 1;
                } else if ch == '{' {
                    depth -= 1;
                }
            }
            if depth < 0 {
                if let Some(caps) = func_re.captures(l) {
                    return Some(caps[1].to_string());
                }
            }
        }
        None
    }
}
