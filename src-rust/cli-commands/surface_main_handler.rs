use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

pub struct MainHandlerSurface {
    _container: Option<Box<dyn ServiceContainerAggregate>>,
}

impl MainHandlerSurface {
    pub fn new(container: Option<Box<dyn ServiceContainerAggregate>>) -> Self {
        Self {
            _container: container,
        }
    }

    pub fn execute(&self) {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::ERROR.into()),
            )
            .init();
    }
}

pub fn run_cli_entry() {
    let surface = MainHandlerSurface::new(None);
    surface.execute();
}
