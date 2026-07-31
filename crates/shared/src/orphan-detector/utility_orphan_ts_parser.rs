// PURPOSE: utility_orphan_ts_parser — comment-aware structured parsing for TypeScript/JavaScript.
// Stateless standalone functions. Depends only on taxonomy VOs.

use super::taxonomy_orphan_parse_result_vo::{AstFnDefVO, AstImportVO, TsParseResultVO};

// ─── Block 1: Main Parse Function ─────────────────────────

pub fn parse_ts(content: &str) -> TsParseResultVO {
    let mut result = TsParseResultVO {
        parse_ok: true,
        ..Default::default()
    };
    let code_lines = strip_ts_comments(content);
    let mut line_num = 0usize;
    for line in &code_lines {
        line_num += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("import ") {
            parse_ts_import(trimmed, &mut result, line_num);
        } else if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            parse_ts_export(trimmed, &mut result, line_num);
        } else if let Some(rest) = trimmed.strip_prefix("class ") {
            parse_ts_class(rest, &mut result);
        } else if trimmed.starts_with("function ") || trimmed.starts_with("async function ") {
            let is_async = trimmed.starts_with("async function ");
            let fn_part = if is_async {
                &trimmed[15..]
            } else {
                &trimmed[9..]
            };
            if let Some(paren_start) = fn_part.find('(') {
                let name = fn_part[..paren_start].trim().to_string();
                let is_dummy = trimmed.contains("=> {}") || trimmed.ends_with(";");
                result.functions.push(AstFnDefVO {
                    name,
                    is_pub: true,
                    line: line_num,
                    end_line: line_num,
                    is_dummy,
                });
            }
        }
    }
    // Collect used identifiers
    let mut ids = std::collections::HashSet::new();
    for line in &code_lines {
        let trimmed = line.trim();
        if trimmed.starts_with("import ")
            || trimmed.starts_with("export ")
            || trimmed.starts_with("//")
        {
            continue;
        }
        for word in trimmed.split(|c: char| !c.is_alphanumeric() && c != '_' && c != '$') {
            if !word.is_empty()
                && word
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$')
            {
                ids.insert(word.to_string());
            }
        }
    }
    result.used_identifiers = ids.into_iter().collect();
    result
}

// ─── Block 2: Statement Parsers ───────────────────────────

fn parse_ts_import(trimmed: &str, result: &mut TsParseResultVO, line: usize) {
    if let Some(from_pos) = trimmed.find(" from ") {
        let path_part = trimmed[from_pos + 6..].trim();
        let path = path_part
            .trim_matches(|c| c == '\'' || c == '"' || c == ';')
            .to_string();
        let segments = path_to_segments(&path);
        result.imports.push(AstImportVO::new(
            path,
            segments,
            false,
            trimmed.contains("* as"),
            line,
        ));
    } else {
        let path = trimmed
            .strip_prefix("import ")
            .unwrap_or("")
            .trim()
            .trim_matches(|c| c == '\'' || c == '"' || c == ';')
            .to_string();
        if !path.is_empty() {
            let segments = path_to_segments(&path);
            result
                .imports
                .push(AstImportVO::new(path, segments, false, false, line));
        }
    }
}

fn parse_ts_export(trimmed: &str, result: &mut TsParseResultVO, line: usize) {
    let Some(from_pos) = trimmed.find(" from ") else {
        return;
    };
    let path_part = trimmed[from_pos + 6..].trim();
    let path = path_part
        .trim_matches(|c| c == '\'' || c == '"' || c == ';')
        .to_string();
    let segments = path_to_segments(&path);
    result.imports.push(AstImportVO::new(
        path,
        segments,
        true,
        trimmed.contains('*'),
        line,
    ));
}

fn parse_ts_class(rest: &str, result: &mut TsParseResultVO) {
    let Some(impl_pos) = rest.find(" implements ") else {
        return;
    };
    let class_name = rest[..impl_pos].trim().to_string();
    let after_impl = &rest[impl_pos + 12..];
    let interfaces: Vec<String> = after_impl
        .split(['{', ','])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    result.class_implements.push((class_name, interfaces));
}

// ─── Block 3: Helpers ─────────────────────────────────────

fn path_to_segments(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty() && *s != "." && *s != "..")
        .map(String::from)
        .collect()
}

fn strip_ts_comments(content: &str) -> Vec<String> {
    let mut in_block_comment = false;
    content
        .lines()
        .map(|line| {
            if in_block_comment {
                if let Some(end_pos) = line.find("*/") {
                    in_block_comment = false;
                    return line[end_pos + 2..].to_string();
                }
                return String::new();
            }
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                return String::new();
            }
            if trimmed.starts_with("/*") {
                if let Some(end_pos) = line.find("*/") {
                    return line[end_pos + 2..].to_string();
                }
                in_block_comment = true;
                return String::new();
            }
            let mut in_single = false;
            let mut in_double = false;
            let mut in_template = false;
            let mut result = String::new();
            let mut prev_char = ' ';
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let ch = chars[i];
                if ch == '\'' && !in_double && !in_template && prev_char != '\\' {
                    in_single = !in_single;
                } else if ch == '"' && !in_single && !in_template && prev_char != '\\' {
                    in_double = !in_double;
                } else if ch == '`' && !in_single && !in_double && prev_char != '\\' {
                    in_template = !in_template;
                } else if ch == '/' && !in_single && !in_double && !in_template {
                    if i + 1 < chars.len() && chars[i + 1] == '/' {
                        break;
                    }
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
                        if let Some(end) = line[i + 2..].find("*/") {
                            i += end + 3;
                            prev_char = '/';
                            continue;
                        }
                        in_block_comment = true;
                        break;
                    }
                }
                result.push(ch);
                prev_char = ch;
                i += 1;
            }
            result
        })
        .collect()
}
