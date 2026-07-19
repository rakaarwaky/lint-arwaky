use regex::Regex;

pub fn has_rust_impl(content: &str, rust_impl_pattern: &str, re_trait: &Regex) -> bool {
    content.contains(rust_impl_pattern)
        || content.lines().any(|ln| {
            let t = ln.trim();
            t.starts_with("impl") && re_trait.is_match(t) && t.contains(" for")
        })
}

pub fn has_rust_call(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') {
            continue;
        }
        if (t.starts_with("use ")
            || t.contains("::")
            || t.contains("<dyn ")
            || t.contains("Arc<dyn "))
            && re_trait.is_match(t)
        {
            return true;
        }
    }
    false
}

pub fn has_rust_wire(content: &str, re_trait: &Regex) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') {
            continue;
        }
        if !re_trait.is_match(t) {
            continue;
        }

        // Check within a 30-line window for direct wiring (Arc::new, Box::new)
        let end = std::cmp::min(i + 30, total);
        if lines[i..end].iter().any(|&ln| {
            let tl = ln.trim();
            !tl.starts_with("//")
                && !tl.starts_with("/*")
                && !tl.starts_with('*')
                && (tl.contains("Arc::new(") || tl.contains("Box::new("))
        }) {
            return true;
        }

        // For constructor injection patterns: if trait appears in a context other than
        // an impl block (e.g., function parameter, use statement), scan the entire file
        // for Arc::new(Type) where Type implements this trait. This covers cases where
        // the trait parameter and the Arc::new call are far apart in the same container.
        let is_impl_line = t.contains("impl") && t.contains("for") && re_trait.is_match(t);
        if !is_impl_line {
            // Check if there's any impl <Trait> for somewhere in the file
            let has_impl = lines.iter().any(|&ln| {
                let lt = ln.trim();
                !lt.starts_with("//")
                    && !lt.starts_with("/*")
                    && re_trait.is_match(lt)
                    && lt.contains("impl")
                    && lt.contains("for")
            });
            if has_impl {
                // Check if there's any Arc::new( or Box::new( anywhere in the file
                let has_new = lines.iter().any(|&ln| {
                    let tl = ln.trim();
                    !tl.starts_with("//")
                        && !tl.starts_with("/*")
                        && (tl.contains("Arc::new(") || tl.contains("Box::new("))
                });
                if has_new {
                    return true;
                }
            }
        }
    }
    false
}

pub fn has_py_impl(content: &str, trait_name: &str) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            continue;
        }
        if t.starts_with("class ") && t.contains(trait_name) && t.contains('(') {
            return true;
        }
    }
    false
}

pub fn has_py_call(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            continue;
        }
        if (t.starts_with("from ") || t.starts_with("import ")) && re_trait.is_match(t) {
            return true;
        }
        if re_trait.is_match(t) && (t.contains('.') || t.contains(": ")) {
            return true;
        }
    }
    false
}

pub fn has_py_wire(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            continue;
        }
        if re_trait.is_match(t) && t.contains('(') && !t.starts_with("class ") {
            return true;
        }
    }
    false
}

pub fn has_ts_impl(content: &str, trait_name: &str) -> bool {
    let re = super::taxonomy_contract_regex_utility::word_boundary_re(trait_name);
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        if (t.contains("implements ") || t.contains("extends ")) && re.is_match(t) {
            return true;
        }
    }
    false
}

pub fn has_ts_call(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        if t.starts_with("import ") && re_trait.is_match(t) {
            return true;
        }
        if re_trait.is_match(t) && (t.contains('.') || t.contains(": ")) {
            return true;
        }
    }
    false
}

pub fn has_ts_wire(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        if re_trait.is_match(t) && t.contains("new ") && t.contains('(') {
            return true;
        }
    }
    false
}

pub fn strip_comments(content: &str, ext: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if in_block_comment {
            if trimmed.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }

        if ext == "rs" {
            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                if trimmed.starts_with("/*") && !trimmed.contains("*/") {
                    in_block_comment = true;
                }
                continue;
            }
            let code_line = if let Some(pos) = line.find("//") {
                &line[..pos]
            } else {
                line
            };
            result.push_str(code_line);
            result.push('\n');
            continue;
        }

        if ext == "py" {
            if trimmed.starts_with('#') {
                continue;
            }
            let code_line = if let Some(pos) = line.find('#') {
                &line[..pos]
            } else {
                line
            };
            result.push_str(code_line);
            result.push('\n');
            continue;
        }

        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            if trimmed.starts_with("/*") && !trimmed.contains("*/") {
                in_block_comment = true;
            }
            continue;
        }
        let code_line = if let Some(pos) = line.find("//") {
            &line[..pos]
        } else {
            line
        };
        result.push_str(code_line);
        result.push('\n');
    }

    result
}

pub fn extract_contract_trait_name(content: &str, file_path: &str) -> Option<String> {
    let ext = std::path::Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let code = strip_comments(content, ext);

    match ext {
        "rs" => super::taxonomy_contract_regex_utility::re_contract_rust()
            .and_then(|re| re.captures(&code))
            .map(|caps| caps[1].to_string()),
        "py" => {
            if let Some(caps) = super::taxonomy_contract_regex_utility::re_contract_py()
                .and_then(|re| re.captures(&code))
            {
                caps.get(1)
                    .or_else(|| caps.get(2))
                    .map(|m| m.as_str().to_string())
            } else {
                super::taxonomy_contract_regex_utility::re_contract_py_fallback()
                    .and_then(|re| re.captures(&code))
                    .map(|caps| caps[1].to_string())
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            super::taxonomy_contract_regex_utility::re_ts_interface_export()
                .and_then(|re| re.captures(&code))
                .or_else(|| {
                    super::taxonomy_contract_regex_utility::re_interface()
                        .and_then(|re| re.captures(&code))
                })
                .map(|caps| caps[1].to_string())
        }
        _ => None,
    }
}
