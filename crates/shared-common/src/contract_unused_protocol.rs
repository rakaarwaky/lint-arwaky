// PURPOSE: IUnusedProtocol — port trait for AES023: find unused imports in a file

/* UNKNOWN: SymbolName */
use shared_common::taxonomy_name_vo::SymbolName;
use source_parsing::taxonomy_path_vo::FilePath;

pub trait IUnusedProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<SymbolName>;
}
