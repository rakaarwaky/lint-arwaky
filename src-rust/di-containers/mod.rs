// PURPOSE: Module declarations and re-exports for di-containers (aggregates, routing VOs, containers)
pub mod agent_checker_container;
pub use agent_checker_container::CheckerContainer;
pub mod agent_registry_container;
pub use agent_registry_container::ProjectContainerRegistry;
pub mod agent_injection_container;
pub use agent_injection_container::{Container, DependencyInjectionContainer};
pub mod contract_adapter_aggregate;
pub use contract_adapter_aggregate::AdapterContainerAggregate;
pub mod contract_capability_aggregate;
pub use contract_capability_aggregate::CapabilityContainerAggregate;
pub mod contract_infra_aggregate;
pub use contract_infra_aggregate::InfrastructureContainerAggregate;
pub mod contract_orchestrator_aggregate;
pub use contract_orchestrator_aggregate::OrchestratorContainerAggregate;
pub mod contract_registry_aggregate;
pub use contract_registry_aggregate::ContainerRegistryAggregate;
pub mod contract_service_aggregate;
pub use contract_service_aggregate::{DefaultServiceContainer, ServiceContainerAggregate};
pub mod taxonomy_routing_vo;
pub use taxonomy_routing_vo::{
    CapabilityReference, CapabilityReferenceList, CapabilityRoutingContext, ClassDefinitionMap,
    ClassFileMap, ClassMethodsVO, ClassNameVO, ClassUsageItem, ClassUsageItemList, ClassUsageMap,
};
