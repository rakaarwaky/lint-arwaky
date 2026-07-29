def execute(self, request: <ScanRequest>VO) -> list[<ResultVO>]:
    results: list[<ResultVO>] = []
    for file in request.files():
        try:
            result = self.analyzer.analyze(file)
            results.extend(result.into_results())
        except Exception as e:
            results.append(<ResultVO>.from_analysis_error(file, e))
    return results
