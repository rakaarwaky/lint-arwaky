extern crate shared_lint_arwaky as shared;

use std::collections::HashMap;

use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_definition_vo::{LayerDefinition, NamingConfig};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::{ArchitectureConfig, ArchitectureRule};
use shared::config_system::utility_config_merger::merge_config;

fn make_config(
    layers: HashMap<LayerNameVO, LayerDefinition>,
    rules: Vec<ArchitectureRule>,
) -> ArchitectureConfig {
    ArchitectureConfig {
        enabled: BooleanVO::new(true),
        layers,
        rules,
        naming: NamingConfig::new(Count::new(2)),
        ignored_paths: FilePathList { values: vec![] },
        mandatory_class_definition: BooleanVO::new(false),
    }
}

#[test]
fn merge_empty_config() {
    let config = make_config(HashMap::new(), vec![]);
    let (merged, _) = merge_config(&config);
    assert!(merged.is_empty());
}

#[test]
fn merge_global_rule() {
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("agent"), LayerDefinition::default());
    let rule = ArchitectureRule {
        scope: LayerNameVO::new(""),
        forbidden: shared::common::taxonomy_common_vo::PatternList {
            values: vec!["capabilities".to_string()],
        },
        ..Default::default()
    };
    let config = make_config(layers, vec![rule]);
    let (merged, _) = merge_config(&config);
    assert!(merged[&LayerNameVO::new("agent")]
        .forbidden
        .values
        .contains(&"capabilities".to_string()));
}
