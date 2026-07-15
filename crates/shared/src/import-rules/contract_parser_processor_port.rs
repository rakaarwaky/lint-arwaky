// PURPOSE: IParserProcessorPort — contract trait for import parsing
use crate::common::taxonomy_name_vo::SymbolName;

pub trait IParserProcessorPort: Send + Sync {
    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName>;
}
