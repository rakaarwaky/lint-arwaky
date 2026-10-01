// PURPOSE: AdapterNameList — value object for a list of adapter names
use crate::list_wrapper_vo;
use crate::taxonomy_adapter_name_vo::AdapterName;

list_wrapper_vo!(AdapterNameList, AdapterName);

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
