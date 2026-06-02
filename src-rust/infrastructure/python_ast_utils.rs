/// python_ast_utils — Shared AST helper functions for Python analysis.

pub struct PythonASTUtils;

impl PythonASTUtils {
    pub fn new() -> Self {
        Self
    }

    pub fn is_dead_class(name: &str, bases: &[String]) -> bool {
        bases.iter().any(|b| b == "_arch_dead_marker")
    }
}
