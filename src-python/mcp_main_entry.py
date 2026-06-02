"""Bootstrap entry for auto-linter MCP server."""


def main() -> None:
    """Entry point for the auto-linter MCP server."""
    from .agent.agent_container_registry import get_container
    from .surfaces.mcp_server_handler import McpServerHandlerSurface

    # 1. Retrieve the system-wide Dependency Injection container.
    # Entry points are responsible for the first wire-up.
    container = get_container()

    # 2. Instantiate the MCP surface and pass the container as an Aggregate contract.
    surface = McpServerHandlerSurface()

    # 3. Execute the server loop.
    surface.run_server(container)


if __name__ == "__main__":
    main()
