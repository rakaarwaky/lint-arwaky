// PURPOSE: Utility functions for cycle detection — pure functions only, no contract dependencies

pub fn normalize_to_layer(name: &str) -> String {
    let layer_prefixes = [
        "taxonomy_",
        "contract_",
        "capabilities_",
        "utility_",
        "agent_",
        "surface_",
    ];
    let base = match name.rsplit('/').next() {
        Some(b) => b,
        None => name,
    };
    for prefix in &layer_prefixes {
        if base.starts_with(prefix) {
            return prefix.trim_end_matches('_').to_string();
        }
    }
    name.to_string()
}
