"""python_primitive_checker — Analyzer for primitive type usage (AES006)."""

from __future__ import annotations
import ast
from ..taxonomy import (
    FilePath,
    LineNumber,
    ColumnNumber,
    PrimitiveTypeList,
    PrimitiveViolation,
    PrimitiveViolationList,
    PrimitiveTypeName,
)


class PrimitiveChecker:
    """Specialized analyzer for detecting primitive type usage violations."""

    def find_primitive_violations(
        self, path: FilePath, tree: ast.AST, primitive_types: PrimitiveTypeList
    ) -> PrimitiveViolationList:
        results: list[PrimitiveViolation] = []

        def _walk(node: ast.AST, in_function: bool = False):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                is_private = node.name.startswith("_") and node.name != "__init__"
                if not is_private:
                    self._check_function_def_primitives(node, primitive_types, results)
                    for item in node.body:
                        _walk(item, in_function=True)
                return
            elif isinstance(node, ast.ClassDef):
                self._check_class_def_primitives(node, primitive_types, results)
                for item in node.body:
                    _walk(item, in_function=False)
                return
            elif isinstance(node, ast.AnnAssign) and node.annotation:
                if not in_function:
                    is_private = False
                    if isinstance(node.target, ast.Name) and node.target.id.startswith(
                        "_"
                    ):
                        is_private = True
                    if not is_private:
                        self._check_ann_assign_primitives(
                            node, primitive_types, results
                        )
            elif isinstance(node, ast.TypeAlias):
                self._check_type_alias_primitives(node, primitive_types, results)
            elif isinstance(node, ast.Assign):
                self._check_assign_type_alias_primitives(node, primitive_types, results)
            elif isinstance(node, ast.Call):
                if not in_function:
                    self._check_call_primitives(node, primitive_types, results)

            for child in ast.iter_child_nodes(node):
                _walk(child, in_function)

        _walk(tree)
        return PrimitiveViolationList(values=results)

    def _check_function_def_primitives(
        self,
        node: ast.FunctionDef | ast.AsyncFunctionDef,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        all_args = []
        all_args.extend(node.args.posonlyargs)
        all_args.extend(node.args.args)
        if node.args.vararg:
            all_args.append(node.args.vararg)
        all_args.extend(node.args.kwonlyargs)
        if node.args.kwarg:
            all_args.append(node.args.kwarg)

        for arg in all_args:
            if arg.annotation:
                self._check_node_for_primitives(
                    arg.annotation, primitive_types, results
                )

        if node.returns:
            self._check_node_for_primitives(node.returns, primitive_types, results)

    def _check_ann_assign_primitives(
        self,
        node: ast.AnnAssign,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        self._check_node_for_primitives(node.annotation, primitive_types, results)

    def _check_type_alias_primitives(
        self,
        node: ast.TypeAlias,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        self._check_node_for_primitives(node.value, primitive_types, results)

    def _check_assign_type_alias_primitives(
        self,
        node: ast.Assign,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        for target in node.targets:
            if isinstance(target, ast.Name) and target.id[0].isupper():
                self._check_node_for_primitives(node.value, primitive_types, results)
                break

    def _check_class_def_primitives(
        self,
        node: ast.ClassDef,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        for base in node.bases:
            self._check_node_for_primitives(base, primitive_types, results)

    def _check_call_primitives(
        self,
        node: ast.Call,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        if isinstance(node.func, ast.Name) and node.func.id[0].isupper():
            for arg in node.args:
                self._check_node_for_primitives(arg, primitive_types, results)
            for kw in node.keywords:
                self._check_node_for_primitives(kw.value, primitive_types, results)

    def _check_node_for_primitives(
        self,
        node: ast.AST,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        if isinstance(node, ast.Name):
            self._check_name_for_primitive(node, primitive_types, results)
        elif isinstance(node, ast.Subscript):
            self._check_subscript_for_primitive(node, primitive_types, results)
        elif isinstance(node, ast.Attribute):
            self._check_node_for_primitives(node.value, primitive_types, results)
        elif isinstance(node, (ast.Tuple, ast.List)):
            for elt in node.elts:
                self._check_node_for_primitives(elt, primitive_types, results)
        elif isinstance(node, ast.BinOp) and isinstance(node.op, ast.BitOr):
            self._check_node_for_primitives(node.left, primitive_types, results)
            self._check_node_for_primitives(node.right, primitive_types, results)

    def _check_name_for_primitive(
        self,
        node: ast.Name,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        if node.id in primitive_types:
            results.append(
                PrimitiveViolation(
                    line=LineNumber(value=node.lineno),
                    column=ColumnNumber(value=node.col_offset),
                    type_name=PrimitiveTypeName(value=node.id),
                )
            )

    def _check_subscript_for_primitive(
        self,
        node: ast.Subscript,
        primitive_types: PrimitiveTypeList,
        results: list[PrimitiveViolation],
    ) -> None:
        self._check_node_for_primitives(node.value, primitive_types, results)
        if hasattr(node, "slice"):
            self._check_node_for_primitives(node.slice, primitive_types, results)
