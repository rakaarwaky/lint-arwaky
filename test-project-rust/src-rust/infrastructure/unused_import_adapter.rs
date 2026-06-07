use crate::taxonomy::base_vo::BaseVo;
pub struct UnusedImportAdapter;
impl UnusedImportAdapter {
    pub fn label(&self) -> &'static str { "taxonomy" }
}
