use crate::taxonomy::FilePath;
use crate::taxonomy::SymbolName;


pub trait IUnusedProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<SymbolName>;
}
