// PURPOSE: MyPy regex patterns — stateless utility for parsing MyPy type checker output
use regex::Regex;
use std::sync::OnceLock;

/// Pattern with column: `file:line:col: type: message [code]`
pub fn mypy_re_with_col() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$").ok())
        .as_ref()
}

/// Pattern without column: `file:line: type: message [code]`
pub fn mypy_re_without_col() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$").ok())
        .as_ref()
}
