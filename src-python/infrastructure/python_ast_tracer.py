import ast
import logging
import os
import re
import glob


from ..taxonomy import (
    Count,
    DirectoryPath,
    FilePath,
    LineNumber,
    ResponseData,
    ScopeRef,
    SymbolName,
    SymbolNameList,
    CallChainList,
    DataFlowList,
    ResponseDataList,
)
from ..contract import ISemanticTracerPort
from .naming_variant_provider import PythonNamingVariantProvider

logger = logging.getLogger(__name__)


class _ScopeVisitor(ast.NodeVisitor):
    def __init__(self, target):
        self.target = target
        self.current_path = []
        self.best_match = []

    def generic_visit(self, node: ast.AST):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            start = getattr(node, "lineno", None)
            end = getattr(node, "end_lineno", None)
            if start and end and start <= self.target <= end:
                prefix = "class " if isinstance(node, ast.ClassDef) else "def "
                self.current_path.append(f"{prefix}{getattr(node, 'name', 'unknown')}")
                self.best_match = list(self.current_path)
                super().generic_visit(node)
                self.current_path.pop()
                return
        super().generic_visit(node)


class _SymbolLocationVisitor(ast.NodeVisitor):
    def __init__(self, sym, lines):
        self.sym = sym
        self.lines = lines
        self.occurrences = []

    def visit_Name(self, node: ast.Name):
        if node.id == self.sym:
            action = "assignment" if isinstance(node.ctx, ast.Store) else "usage"
            self._add(node, action)
        self.generic_visit(node)

    def visit_FunctionDef(self, node: ast.FunctionDef):
        if node.name == self.sym:
            self._add(node, "definition (function)")
        self.generic_visit(node)

    def visit_ClassDef(self, node: ast.ClassDef):
        if node.name == self.sym:
            self._add(node, "definition (class)")
        self.generic_visit(node)

    def visit_Attribute(self, node: ast.Attribute):
        if isinstance(node.value, ast.Name) and node.value.id == self.sym:
            self._add(node, f"access member '{node.attr}'")
        elif node.attr == self.sym:
            base = "assignment" if isinstance(node.ctx, ast.Store) else "usage"
            self._add(node, base + " (attribute)")
        self.generic_visit(node)

    def _add(self, node, action):
        line = getattr(node, "lineno", 0)
        if line:
            self.occurrences.append(
                {
                    "line": line,
                    "column": getattr(node, "col_offset", 0),
                    "action": action,
                    "context": self.lines[line - 1].strip(),
                }
            )


class _FlowVisitor(ast.NodeVisitor):
    def __init__(self, var, lines):
        self.var = var
        self.lines = lines
        self.flows = []

    def visit_Name(self, node: ast.Name):
        if node.id == self.var:
            line = getattr(node, "lineno", 0)
            if line:
                tag = "Assignment" if isinstance(node.ctx, ast.Store) else "Usage"
                self.flows.append(
                    f"Line {line} [{tag}]: {self.lines[line - 1].strip()}"
                )
        self.generic_visit(node)


class PythonTracer(ISemanticTracerPort):
    """AST-based tracer for Python code to enrich lint context."""

    def get_variant_dict(self, name: SymbolName) -> ResponseData:
        return PythonNamingVariantProvider().get_variant_dict(name)

    def build_variants(self, name: SymbolName) -> SymbolNameList:
        values = [
            SymbolName(value=v)
            for v in PythonNamingVariantProvider().build_variants(name)
        ]
        return SymbolNameList(values=values)

    def get_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | None:
        try:
            with open(str(file_path), "r", encoding="utf-8") as f:
                tree = ast.parse(f.read())
            v = _ScopeVisitor(int(line))
            v.visit(tree)
            if v.best_match:
                return ScopeRef(name=" -> ".join(v.best_match))
            return None
        except Exception:
            return None

    def get_symbol_locations(
        self, file_path: FilePath, symbol: SymbolName
    ) -> ResponseDataList:
        try:
            with open(str(file_path), "r", encoding="utf-8") as f:
                lines = f.readlines()
            v = _SymbolLocationVisitor(str(symbol), lines)
            v.visit(ast.parse("".join(lines)))
            v.occurrences.sort(key=lambda x: x["line"])
            return ResponseDataList(
                values=[ResponseData(value=o) for o in v.occurrences]
            )
        except Exception:
            return ResponseDataList(values=[])

    def find_flow(
        self, file_path: FilePath, var_name: SymbolName, start_line: LineNumber
    ) -> DataFlowList:
        try:
            with open(str(file_path), "r", encoding="utf-8") as f:
                lines = f.readlines()
            content = "".join(lines)
            tree = ast.parse(content)
            target: ast.AST = tree
            for node in ast.walk(tree):
                if isinstance(
                    node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
                ):
                    if (
                        getattr(node, "lineno", 0)
                        <= int(start_line)
                        <= getattr(node, "end_lineno", 0)
                    ):
                        target = node
            v = _FlowVisitor(str(var_name), lines)
            v.visit(target)
            res = list(dict.fromkeys(v.flows))
            res.sort(key=lambda s: int(s.split("Line ")[1].split(" ")[0]))
            return DataFlowList(values=res)
        except Exception:
            return DataFlowList(values=[])

    def trace_call_chain(
        self, root_dir: DirectoryPath, target_name: SymbolName
    ) -> CallChainList:
        callers = []
        root_str = str(root_dir)
        target_str = str(target_name)
        pattern = re.compile(rf"\b{re.escape(target_str)}\(")
        for path in glob.glob(os.path.join(root_str, "**", "*.py"), recursive=True):
            try:
                with open(path, "r", encoding="utf-8", errors="ignore") as f:
                    for i, line in enumerate(f):
                        if pattern.search(line):
                            rel = os.path.relpath(path, root_str)
                            callers.append(f"{rel}:{i + 1}: {line.strip()}")
            except Exception:
                logger.debug("Failed to read file %s in call chain search", path)
                continue
        return CallChainList(values=callers)

    def project_wide_rename(
        self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName
    ) -> Count:
        root_str = str(root_dir)
        old_str = str(old_name)
        new_str = str(new_name)
        mod_count = 0
        p = re.compile(
            r"(\"\"\"(?:.|\\n)*?\"\"\"|\'\'\'(?:.|\\n)*?\'\'\'|\"(?:\\.|[^\"\\\\])*\"|\'(?:\\.|[^\'\\\\])*\'|\#[^\n]*)| \b"
            + re.escape(old_str)
            + r"\b",
            re.VERBOSE | re.DOTALL,
        )
        for path in glob.glob(os.path.join(root_str, "**", "*.py"), recursive=True):
            try:
                with open(path, "r", encoding="utf-8") as f:
                    content = f.read()
                if old_str in content:
                    new_content = p.sub(
                        lambda m: m.group(1) if m.group(1) else new_str, content
                    )
                    if new_content != content:
                        with open(path, "w", encoding="utf-8") as f:
                            f.write(new_content)
                        mod_count += 1
            except Exception:
                logger.debug("Failed to rename in file %s", path)
                continue
        return Count(value=mod_count)
