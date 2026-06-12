# AES025: MCP tool without input schema

def register_tool():
    pass


class McpToolBad:
    def __init__(self):
        self.server = None

    def add_tool(self):
        server.add_tool(register_tool)

    def mcp_serve(self):
        self.add_tool()
