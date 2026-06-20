// PURPOSE: taxonomy_import_helper — pure utility functions for path matching, cycle analysis, and string parsing of imports
use std::collections::{HashMap, HashSet};
use once_cell::sync::Lazy;
use regex::Regex;
use crate::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

// ─── Path & Layer Matching ───

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("infrastructure_", "infrastructure"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }

    None
}

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = std::path::Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| file_path.replace('\\', "/"));
    let normalized_root = std::path::Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| root_dir.trim_end_matches('/').replace('\\', "/"));
    if normalized_file.starts_with(&normalized_root) {
        normalized_file[normalized_root.len()..]
            .trim_start_matches('/')
            .to_string()
    } else {
        normalized_file
    }
}

pub fn match_layer_recursive(rel: &str, path_def: &str) -> bool {
    let last_segment = path_def.rsplit('/').next().unwrap_or(path_def);
    rel.starts_with(path_def) || rel.starts_with(last_segment)
}

pub fn match_layer_nonrecursive(rel: &str, path_def: &str) -> bool {
    let norm_path_def = path_def.trim_end_matches('/');

    let parent_dir = match std::path::Path::new(rel).parent().and_then(|p| p.to_str()) {
        Some("") => ".",
        Some(p) => p.trim_end_matches('/'),
        None => ".",
    };

    if parent_dir == norm_path_def {
        return true;
    }

    if parent_dir == "." && !norm_path_def.is_empty() && norm_path_def != "." {
        return true;
    }

    if parent_dir == "." && rel.ends_with(norm_path_def) {
        return true;
    }

    if parent_dir == "." && !norm_path_def.is_empty() {
        return true;
    }

    false
}

// ─── Dummy Checks ───

pub fn dummy_function_ranges(lines: &[&str], lang: LanguageVO) -> Vec<(usize, usize)> {
    match lang {
        LanguageVO::Rust => rust_dummy_function_ranges(lines),
        LanguageVO::Python => python_dummy_function_ranges(lines),
        LanguageVO::JavaScript => js_dummy_function_ranges(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn imported_symbols(lines: &[&str], lang: LanguageVO) -> Vec<(String, usize)> {
    match lang {
        LanguageVO::Rust => rust_imported_symbols(lines),
        LanguageVO::Python => python_imported_symbols(lines),
        LanguageVO::JavaScript => js_imported_symbols(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(String, usize)> {
    let mut traits = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((trait_name, i + 1));
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    traits
}

pub fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(usize, usize)],
    dummy_impl_traits: &[String],
) -> bool {
    if (symbol.starts_with('I')
        && symbol.len() > 1
        && symbol.chars().nth(1).unwrap_or(' ').is_uppercase())
        || symbol.ends_with("Protocol")
        || symbol.ends_with("Port")
        || symbol.ends_with("Trait")
        || symbol.ends_with("Aggregate")
        || symbol.ends_with("Ext")
        || symbol == "Default"
        || symbol == "Debug"
        || symbol == "Display"
        || symbol == "Clone"
        || symbol == "Copy"
        || symbol == "From"
        || symbol == "Into"
        || symbol == "TryFrom"
        || symbol == "TryInto"
        || symbol == "AsRef"
        || symbol == "AsMut"
        || symbol == "Deref"
        || symbol == "DerefMut"
        || symbol == "Iterator"
        || symbol == "IntoIterator"
        || symbol == "Future"
        || symbol == "Stream"
        || symbol == "Read"
        || symbol == "Write"
        || symbol == "BufRead"
        || symbol == "Seek"
        || symbol == "Hash"
        || symbol == "PartialEq"
        || symbol == "Eq"
        || symbol == "PartialOrd"
        || symbol == "Ord"
        || symbol == "Send"
        || symbol == "Sync"
        || symbol == "Unpin"
        || symbol == "Sized"
        || symbol == "Drop"
        || symbol == "Fn"
        || symbol == "FnMut"
        || symbol == "FnOnce"
        || symbol == "async_trait"
        || symbol == "Parser"
        || symbol == "Digest"
        || symbol == "Manager"
        || symbol == "Emitter"
        || symbol == "Serialize"
        || symbol == "Deserialize"
    {
        return true;
    }

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_dummy_range(line_no, dummy_ranges)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
            || (trimmed.starts_with("#") && !trimmed.starts_with("#["))
            || trimmed.contains("PhantomData")
        {
            continue;
        }

        if !trimmed.contains(symbol) {
            continue;
        }

        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                if dummy_impl_traits.contains(&trait_name) {
                    continue;
                }
            }
        }

        return true;
    }

    false
}

// ─── Cycle Detection ───

pub fn normalize_to_layer(name: &str) -> String {
    let layer_prefixes = [
        "taxonomy_",
        "contract_",
        "capabilities_",
        "infrastructure_",
        "agent_",
        "surface_",
    ];
    let base = name.rsplit('/').next().unwrap_or(name);
    for prefix in &layer_prefixes {
        if base.starts_with(prefix) {
            return prefix.trim_end_matches('_').to_string();
        }
    }
    name.to_string()
}

pub fn detect_cycle_edges(edges: &[DependencyEdge]) -> Vec<SymbolName> {
    let normalized_edges: Vec<DependencyEdge> = edges
        .iter()
        .map(|e| DependencyEdge::new(normalize_to_layer(&e.source), normalize_to_layer(&e.target)))
        .collect();

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for e in &normalized_edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .insert(e.target.clone());
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

    fn dfs_collect_paths(
        node: &str,
        graph: &HashMap<String, HashSet<String>>,
        visited: &mut HashSet<String>,
        path_stack: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        if path_stack.contains(&node.to_string()) {
            if let Some(pos) = path_stack.iter().position(|n| n == node) {
                let cycle: Vec<String> = path_stack[pos..].to_vec();
                cycles.push(cycle);
            }
            return;
        }
        if visited.contains(node) {
            return;
        }
        visited.insert(node.to_string());
        path_stack.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                dfs_collect_paths(neighbor, graph, visited, path_stack, cycles);
            }
        }

        path_stack.pop();
    }

    let nodes: Vec<String> = graph.keys().cloned().collect();
    for node in &nodes {
        let mut local_visited: HashSet<String> = HashSet::new();
        let mut path_stack: Vec<String> = Vec::new();
        let mut cycles: Vec<Vec<String>> = Vec::new();
        dfs_collect_paths(
            node,
            &graph,
            &mut local_visited,
            &mut path_stack,
            &mut cycles,
        );
        for cycle in cycles {
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            let dedup_key = sorted_cycle.join("->");
            if reported.insert(dedup_key) {
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i].clone(), next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

pub fn extract_import_modules(content: &str) -> Vec<String> {
    let mut modules = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(module) = rest.split_whitespace().next() {
                modules.push(module.to_string());
            }
        } else if trimmed.starts_with("import ") {
            if let Some(pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[pos + 6..].trim();
                let cleaned = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                modules.push(cleaned.to_string());
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                    let cleaned = rest
                        .trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    modules.push(cleaned.to_string());
                } else if let Some(first_token) = rest.split_whitespace().next() {
                    modules.push(first_token.trim_end_matches(',').to_string());
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';');
            modules.push(module.to_string());
        }
    }
    modules
}

// ─── Unused Imports ───

pub fn extract_imported_aliases(content: &str) -> HashMap<String, String> {
    let mut aliases: HashMap<String, String> = HashMap::new();
    let mut in_cfg_test = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if trimmed == "}" || trimmed.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("from ") {
            let parts: Vec<&str> = rest.splitn(2, " import ").collect();
            if parts.len() == 2 {
                let module = parts[0].trim();
                let names_str = parts[1].trim().trim_matches(|c| c == '(' || c == ')');
                for name_part in names_str.split(',') {
                    let name_part = name_part.trim();
                    if name_part.contains(" as ") {
                        let alias_parts: Vec<&str> = name_part.splitn(2, " as ").collect();
                        let alias = alias_parts[1].trim().to_string();
                        let fullname = format!("{}.{}", module, alias_parts[0].trim());
                        aliases.insert(alias, fullname);
                    } else if !name_part.is_empty() {
                        let fullname = format!("{}.{}", module, name_part);
                        aliases.insert(name_part.to_string(), fullname);
                    }
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("import ") {
            for name_part in rest.split(',') {
                let name_part = name_part.trim();
                if name_part.contains(" as ") {
                    let alias_parts: Vec<&str> = name_part.splitn(2, " as ").collect();
                    let alias = alias_parts[1].trim().to_string();
                    let fullname = alias_parts[0].trim().to_string();
                    aliases.insert(alias, fullname);
                } else if !name_part.is_empty() {
                    aliases.insert(name_part.to_string(), name_part.to_string());
                }
            }
        }
    }

    aliases
}

pub fn extract_exported_symbols(content: &str) -> HashSet<String> {
    let mut exported: HashSet<String> = HashSet::new();
    if let Some(caps) = ALL_RE.as_ref().and_then(|re| re.captures(content)) {
        let inner = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        for item in inner.split(',') {
            let name = item.trim().trim_matches('"').trim_matches('\'').to_string();
            if !name.is_empty() {
                exported.insert(name);
            }
        }
    }
    exported
}

pub fn extract_used_symbols(
    content: &str,
    all_imports: &HashMap<String, String>,
) -> HashSet<String> {
    let mut used: HashSet<String> = HashSet::new();

    let code_lines: String = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("import ") && !t.starts_with("from ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in all_imports.keys() {
        let pattern = format!(r"\b{}\b", regex::escape(alias));
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(&code_lines) {
                used.insert(alias.clone());
            }
        }
    }

    used
}

pub fn extract_rust_js_imports(content: &str) -> Vec<(String, usize)> {
    let mut imports = Vec::new();
    let mut in_cfg_test = false;
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if t == "}" || t.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        let names: Vec<String> = if t.starts_with("use ") {
            let target = t.trim_end_matches(';').trim_start_matches("use ").trim();
            if target.starts_with("std::")
                || target.starts_with("core::")
                || target.starts_with("alloc::")
            {
                continue;
            }
            if let Some(brace_pos) = target.find("::{") {
                let inner = target[brace_pos + 3..].trim_end_matches('}').trim();
                inner
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split(" as ")
                            .last()
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
                    .filter(|n| !n.is_empty() && n != "_" && n != "*")
                    .collect()
            } else {
                let name = target
                    .split("::")
                    .last()
                    .unwrap_or("")
                    .split(" as ")
                    .last()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty() || name == "_" || name == "*" {
                    continue;
                }
                vec![name]
            }
        } else if t.starts_with("import ") {
            if let Some(from_idx) = t.find(" from ") {
                let import_part = t[7..from_idx].trim();
                let names: Vec<String> = if import_part.starts_with('{') {
                    import_part[1..import_part.len() - 1]
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .split(" as ")
                                .last()
                                .unwrap_or("")
                                .trim()
                                .to_string()
                        })
                        .filter(|n| !n.is_empty())
                        .collect()
                } else {
                    vec![import_part.trim().to_string()]
                };
                names
            } else {
                continue;
            }
        } else {
            continue;
        };

        for name in names {
            if (name.starts_with('I')
                && name.len() > 1
                && name.chars().nth(1).unwrap_or(' ').is_uppercase())
                || name.ends_with("Protocol")
                || name.ends_with("Port")
                || name.ends_with("Trait")
                || name.ends_with("Aggregate")
                || name == "Parser"
            {
                continue;
            }
            imports.push((name, i));
        }
    }
    imports
}

pub fn is_name_used(name: &str, content: &str, exclude_line: usize) -> bool {
    if is_rust_trait_import(name) {
        return true;
    }

    let rest = content
        .lines()
        .enumerate()
        .filter(|(j, _)| *j != exclude_line)
        .map(|(_, l)| l)
        .collect::<Vec<_>>()
        .join("\n");
    rest.contains(name)
}

// ─── Helpers local to this module ───

fn is_rust_trait_import(name: &str) -> bool {
    if name.starts_with('I')
        && name.len() > 1
        && name.chars().nth(1).unwrap_or(' ').is_uppercase()
    {
        return true;
    }
    if name.ends_with("Protocol")
        || name.ends_with("Port")
        || name.ends_with("Trait")
        || name.ends_with("Aggregate")
        || name.ends_with("Ext")
    {
        return true;
    }
    matches!(
        name,
        "Default"
            | "Debug"
            | "Display"
            | "Clone"
            | "Copy"
            | "PartialEq"
            | "Eq"
            | "PartialOrd"
            | "Ord"
            | "Hash"
            | "From"
            | "Into"
            | "TryFrom"
            | "TryInto"
            | "AsRef"
            | "AsMut"
            | "Deref"
            | "DerefMut"
            | "Iterator"
            | "IntoIterator"
            | "ExactSizeIterator"
            | "FusedIterator"
            | "Future"
            | "Stream"
            | "Read"
            | "Write"
            | "BufRead"
            | "Seek"
            | "Send"
            | "Sync"
            | "Unpin"
            | "Sized"
            | "Drop"
            | "Fn"
            | "FnMut"
            | "FnOnce"
            | "async_trait"
            | "Digest"
            | "Manager"
            | "Emitter"
            | "Serialize"
            | "Deserialize"
            | "Parser"
    )
}

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn _use_") || trimmed.starts_with("fn dummy_") {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;

            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("def _use_") || trimmed.starts_with("def dummy_") {
            let start = i + 1;
            let mut end = i + 1;
            let indent = lines[i].len() - lines[i].trim_start().len();

            for (idx, line) in lines.iter().enumerate().skip(i + 1) {
                let t = line.trim();
                if t.is_empty() || t.starts_with('#') {
                    end = idx + 1;
                    continue;
                }
                let line_indent = line.len() - line.trim_start().len();
                if line_indent <= indent && !t.is_empty() {
                    break;
                }
                end = idx + 1;
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("function _use")
            || trimmed.starts_with("function dummy")
            || trimmed.starts_with("const _use")
            || trimmed.starts_with("const dummy")
        {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;

            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
            continue;
        }

        if trimmed == "use super::*;" {
            continue;
        }

        let body = trimmed
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();

        if body.contains('{') {
            if let Some(open) = body.find('{') {
                if let Some(close) = body.rfind('}') {
                    let inside = &body[open + 1..close];
                    for part in inside.split(',') {
                        if let Some(symbol) = rust_imported_symbol_from_part(part.trim()) {
                            symbols.push((symbol, idx + 1));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((symbol, idx + 1));
        }
    }

    symbols
}

fn rust_imported_symbol_from_part(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() || part == "self" || part.starts_with('*') {
        return None;
    }

    if let Some((_, alias)) = part.split_once(" as ") {
        return Some(alias.trim().to_string());
    }

    let name = part.split("::").last().unwrap_or(part).trim();
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }

    Some(name.to_string())
}

fn python_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name = name.split_whitespace().next().unwrap_or("");
                    if !name.is_empty() && name != "*" {
                        symbols.push((name.to_string(), idx + 1));
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") {
            let module = trimmed
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
                .unwrap_or("");
            if !module.is_empty() {
                let name = module.rsplit('.').next().unwrap_or(module);
                symbols.push((name.to_string(), idx + 1));
            }
        }
    }

    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = part.split_whitespace().next().unwrap_or("");
                        if !name.is_empty() && name != "type" {
                            symbols.push((name.to_string(), idx + 1));
                        }
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let name = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or("");
                let name = name.trim();
                if !name.is_empty() && name != "default" {
                    symbols.push((name.to_string(), idx + 1));
                }
            }
            continue;
        }

        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = part.trim().split(':').next().unwrap_or("").trim();
                        if !name.is_empty() {
                            symbols.push((name.to_string(), idx + 1));
                        }
                    }
                }
            }
        }
    }

    symbols
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| line_no >= *start && line_no <= *end)
}

fn impl_trait_name(line: &str) -> Option<String> {
    let after_impl = line.strip_prefix("impl ")?.trim();
    let (trait_part, _) = after_impl.split_once(" for ")?;
    let trait_name = trait_part.split("::").last().unwrap_or(trait_part).trim();
    if trait_name.is_empty() {
        return None;
    }
    Some(trait_name.to_string())
}

fn impl_block<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn trait_impl_is_dummy(lines: &[&str]) -> bool {
    let mut method_count = 0usize;
    let mut dummy_count = 0usize;
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
            method_count += 1;
            let (end, body) = function_body(lines, i);
            if function_body_is_dummy(&body) {
                dummy_count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }

    method_count > 0 && dummy_count == method_count
}

fn function_body<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn function_body_is_dummy(lines: &[&str]) -> bool {
    let body: String = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");

    let trimmed = body.trim();
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    if trimmed.len() < 100 {
        let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
        let short_markers = ["todo!(", "unimplemented!(", "panic!(", "unreachable!("];
        if inner.is_empty() || short_markers.iter().any(|m| inner.starts_with(m)) {
            return true;
        }
    }

    let panic_marker = format!("{}!(", "panic");
    let dummy_markers = [
        "todo!(",
        "unimplemented!(",
        &panic_marker,
        "unreachable!(",
        "return Err(Default::default())",
        "return Ok(Default::default())",
    ];

    dummy_markers.iter().any(|marker| body.contains(marker))
}
