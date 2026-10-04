use std::collections::HashMap;

use shared_common::taxonomy_common_vo::{BooleanVO, Count};
use shared_common::taxonomy_definition_vo::{LayerDefinition, NamingConfig};
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_paths_vo::FilePathList;
use shared_config_system::taxonomy_config_system_vo::{ArchitectureConfig, ArchitectureRule};
use shared_config_system::utility_config_merger::merge_config;
use shared_config_system::utility_config_parser::default_aes_config;

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
        forbidden: shared_common::taxonomy_common_vo::PatternList {
            values: vec!["capabilities".to_string()],
        },
        ..Default::default()
    };
    let config = make_config(layers, vec![rule]);
    let (merged, _) = merge_config(&config);
    assert!(
        merged[&LayerNameVO::new("agent")]
            .forbidden
            .values
            .contains(&"capabilities".to_string())
    );
}
