// dispatch_parser_types — Internal parser state types for dispatch routing analysis.
//
// These types represent the INTERNAL PARSING STATE of the dispatch routing parser
// algorithm. They are implementation details of how the parser tracks class
// boundaries and brace depth while scanning source code.
//
// They are NOT domain concepts and should NOT be in the shared taxonomy.

/// Tracks current brace { } nesting depth during parsing.
#[derive(Debug, Clone, Default)]
pub struct BraceDepthVO {
    pub value: i32,
}

/// Holds method arguments string captured during parsing.
#[derive(Debug, Clone, Default)]
pub struct MethodArgsVO {
    pub value: Option<String>,
}

/// A scope name (class/method) during parse traversal.
#[derive(Debug, Clone, Default)]
pub struct ScopeNameVO {
    pub value: Option<String>,
}

/// Indentation size — tracks indent level during parsing.
#[derive(Debug, Clone, Default)]
pub struct IndentSizeVO {
    pub value: i32,
}

/// Aggregate state of the class parser at any given line.
#[derive(Debug, Clone)]
pub struct ClassParsingStateVO {
    pub current_class: ScopeNameVO,
    pub class_brace_depth: BraceDepthVO,
    pub current_brace_depth: BraceDepthVO,
}

impl ClassParsingStateVO {
    pub fn new() -> Self {
        Self {
            current_class: ScopeNameVO::default(),
            class_brace_depth: BraceDepthVO::default(),
            current_brace_depth: BraceDepthVO::default(),
        }
    }

    pub fn is_inside_class(&self) -> bool {
        self.current_class.value.is_some()
    }

    pub fn enter_class(&mut self, class_name: String, brace_depth: i32) {
        self.current_class = ScopeNameVO { value: Some(class_name) };
        self.class_brace_depth = BraceDepthVO { value: brace_depth };
    }

    pub fn exit_class_if_needed(&mut self) {
        if self.current_brace_depth.value <= self.class_brace_depth.value {
            self.current_class = ScopeNameVO::default();
            self.class_brace_depth = BraceDepthVO::default();
        }
    }

    pub fn update_brace_depth(&mut self, delta: i32) {
        self.current_brace_depth.value += delta;
    }
}
