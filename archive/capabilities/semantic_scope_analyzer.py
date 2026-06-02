"""semantic_scope_analyzer — AST-based semantic scope analysis capability."""

from __future__ import annotations
from ..taxonomy import (
    CallChainList,
    Count,
    DataFlowList,
    DirectoryPath,
    FileContentVO,
    FilePath,
    LineNumber,
    ResponseData,
    ResponseDataList,
    ScopeRef,
    SymbolName,
    SymbolNameList,
)

import ast
import re

from ..contract import ISemanticTracerProtocol
from ..contract import IFileSystemPort


class SemanticScopeAnalyzer(ISemanticTracerProtocol):
    """AST-based semantic scope analyzer for Python code."""

    def __init__(self, fs: IFileSystemPort):
        self._fs = fs

    def get_variant_dict(self, name: SymbolName) -> ResponseData:
        n = str(name)
        words = re.findall(r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+", n)
        words = [w.lower() for w in words]
        if not words:
            return ResponseData(
                value={
                    "snake_case": n,
                    "pascal_case": n,
                    "camel_case": n,
                    "screaming_snake": n.upper(),
                }
            )
        snake_case = "_".join(words)
        _rest = "".join(str(w).capitalize() for w in words[1:])
        return ResponseData(
            value={
                "snake_case": snake_case,
                "camel_case": str(words[0]) + _rest,
                "pascal_case": "".join(str(w).capitalize() for w in words),
                "screaming_snake": snake_case.upper(),
            }
        )

    def build_variants(self, name: SymbolName) -> SymbolNameList:
        n = str(name)
        d = self.get_variant_dict(name)
        kebab = d.value["snake_case"].replace("_", "-")
        variants = list(
            {
                n,
                d.value["snake_case"],
                d.value["camel_case"],
                d.value["pascal_case"],
                d.value["screaming_snake"],
                kebab,
            }
        )
        return SymbolNameList(values=[SymbolName(value=v) for v in variants])

    def get_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | None:
        if not self._fs.exists(file_path).value:
            return None

        source_vo: FileContentVO = self._fs.read_text(file_path)
        source = source_vo.value

        try:
            tree = ast.parse(source)
        except SyntaxError:
            return None

        class ScopeVisitor(ast.NodeVisitor):
            def __init__(self, target_line: LineNumber):
                self.target_line = int(target_line)
                self.current_path: list[str] = []
                self.best_match: list[str] = []

            def generic_visit(self, node):
                is_scope = isinstance(
                    node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
                )
                if is_scope:
                    if hasattr(node, "lineno") and hasattr(node, "end_lineno"):
                        if node.lineno <= self.target_line <= node.end_lineno:
                            scope_name = (
                                f"class {node.name}"
                                if isinstance(node, ast.ClassDef)
                                else f"def {node.name}"
                            )
                            self.current_path.append(scope_name)
                            self.best_match = list(self.current_path)
                            super().generic_visit(node)
                            self.current_path.pop()
                            return
                super().generic_visit(node)

        visitor = ScopeVisitor(line)
        visitor.visit(tree)
        if visitor.best_match:
            return ScopeRef(name=" -> ".join(visitor.best_match), kind="")
        return None

    def get_symbol_locations(
        self, file_path: FilePath, symbol: SymbolName
    ) -> ResponseDataList:
        """Stub for symbol location retrieval."""
        return ResponseDataList(values=[])

    def find_flow(
        self,
        file_path: FilePath,
        var_name: SymbolName,
        start_line: LineNumber = LineNumber(value=0),
    ) -> DataFlowList:
        if not self._fs.exists(file_path).value:
            return DataFlowList(values=[])

        source_vo: FileContentVO = self._fs.read_text(file_path)
        source = source_vo.value
        lines = source.splitlines(keepends=True)

        try:
            tree = ast.parse(source)
        except SyntaxError:
            return DataFlowList(values=[])

        vn = str(var_name)
        sl = int(start_line)
        flows = []
        target_scope: ast.AST | None = tree

        if sl > 0:

            class TargetScopeVisitor(ast.NodeVisitor):
                def __init__(self, target: LineNumber):
                    self.target = int(target)
                    self.node: ast.AST | None = None

                def generic_visit(self, node: ast.AST):
                    if isinstance(
                        node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
                    ):
                        lineno = getattr(node, "lineno", None)
                        end_lineno = getattr(node, "end_lineno", None)
                        if lineno is not None and end_lineno is not None:
                            if lineno <= self.target <= end_lineno:
                                self.node = node
                    super().generic_visit(node)

            tsv = TargetScopeVisitor(start_line)
            tsv.visit(tree)
            if tsv.node is not None:
                target_scope = tsv.node

        class FlowVisitor(ast.NodeVisitor):
            def visit_Name(self, node):
                if node.id == vn:
                    if hasattr(node, "lineno"):
                        line_text = lines[node.lineno - 1].strip()
                        if isinstance(node.ctx, ast.Store):
                            flows.append(
                                f"Line {node.lineno} [Assignment]: {line_text}"
                            )
                        elif isinstance(node.ctx, ast.Load):
                            flows.append(f"Line {node.lineno} [Usage]: {line_text}")
                self.generic_visit(node)

            def visit_Call(self, node):
                if isinstance(node.func, ast.Attribute) and isinstance(
                    node.func.value, ast.Name
                ):
                    if node.func.value.id == vn:
                        if hasattr(node, "lineno"):
                            line_text = lines[node.lineno - 1].strip()
                            flows.append(
                                f"Line {node.lineno} [Mutation '{node.func.attr}']: {line_text}"
                            )
                self.generic_visit(node)

        if target_scope is not None:
            FlowVisitor().visit(target_scope)

        unique_flows = list(dict.fromkeys(flows))
        unique_flows.sort(key=self._extract_lineno)
        return DataFlowList(values=unique_flows)

    def _extract_lineno(self, fstr: str) -> int:
        """Helper to extract line number from flow string."""
        try:
            return int(fstr.split("Line ")[1].split(" ")[0])
        except Exception:
            return 0

    def trace_call_chain(
        self, root_dir: DirectoryPath, target_name: SymbolName
    ) -> CallChainList:
        name = str(target_name)
        callers = []
        pattern = re.compile(rf"\b{name}\(")

        # Using the tool (infrastructure) to find files
        py_files = self._fs.glob(f"{root_dir}/**/*.py")

        for filepath in py_files:
            source_vo = self._fs.read_text(filepath)
            lines = source_vo.value.splitlines()

            for i, line in enumerate(lines):
                if pattern.search(line) and f"def {name}" not in line:
                    rel_path = self._fs.get_relative_path(
                        filepath, FilePath(value=str(root_dir))
                    )
                    callers.append(f"{rel_path}:{i + 1} -> {line.strip()}")
        return CallChainList(values=callers)

    def project_wide_rename(
        self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName
    ) -> Count:
        old = str(old_name)
        new = str(new_name)
        pattern = re.compile(
            rf"""
            (
                \"\"\"(?:\\.|[^\\])*?\"\"\" |
                \'\'\'(?:\\.|[^\\])*?\'\'\' |
                \"(?:\\.|[^\"\\])*\" |
                \'(?:\\.|[^\'\\])*\' |
                \#[^\n]*
            )
            |
            \b({re.escape(old)})\b
        """,
            re.VERBOSE | re.DOTALL,
        )

        def replacer(match):
            if match.group(1) is not None:
                return match.group(1)
            return new

        py_files = self._fs.glob(f"{root_dir}/**/*.py")
        modified_count: int = 0
        for filepath in py_files:
            source_vo = self._fs.read_text(filepath)
            source = source_vo.value

            if old in source:
                new_source = pattern.sub(replacer, source)
                if new_source != source:
                    res = self._fs.write_text(filepath, FileContentVO(value=new_source))
                    if res.value:
                        modified_count += 1
        return Count(value=modified_count)
