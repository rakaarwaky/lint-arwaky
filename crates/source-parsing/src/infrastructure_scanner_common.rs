// PURPOSE: ScannerCommon — shared scaffolding for language scanners
use std::collections::HashSet;
use std::fs;

use shared::source_parsing::taxonomy_parser_error::SourceParserError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_error::Cause;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_error_vo::ErrorCode;

pub fn read_file_content(path: &FilePath) -> Result<String, SourceParserError> {
    fs::read_to_string(&path.value).map_err(|e| SourceParserError {
        path: path.clone(),
        message: ErrorMessage::new(format!("Failed to read file: {}", e)),
        error_code: ErrorCode::raw("FILE_READ_ERROR"),
        cause: Cause::new(e.to_string()),
    })
}

pub struct ParseState {
    pub defined: HashSet<String>,
    pub used: HashSet<String>,
    pub exported: HashSet<String>,
    pub skip_until: usize,
    pub brace_count: i32,
}

impl Default for ParseState {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseState {
    pub fn new() -> Self {
        Self {
            defined: HashSet::new(),
            used: HashSet::new(),
            exported: HashSet::new(),
            skip_until: 0,
            brace_count: 0,
        }
    }
}

pub fn is_empty_line(stripped: &str) -> bool {
    stripped.is_empty()
}

pub fn is_comment_line(stripped: &str, comment_prefixes: &[&str]) -> bool {
    comment_prefixes
        .iter()
        .any(|prefix| stripped.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty_line() {
        assert!(is_empty_line(""));
        assert!(!is_empty_line("code"));
    }

    #[test]
    fn test_is_comment_line_rust() {
        let prefixes = &["//", "/*", "*"];
        assert!(is_comment_line("// comment", prefixes));
        assert!(is_comment_line("/* block", prefixes));
        assert!(is_comment_line("* continuation", prefixes));
        assert!(!is_comment_line("let x = 1;", prefixes));
    }

    #[test]
    fn test_is_comment_line_python() {
        let prefixes = &["#"];
        assert!(is_comment_line("# comment", prefixes));
        assert!(!is_comment_line("x = 1", prefixes));
    }
}
