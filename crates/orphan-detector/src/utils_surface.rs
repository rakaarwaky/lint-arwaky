use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;
use std::sync::Arc;

pub fn get_surface_suffix(
    basename: &str,
    extractor: &Arc<dyn IOrphanFilenameExtractorProtocol>,
) -> String {
    extractor
        .file_suffix(&shared::common::taxonomy_path_vo::FilePath {
            value: basename.to_string(),
        })
        .value
}

pub fn surface_category(suffix: &str) -> &'static str {
    match suffix {
        "command" | "controller" | "page" => "smart",
        "hook" | "store" | "action" | "screen" | "router" => "utility",
        "component" | "view" | "layout" => "passive",
        _ => "unknown",
    }
}
