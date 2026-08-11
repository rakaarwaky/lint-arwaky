// PURPOSE: AdapterNameList — value object for a list of adapter names
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::list_wrapper_vo;

list_wrapper_vo!(AdapterNameList, AdapterName);

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
