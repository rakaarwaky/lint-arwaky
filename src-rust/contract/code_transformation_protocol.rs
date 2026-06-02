use super::*;

pub trait ICodeTransformationProtocol: Send + Sync {
    fn rename_symbol(&self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName) -> Count;
}
