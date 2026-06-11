// PURPOSE: LanguageContainer — wiring for language-adapters feature (root layer, wiring only)
use std::sync::Arc;

pub struct LanguageContainer {
    javascript_flow_port: Arc<dyn crate::language_adapters::contract_flow_port::IJavascriptFlowPort>,
    naming_provider_port: Arc<dyn crate::language_adapters::contract_naming_port::INamingProviderPort>,
    javascript_scope_port: Arc<dyn crate::language_adapters::contract_scope_port::IJavascriptScopePort>,
    semantic_tracer_port: Arc<dyn crate::language_adapters::contract_semantic_tracer_port::ISemanticTracerPort>,
    naming_variant_port: Arc<dyn crate::language_adapters::contract_variant_port::INamingVariantPort>,
}

impl LanguageContainer {
    pub fn new() -> Self {
        let variant: Arc<dyn crate::language_adapters::contract_variant_port::INamingVariantPort> = Arc::new(
            crate::language_adapters::infrastructure_py_variants::PythonNamingVariantProvider::new(),
        );
        Self {
            javascript_flow_port: Arc::new(
                crate::language_adapters::infrastructure_js_flow_tracer::JSFlowAdapter::new(),
            ),
            naming_provider_port: Arc::new(
                crate::language_adapters::infrastructure_js_naming_provider::JavascriptNamingProvider::new(),
            ),
            javascript_scope_port: Arc::new(
                crate::language_adapters::infrastructure_js_scope_provider::JSScopeProvider::new(),
            ),
            semantic_tracer_port: Arc::new(
                crate::language_adapters::infrastructure_py_ast_tracer::PythonTracer::new(
                    Box::new(crate::language_adapters::infrastructure_py_variants::PythonNamingVariantProvider::new()),
                ),
            ),
            naming_variant_port: variant,
        }
    }

    pub fn javascript_flow_port(&self) -> Arc<dyn crate::language_adapters::contract_flow_port::IJavascriptFlowPort> {
        self.javascript_flow_port.clone()
    }

    pub fn naming_provider_port(&self) -> Arc<dyn crate::language_adapters::contract_naming_port::INamingProviderPort> {
        self.naming_provider_port.clone()
    }

    pub fn javascript_scope_port(&self) -> Arc<dyn crate::language_adapters::contract_scope_port::IJavascriptScopePort> {
        self.javascript_scope_port.clone()
    }

    pub fn semantic_tracer_port(&self) -> Arc<dyn crate::language_adapters::contract_semantic_tracer_port::ISemanticTracerPort> {
        self.semantic_tracer_port.clone()
    }

    pub fn naming_variant_port(&self) -> Arc<dyn crate::language_adapters::contract_variant_port::INamingVariantPort> {
        self.naming_variant_port.clone()
    }
}
impl Default for LanguageContainer {
    fn default() -> Self {
        Self::new()
    }
}

