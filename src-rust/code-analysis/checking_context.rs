use crate::code_analysis::capabilities_class_checker::ArchClassChecker;
use crate::code_analysis::capabilities_constant_checker::ArchConstantChecker;
use crate::code_analysis::capabilities_line_checker::ArchLineChecker;
use crate::code_analysis::capabilities_primitive_checker::ArchPrimitiveChecker;
use crate::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use crate::code_analysis::contract_constant_protocol::IConstantPurityProtocol;
use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::code_analysis::contract_primitive_protocol::IPrimitiveCheckerProtocol;
use crate::layer_rules::capabilities_compliance_analyzer::ArchComplianceAnalyzer;
use crate::layer_rules::capabilities_import_checker::ArchImportRuleChecker;
use crate::layer_rules::capabilities_internal_checker::ArchInternalChecker;
use crate::layer_rules::capabilities_layer_checker::ArchLayerChecker;
use crate::layer_rules::contract_import_protocol::IArchImportProtocol;
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::layer_rules::contract_rule_protocol::IInternalCheckerProtocol;
use crate::naming_rules::capabilities_naming_checker::ArchNamingChecker;
use crate::naming_rules::contract_provider_port::INamingProviderPort;
use crate::orphan_detector::capabilities_orphan_analyzer::OrphanGraphResolver;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

pub struct CheckingContext {
    pub analyzer: ArchComplianceAnalyzer,
    pub import_checker: ArchImportRuleChecker,
    pub internal_checker: ArchInternalChecker,
    pub naming_checker: ArchNamingChecker,
    pub line_checker: ArchLineChecker,
    pub constant_checker: ArchConstantChecker,
    pub class_checker: ArchClassChecker,
    pub primitive_checker: ArchPrimitiveChecker,
    pub layer_checker: ArchLayerChecker,
    pub orphan_resolver: OrphanGraphResolver,
}

impl CheckingContext {
    pub fn new(config: ArchitectureConfig) -> Self {
        Self {
            analyzer: ArchComplianceAnalyzer::new(config),
            import_checker: ArchImportRuleChecker::new(),
            internal_checker: ArchInternalChecker::new(),
            naming_checker: ArchNamingChecker::new(),
            line_checker: ArchLineChecker::new(),
            constant_checker: ArchConstantChecker::new(),
            class_checker: ArchClassChecker::new(),
            primitive_checker: ArchPrimitiveChecker::new(),
            layer_checker: ArchLayerChecker::new(),
            orphan_resolver: OrphanGraphResolver::new(),
        }
    }
}
