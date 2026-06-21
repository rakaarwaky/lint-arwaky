use regex::Regex;

pub fn extract_struct_names(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?:pub\s+)?struct\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
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
    let re = Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
        for cap in re.captures_iter(content) {
            names.push(cap[1].to_string());
        }
    }
    names
}
