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

pub struct OrphanUtility;

impl OrphanUtility {
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
}
