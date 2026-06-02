"""Bootstrap entry for auto-lint CLI."""


def main() -> None:
    """Entry point for the pip-installed ``auto-lint`` command."""
    from .agent.dependency_injection_container import Container
    from .surfaces.cli_main_handler import MainHandlerSurface

    # 1. Initialize the system-wide Dependency Injection container.
    # The entry point is the only place allowed to touch concrete containers.
    container = Container()

    # 2. Instantiate the surface handler and inject the container as a contract.
    surface = MainHandlerSurface(container=container)

    # 3. Hand off control to the surface execution logic.
    surface.execute()


if __name__ == "__main__":
    main()
