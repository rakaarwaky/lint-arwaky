use crate::agent::dependency_injection_container::Container;
use crate::surfaces::cli_main_handler::MainHandlerSurface;

pub fn run_cli() {
    let container = Container::new();
    let surface = MainHandlerSurface::new(container);
    surface.execute();
}
