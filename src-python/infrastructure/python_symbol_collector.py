"""python_symbol_collector — Collector for symbols and imports from Python AST."""

from __future__ import annotations
import ast

from ..taxonomy import (
    MetadataVO,
    InheritanceMap,
    SymbolName,
    SymbolNameList,
    ImportInfo,
    LineNumber,
    ModuleName,
    Count,
)


class SymbolCollector(ast.NodeVisitor):
    """AST visitor that collects defined, used, and exported symbols."""

    def __init__(self) -> None:
        self._defined: set[str] = set()
        self._used: set[str] = set()
        self._exported: set[str] = set()
        self._imported_aliases: dict[str, str] = {}
        self._class_bases: dict[str, list[str]] = {}
        self._imports_list: list[ImportInfo] = []

        # New collections for full port compliance
        self._class_defs: list[dict[str, object]] = []
        self._func_defs: list[dict[str, object]] = []
        self._class_methods: dict[str, list[str]] = {}
        self._assignments: list[dict[str, object]] = []
        self._control_flow_count: int = 0

    @property
    def defined(self) -> SymbolNameList:
        return SymbolNameList(
            values=[SymbolName(value=s) for s in sorted(self._defined)]
        )

    @property
    def used(self) -> SymbolNameList:
        return SymbolNameList(values=[SymbolName(value=s) for s in sorted(self._used)])

    @property
    def exported(self) -> SymbolNameList:
        return SymbolNameList(
            values=[SymbolName(value=s) for s in sorted(self._exported)]
        )

    @property
    def imported_aliases(self) -> MetadataVO:
        return MetadataVO(value=self._imported_aliases)

    @property
    def class_bases(self) -> InheritanceMap:
        return InheritanceMap(mapping=self._class_bases)

    @property
    def imports_list(self) -> list[ImportInfo]:
        return self._imports_list

    @property
    def class_definitions(self) -> list[dict]:
        return self._class_defs

    @property
    def function_definitions(self) -> list[dict]:
        return self._func_defs

    @property
    def class_methods(self) -> dict:
        return self._class_methods

    @property
    def assignments(self) -> list:
        return self._assignments

    @property
    def control_flow_count(self) -> Count:
        return Count(value=self._control_flow_count)

    def visit_Import(self, node: ast.Import) -> None:
        for alias in node.names:
            self._imported_aliases[alias.asname or alias.name] = alias.name
            self._imports_list.append(
                ImportInfo(
                    line=LineNumber(value=node.lineno),
                    module=ModuleName(value=alias.name),
                )
            )
        self.generic_visit(node)

    def visit_ImportFrom(self, node: ast.ImportFrom) -> None:
        module = node.module or ""
        for alias in node.names:
            fullname = f"{module}.{alias.name}" if module else alias.name
            self._imported_aliases[alias.asname or alias.name] = fullname
            self._imports_list.append(
                ImportInfo(
                    line=LineNumber(value=node.lineno),
                    module=ModuleName(value=fullname),
                )
            )
        self.generic_visit(node)

    def visit_FunctionDef(self, node: ast.FunctionDef | ast.AsyncFunctionDef) -> None:
        self._defined.add(node.name)
        self._func_defs.append(
            {"name": node.name, "line": node.lineno, "column": node.col_offset}
        )
        self.generic_visit(node)

    def visit_AsyncFunctionDef(self, node: ast.AsyncFunctionDef) -> None:
        self.visit_FunctionDef(node)

    def visit_ClassDef(self, node: ast.ClassDef) -> None:
        self._defined.add(node.name)
        is_dead = self._is_dead_class(node)
        bases = [ast.unparse(base) for base in node.bases]
        resolved_bases = [self._imported_aliases.get(b, b) for b in bases]

        self._class_defs.append({
            "name": node.name,
            "line": node.lineno,
            "column": node.col_offset,
            "is_dead": is_dead,
            "bases": bases,
            "resolved_bases": resolved_bases,
        })
        self._class_bases[node.name] = resolved_bases
        self._class_methods[node.name] = self._collect_methods(node.body)
        self.generic_visit(node)

    @staticmethod
    def _is_dead_class(node) -> bool:
        """Check if class is dead (empty/compliance marker)."""
        for item in node.body:
            if isinstance(
                item, (ast.FunctionDef, ast.AsyncFunctionDef, ast.Assign, ast.AnnAssign)
            ):
                return False
            if isinstance(item, ast.Expr):
                val = item.value
                is_ellipsis = val is Ellipsis or (
                    hasattr(ast, 'Constant')
                    and isinstance(val, ast.Constant)
                    and val.value is Ellipsis
                )
                if is_ellipsis:
                    continue
                return False
            if not isinstance(item, (ast.Pass, ast.Expr)):
                return False
        return True

    @staticmethod
    def _collect_methods(body) -> list:
        """Extract method names from class body."""
        return [
            item.name
            for item in body
            if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef))
        ]


    def visit_Name(self, node: ast.Name) -> None:
        if isinstance(node.ctx, ast.Load):
            self._used.add(node.id)
        self.generic_visit(node)

    def visit_Assign(self, node: ast.Assign) -> None:
        for target in node.targets:
            if isinstance(target, ast.Name):
                if target.id == "__all__":
                    if isinstance(node.value, (ast.List, ast.Tuple, ast.Set)):
                        for elt in node.value.elts:
                            if isinstance(elt, ast.Constant):
                                value = elt.value
                                if isinstance(value, str):
                                    self._exported.add(value)
                            elif isinstance(
                                elt, ast.Str
                            ):  # Compatibility for older python
                                s_val = elt.s
                                if isinstance(s_val, str):
                                    self._exported.add(s_val)
                else:
                    self._assignments.append(
                        {
                            "name": target.id,
                            "type": "Assign",
                            "line": node.lineno,
                            "column": node.col_offset,
                        }
                    )
        self.generic_visit(node)

    def visit_AnnAssign(self, node: ast.AnnAssign) -> None:
        if isinstance(node.target, ast.Name):
            self._defined.add(node.target.id)
            self._assignments.append(
                {
                    "name": node.target.id,
                    "type": ast.unparse(node.annotation),
                    "line": node.lineno,
                    "column": node.col_offset,
                }
            )
        self.generic_visit(node)

    def visit_If(self, node: ast.If) -> None:
        self._control_flow_count += 1
        self.generic_visit(node)

    def visit_For(self, node: ast.For) -> None:
        self._control_flow_count += 1
        self.generic_visit(node)

    def visit_While(self, node: ast.While) -> None:
        self._control_flow_count += 1
        self.generic_visit(node)

    def visit_Try(self, node: ast.Try) -> None:
        self._control_flow_count += 1
        self.generic_visit(node)
