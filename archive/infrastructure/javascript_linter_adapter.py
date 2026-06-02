from ..taxonomy import (
    AdapterName,
    ColumnNumber,
    ComplianceStatus,
    ErrorCode,
    FilePath,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    PatternList,
    Severity,
    ScanError,
    AdapterError,
    ErrorMessage,
)

import json
import logging
import os
import re

from ..contract import ILinterAdapterPort, ICommandExecutorPort, IPathNormalizationPort

logger = logging.getLogger("mcp.adapters.javascript")


def _resolve_working_dir(path: FilePath) -> FilePath:
    path_str = str(path)
    try:
        from pathlib import Path
        current = Path(os.path.abspath(path_str))
        if current.is_file():
            current = current.parent
        for _ in range(10):
            if (
                (current / "auto_linter.config.python.yaml").is_file() or (current / "auto_linter.config.javascript.yaml").is_file() or (current / "auto_linter.config.rust.yaml").is_file()
                or (current / "auto_linter.config.python.yaml").is_file()
                or (current / "auto_linter.config.javascript.yaml").is_file()
                or (current / "auto_linter.config.rust.yaml").is_file()
                or (current / "package.json").is_file()
                or (current / ".git").is_dir()
            ):
                return FilePath(value=str(current))
            if current.parent == current:
                break
            current = current.parent
    except Exception as e:
        logger.debug(f"Error resolving working directory: {e}")
    return FilePath(value=".")


def _resolve_js_cmd(executable: str, args: list[str], working_dir: str = ".") -> list[str]:
    """Resolve JS executable path: check global PATH, local node_modules up to root, fallback to npx."""
    import shutil

    # 1. Check global PATH
    glob_bin = shutil.which(executable)
    if glob_bin:
        return [glob_bin] + args

    # 2. Check parent directories of working_dir for node_modules/.bin
    curr = os.path.abspath(working_dir)
    while True:
        local_bin = os.path.join(curr, "node_modules", ".bin", executable)
        if os.path.exists(local_bin) and os.path.isfile(local_bin):
            return [local_bin] + args
        parent = os.path.dirname(curr)
        if parent == curr:
            break
        curr = parent

    # 3. Check parent directories of current working directory
    curr = os.path.abspath(os.getcwd())
    while True:
        local_bin = os.path.join(curr, "node_modules", ".bin", executable)
        if os.path.exists(local_bin) and os.path.isfile(local_bin):
            return [local_bin] + args
        parent = os.path.dirname(curr)
        if parent == curr:
            break
        curr = parent

    # 4. Fallback to npx if it exists in PATH
    if shutil.which("npx"):
        return ["npx", executable] + args

    # Ultimate fallback
    return [executable] + args


class PrettierAdapter(ILinterAdapterPort):
    def __init__(
        self, executor: ICommandExecutorPort, path_norm: IPathNormalizationPort
    ):
        self.executor = executor
        self.path_norm = path_norm

    def name(self) -> AdapterName:
        return AdapterName(value="prettier")

    async def scan(self, path: FilePath) -> LintResultList:
        path_str = str(path)
        if os.path.isfile(path_str) and not path_str.endswith(
            (".ts", ".tsx", ".js", ".jsx", ".json", ".css", ".md", ".yml", ".yaml")
        ):
            return LintResultList()

        results: list[LintResult] = []
        try:
            abs_path = os.path.abspath(path_str)
            wd = _resolve_working_dir(path)
            cmd = _resolve_js_cmd("prettier", ["--check", abs_path], str(wd.value))

            response = await self.executor.execute_command(
                command=PatternList(values=cmd), working_dir=wd
            )
            returncode = response.returncode
            stdout = response.stdout
            stderr = response.stderr

            if returncode == 0:
                return LintResultList(values=[])

            combined_output = (stdout.value + stderr.value).strip()

            if "[warn]" in combined_output:
                filename_vo = self.path_norm.resolve_infrastructure_path(path, path)
                results.append(
                    LintResult(
                        file=filename_vo,
                        line=LineNumber(value=1),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="formatting"),
                        message=LintMessage(
                            value="Code style issues found. Run Prettier to fix."
                        ),
                        source=self.name(),
                        severity=Severity.LOW,
                    )
                )
        except Exception as e:
            logger.error(f"Error running Prettier: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"Prettier execution failed: {e}"),
                adapter_name=self.name()
            )

        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        path_str = str(path)
        try:
            abs_path = os.path.abspath(path_str)
            wd = _resolve_working_dir(path)
            cmd = _resolve_js_cmd("prettier", ["--write", abs_path], str(wd.value))

            response = await self.executor.execute_command(
                command=PatternList(values=cmd), working_dir=wd
            )
            success = response.returncode == 0
            return ComplianceStatus(value=success)
        except Exception as e:
            logger.error(f"Error applying Prettier fixes: {e}", exc_info=True)
            return AdapterError(
                adapter_name=self.name(),
                message=ErrorMessage(value=f"Prettier fix failed: {e}")
            )


class TSCAdapter(ILinterAdapterPort):
    def __init__(
        self, executor: ICommandExecutorPort, path_norm: IPathNormalizationPort
    ):
        self.executor = executor
        self.path_norm = path_norm

    def name(self) -> AdapterName:
        return AdapterName(value="tsc")

    async def scan(self, path: FilePath) -> LintResultList:
        path_str = str(path)
        if os.path.isfile(path_str) and not path_str.endswith((".ts", ".tsx")):
            return LintResultList()

        results: list[LintResult] = []
        try:
            abs_path = os.path.abspath(path_str)
            wd = _resolve_working_dir(path)
            args = ["--noEmit", "--pretty", "false"]
            if abs_path != "." and abs_path != "./":
                args.append(abs_path)
            cmd = _resolve_js_cmd("tsc", args, str(wd.value))

            response = await self.executor.execute_command(
                command=PatternList(values=cmd), working_dir=wd
            )
            output = response.stdout.value + response.stderr.value

            pattern1 = re.compile(
                r"^([^(]+)\((\d+),(\d+)\):\s+error\s+(TS\d+):\s+(.*)$"
            )
            pattern2 = re.compile(
                r"^([^:]+):(\d+):(\d+)\s+-\s+error\s+(TS\d+):\s+(.*)$"
            )

            for line in output.splitlines():
                line = line.strip()
                match = pattern1.match(line) or pattern2.match(line)
                if match:
                    filename, line_num, col_num, code, msg = match.groups()

                    filename_vo = self.path_norm.resolve_infrastructure_path(
                        FilePath(value=filename), path
                    )

                    results.append(
                        LintResult(
                            file=filename_vo,
                            line=LineNumber(value=int(line_num)),
                            column=ColumnNumber(value=int(col_num)),
                            code=ErrorCode(code=code),
                            message=LintMessage(value=msg),
                            source=self.name(),
                            severity=Severity.HIGH,
                        )
                    )
        except Exception as e:
            logger.error(f"Error running TSC: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"TSC execution failed: {e}"),
                adapter_name=self.name()
            )

        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        return ComplianceStatus(value=False)


class ESLintAdapter(ILinterAdapterPort):
    def __init__(
        self, executor: ICommandExecutorPort, path_norm: IPathNormalizationPort
    ):
        self.executor = executor
        self.path_norm = path_norm

    def name(self) -> AdapterName:
        return AdapterName(value="eslint")

    async def scan(self, path: FilePath) -> LintResultList:
        path_str = str(path)
        if os.path.isfile(path_str) and not path_str.endswith(
            (".ts", ".tsx", ".js", ".jsx")
        ):
            return LintResultList()

        results: list[LintResult] = []
        try:
            abs_path = os.path.abspath(path_str)
            wd = _resolve_working_dir(path)
            cmd = _resolve_js_cmd("eslint", [abs_path, "--format", "json"], str(wd.value))

            response = await self.executor.execute_command(
                command=PatternList(values=cmd), working_dir=wd
            )
            stdout = response.stdout

            if not stdout.strip():
                return LintResultList(values=[])

            data = json.loads(stdout)
            for file_data in data:
                filename = file_data["filePath"]
                filename_vo = self.path_norm.resolve_infrastructure_path(
                    FilePath(value=filename), path
                )

                for msg in file_data["messages"]:
                    results.append(
                        LintResult(
                            file=filename_vo,
                            line=LineNumber(value=msg.get("line", 1)),
                            column=ColumnNumber(value=msg.get("column", 0)),
                            code=ErrorCode(code=msg.get("ruleId", "ESLINT")),
                            message=LintMessage(value=msg["message"]),
                            source=self.name(),
                            severity=Severity.HIGH
                            if msg["severity"] == 2
                            else Severity.MEDIUM,
                        )
                    )
        except Exception as e:
            logger.error(f"Error running ESLint: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"ESLint execution failed: {e}"),
                adapter_name=self.name()
            )

        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        path_str = str(path)
        try:
            abs_path = os.path.abspath(path_str)
            wd = _resolve_working_dir(path)
            cmd = _resolve_js_cmd("eslint", [abs_path, "--fix"], str(wd.value))

            response = await self.executor.execute_command(
                command=PatternList(values=cmd), working_dir=wd
            )
            success = response.returncode == 0
            return ComplianceStatus(value=success)
        except Exception as e:
            logger.error(f"Error applying ESLint fixes: {e}", exc_info=True)
            return AdapterError(
                adapter_name=self.name(),
                message=ErrorMessage(value=f"ESLint fix failed: {e}")
            )
