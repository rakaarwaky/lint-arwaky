// PURPOSE: TSC regex patterns — stateless utility for parsing TypeScript compiler output
use regex::Regex;
use std::sync::OnceLock;

/// Pattern 1: `file(line,col): error TSxxxx: message`
pub fn tsc_pattern1() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^(]+)\((\d+),(\d+)\):\s+error\s+(TS\d+):\s+(.*)$").ok())
        .as_ref()
}

/// Pattern 2: `file:line:col - error TSxxxx: message`
pub fn tsc_pattern2() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):(\d+)\s+-\s+error\s+(TS\d+):\s+(.*)$").ok())
        .as_ref()
}
