use once_cell::sync::OnceCell;
use regex::Regex;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;
use shared::taxonomy_layer_vo::Identity;

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

// ─── Block 1: Struct Definition ───────────────────────────
pub struct OrphanFilenameExtractor;

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl IOrphanFilenameExtractorProtocol for OrphanFilenameExtractor {
    fn file_basename(&self, fp: &FilePath) -> Identity {
        Identity::new(fp.basename())
    }

    fn file_stem(&self, fp: &FilePath) -> Identity {
        Identity::new(fp.stem())
    }

    fn file_suffix(&self, fp: &FilePath) -> Identity {
        Identity::new(fp.suffix())
    }

    fn extract_struct_names(&self, content: &str) -> Vec<SymbolName> {
        let mut names = Vec::new();
        if let Some(re) = struct_re() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name != "Self" && !name.is_empty() {
                    names.push(SymbolName::new(name));
                }
            }
        }
        names
    }

    fn extract_trait_names(&self, content: &str) -> Vec<SymbolName> {
        let mut names = Vec::new();
        if let Some(re) = trait_re() {
            for cap in re.captures_iter(content) {
                names.push(SymbolName::new(cap[1].to_string()));
            }
        }
        names
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for OrphanFilenameExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanFilenameExtractor {
    pub fn new() -> Self {
        Self
    }
}
