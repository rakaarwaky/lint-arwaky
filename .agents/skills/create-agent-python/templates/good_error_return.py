# GOOD: Return Result type for execution orchestration
def run(self, request: ScanRequestVO) -> Result:
    # Result[ExecutionReport, AgentExecutionError]
    ...
