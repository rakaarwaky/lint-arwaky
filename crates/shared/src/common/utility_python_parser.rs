// PURPOSE: utility_python_parser — comment-aware structured parsing for Python.
// Stateless standalone functions. Depends only on taxonomy VOs.

use crate::orphan_rules::taxonomy_orphan_parse_result_vo::{
    AstFnDefVO, AstImportVO, PythonParseResultVO,
};

pub fn parse_python(content: &str) -> PythonParseResultVO {
    let mut result = PythonParseResultVO {
        parse_ok: true,
        ..Default::default()
    };
    let code_lines = strip_python_comments(content);
    let mut line_num = 0usize;
    for line in &code_lines {
        line_num += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        parse_python_line(trimmed, &mut result, line_num);
    }
    result.used_identifiers = collect_python_identifiers(&code_lines);
    result
}

/// Process a single Python line for imports, classes, and functions.
fn parse_python_line(trimmed: &str, result: &mut PythonParseResultVO, line_num: usize) {
    if let Some(rest) = trimmed.strip_prefix("from ") {
        parse_python_from_import(rest, result, line_num);
    } else if let Some(rest) = trimmed.strip_prefix("import ") {
        parse_python_plain_import(rest, result, line_num);
    } else if let Some(rest) = trimmed.strip_prefix("class ") {
        parse_python_class(rest, result, line_num);
    } else if trimmed.starts_with("def ") || trimmed.starts_with("async def ") {
        parse_python_function(trimmed, result, line_num);
    }
}

/// Parse a function declaration line and add to results.
fn parse_python_function(trimmed: &str, result: &mut PythonParseResultVO, line_num: usize) {
    let is_async = trimmed.starts_with("async def ");
    let def_part = if is_async {
        trimmed.strip_prefix("async def ").unwrap_or(trimmed)
    } else {
        trimmed.strip_prefix("def ").unwrap_or(trimmed)
    };
    if let Some(paren_start) = def_part.find('(') {
        let name = def_part[..paren_start].trim().to_string();
        let line = line_num;
        let is_dummy = trimmed.contains("...") || trimmed.ends_with("pass");
        result.functions.push(AstFnDefVO {
            name,
            is_pub: true,
            line,
            end_line: line,
            is_dummy,
        });
    }
}

/// Collect identifiers from code lines (excluding import/comment lines).
fn collect_python_identifiers(code_lines: &[String]) -> Vec<String> {
    let mut ids = std::collections::HashSet::new();
    for line in code_lines {
        let trimmed = line.trim();
        if trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with('#')
        {
            continue;
        }
        for word in trimmed.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if !word.is_empty()
                && word
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphabetic() || c == '_')
            {
                ids.insert(word.to_string());
            }
        }
    }
    ids.into_iter().collect()
}

fn parse_python_from_import(rest: &str, result: &mut PythonParseResultVO, line: usize) {
    let Some(import_pos) = rest.find(" import ") else {
        return;
    };
    let module_part = rest[..import_pos].trim();
    let names_part = rest[import_pos + 8..].trim();
    let segments: Vec<String> = module_part.split('.').map(String::from).collect();
    result.imports.push(AstImportVO::new(
        module_part.to_string(),
        segments.clone(),
        false,
        names_part.trim() == "*",
        line,
    ));
    let names: Vec<&str> = names_part
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .collect();
    for name in names {
        let mut name_segments = segments.clone();
        name_segments.push(name.to_string());
        result.imports.push(AstImportVO::new(
            format!("{}.{}", module_part, name),
            name_segments,
            false,
            false,
            line,
        ));
    }
}

fn parse_python_plain_import(rest: &str, result: &mut PythonParseResultVO, line: usize) {
    let module = rest.split(" as ").next().unwrap_or(rest).trim();
    let segments: Vec<String> = module.split('.').map(String::from).collect();
    result.imports.push(AstImportVO::new(
        module.to_string(),
        segments,
        false,
        false,
        line,
    ));
}

fn parse_python_class(rest: &str, result: &mut PythonParseResultVO, _line: usize) {
    let Some(paren_start) = rest.find('(') else {
        return;
    };
    let class_name = rest[..paren_start].trim().to_string();
    let Some(paren_end) = rest[paren_start..].find(')') else {
        return;
    };
    let bases_str = &rest[paren_start + 1..paren_start + paren_end];
    let bases: Vec<String> = bases_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    result.class_bases.push((class_name, bases));
}

fn strip_python_comments(content: &str) -> Vec<String> {
    content.lines().map(|line| {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            return String::new();
        }
        strip_python_inline_comment(line)
    }).collect()
}

/// Strip inline `#` comment from a single Python line, respecting string literals.
fn strip_python_inline_comment(line: &str) -> String {
    let mut in_single = false;
    let mut in_double = false;
    let mut result = String::new();
    let mut prev_char = ' ';
    for ch in line.chars() {
        if ch == '\'' && !in_double && prev_char != '\\' {
            in_single = !in_single;
        } else if ch == '"' && !in_single && prev_char != '\\' {
            in_double = !in_double;
        } else if ch == '#' && !in_single && !in_double {
            break;
        }
        result.push(ch);
        prev_char = ch;
    }
    result
}
