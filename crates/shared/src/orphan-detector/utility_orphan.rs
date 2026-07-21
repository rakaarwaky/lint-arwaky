use once_cell::sync::OnceCell;
use regex::Regex;

static STRUCT_RE: OnceCell<Option<Regex>> = OnceCell::new();
static TRAIT_RE: OnceCell<Option<Regex>> = OnceCell::new();

fn struct_re() -> Option<&'static Regex> {
    STRUCT_RE
        .get_or_init(|| Regex::new(r"(?:pub\s+)?struct\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn trait_re() -> Option<&'static Regex> {
    TRAIT_RE
        .get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn extract_struct_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(re) = struct_re() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name != "Self" && !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

pub fn extract_trait_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(re) = trait_re() {
        for cap in re.captures_iter(content) {
            names.push(cap[1].to_string());
        }
    }
    names
}

pub fn normalize_module_component(value: &str) -> String {
    value.replace(['-', '.'], "_")
}

pub fn normalize_module_path(value: &str) -> String {
    value
        .split('/')
        .map(normalize_module_component)
        .collect::<Vec<_>>()
        .join("/")
}

pub fn contains_delimited(content: &str, token: &str) -> bool {
    if !content.contains(token) {
        return false;
    }

    let delimiters: &[char] = &[
        ' ', '\t', '\n', '\r', ';', ',', '(', ')', '{', '}', '"', '\'',
    ];

    for (idx, _) in content.char_indices() {
        // Only check at character boundaries (skip multi-byte sequences)
        if idx > 0 && !is_char_boundary(content, idx) {
            continue;
        }

        let remaining = &content[idx..];
        if !remaining.starts_with(token) {
            continue;
        }

        let before = if idx == 0 {
            ' '
        } else {
            // Safe: char_indices guarantees idx is at a valid char boundary
            content[..idx].chars().next_back().unwrap_or(' ')
        };

        let after_pos = idx + token.chars().map(|c| c.len_utf8()).sum::<usize>();
        let after = content[after_pos..].chars().next().unwrap_or(' ');

        let boundary_before = before.is_whitespace() || delimiters.contains(&before);
        let boundary_after =
            after.is_whitespace() || delimiters.contains(&after) || after == '\n' || after == '\r';

        if boundary_before && boundary_after {
            return true;
        }
    }

    false
}

fn is_char_boundary(s: &str, pos: usize) -> bool {
    if pos >= s.len() {
        return true;
    }
    let bytes = s.as_bytes();
    (bytes[pos] & 0xC0) != 0x80
}

pub fn import_tokens(path: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let path = path.replace('\\', "/");
    let path = path.trim_start_matches('/');

    let stem = std::path::Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    if stem.is_empty() {
        return tokens;
    }

    tokens.push(stem.clone());

    let normalized_stem = normalize_module_component(&stem);
    if normalized_stem != stem {
        tokens.push(normalized_stem);
    }

    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 2 {
        let parent = parts[parts.len() - 2];
        let partial = format!("{}/{}", parent, stem);
        tokens.push(partial.clone());
        tokens.push(partial.replace('/', "::"));

        let normalized_partial = normalize_module_path(&partial);
        if normalized_partial != partial {
            tokens.push(normalized_partial.clone());
            tokens.push(normalized_partial.replace('/', "::"));
        }
    }

    for i in 2..parts.len() {
        let partial = parts[parts.len() - i..].join("/");
        tokens.push(partial);
    }

    let source_prefixes = ["crate::", "shared::", "self::", "super::"];
    let existing: Vec<String> = tokens.clone();
    for prefix in &source_prefixes {
        for tok in &existing {
            tokens.push(format!("{prefix}{tok}"));
        }
    }

    tokens.sort();
    tokens.dedup();
    tokens
}

/// Strip leading generic parameter lists (e.g., `<T>`, `<T: Clone>`) from a string.
fn strip_leading_generics(s: &str) -> &str {
    let mut s = s.trim();

    while let Some(rest) = s.strip_prefix('<') {
        let mut depth = 1usize;
        let mut end = None;

        for (idx, ch) in rest.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(idx);
                        break;
                    }
                }
                _ => {}
            }
        }

        match end {
            Some(pos) => s = rest[pos + 1..].trim(),
            None => break,
        }
    }

    s
}

pub fn has_trait_implementation(content: &str, trait_name: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with('#')
        {
            continue;
        }
        if trimmed.starts_with("impl") && trimmed.contains(" for ") {
            let after_impl = trimmed[4..].trim();

            let trait_part = match after_impl.find(" for ") {
                Some(pos) => after_impl[..pos].trim(),
                None => continue,
            };

            let trait_part = strip_leading_generics(trait_part);
            let trait_base = trait_part.split('<').next().unwrap_or(trait_part).trim();
            let trait_last = trait_base.split("::").last().unwrap_or(trait_base);

            if trait_last == trait_name
                || trait_last.ends_with(trait_name)
                || trait_name.ends_with(trait_last)
            {
                return true;
            }
        }
        if let Some(class_pos) = trimmed.find("class ") {
            let after_class = &trimmed[class_pos + 6..];
            if let Some(paren_pos) = after_class.find('(') {
                let bases = &after_class[paren_pos + 1..];
                if let Some(close_paren) = bases.find(')') {
                    for base in bases[..close_paren].split(',') {
                        if base.trim() == trait_name {
                            return true;
                        }
                    }
                }
            }
        }
        if let Some(impl_pos) = trimmed.find(" implements ") {
            let after_impl = &trimmed[impl_pos + 13..];
            for iface in after_impl.split(',') {
                let iface = iface.trim().trim_end_matches('{').trim();
                if iface == trait_name {
                    return true;
                }
            }
        }
    }
    false
}

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

#[cfg(test)]
mod phase3_regression_tests {
    use super::*;

    /// Regression test for Phase 3.3: has_trait_implementation strips generics correctly.
    /// Trait names with generics like `HashMap<String, i32>` should be matched by their
    /// base name (e.g., "HashMap") without the generic parameters.
    #[test]
    fn has_trait_implementation_strips_generics() {
        let content = "impl HashMap<String, i32> for MyStruct {\n    fn foo(&self) {}\n}";
        assert!(
            has_trait_implementation(content, "HashMap"),
            "Should match 'HashMap' from 'impl HashMap<String, i32>'"
        );
    }

    /// Regression test: has_trait_implementation handles nested generics.
    #[test]
    fn has_trait_implementation_nested_generics() {
        let content = "impl Result<Vec<String>, Error> for MyStruct {\n    fn bar(&self) {}\n}";
        assert!(
            has_trait_implementation(content, "Result"),
            "Should match 'Result' from nested generics"
        );
        assert!(
            !has_trait_implementation(content, "Vec"),
            "Should NOT match 'Vec' when it's a nested generic param"
        );
    }

    /// Regression test: has_trait_implementation handles empty string.
    #[test]
    fn has_trait_implementation_empty_content() {
        assert!(!has_trait_implementation("", "SomeTrait"));
    }

    /// Regression test: has_trait_implementation handles non-matching traits.
    #[test]
    fn has_trait_implementation_no_match() {
        let content = "impl Vec<String> for MyStruct {\n    fn foo(&self) {}\n}";
        assert!(
            !has_trait_implementation(content, "HashMap"),
            "Should NOT match 'HashMap' when impl is for 'Vec'"
        );
    }
}
