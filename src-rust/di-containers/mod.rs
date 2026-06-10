// PURPOSE: Module declarations and re-exports for di-containers (aggregates, routing VOs, containers)
pub mod agent_checker_container;
pub use agent_checker_container::CheckerContainer;
pub mod agent_registry_container;
pub use agent_registry_container::ProjectContainerRegistry;
pub mod agent_injection_container;
pub use agent_injection_container::{Container, DependencyInjectionContainer};
pub mod contract_registry_aggregate;
pub use contract_registry_aggregate::ContainerRegistryAggregate;
pub mod contract_service_aggregate;
pub use contract_service_aggregate::{DefaultServiceContainer, ServiceContainerAggregate};
