pub use self as code_analysis;
pub use ::taxonomy;
pub use ::layer_rules as contract;

pub mod capabilities_deep_import_processor;
pub mod capabilities_forbidden_suffix_vo;
pub mod capabilities_forbidden_trait_processor;
pub mod capabilities_many_functions_processor;
pub mod capabilities_mcp_tool_processor;
pub mod capabilities_no_import_processor;
pub mod capabilities_unmatched_struct_processor;
pub mod capabilities_unused_protocol_processor;
pub mod capabilities_wrong_suffix_use;
pub mod badname;
