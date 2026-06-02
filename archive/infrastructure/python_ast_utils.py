"""python_ast_utils — Shared AST helper functions for Python analysis."""

from __future__ import annotations
import ast

from ..taxonomy import ContentString


class PythonASTUtils:
    """Shared AST utility functions for Python analysis."""

    @staticmethod
    def is_dead_class(node: ast.ClassDef) -> bool:
        """Check if a class is marked with _arch_dead_marker."""
        for item in node.body:
            if isinstance(item, ast.Assign):
                for target in item.targets:
                    if (
                        isinstance(target, ast.Name)
                        and target.id == "_arch_dead_marker"
                    ):
                        return True
            elif isinstance(item, ast.AnnAssign):
                if (
                    isinstance(item.target, ast.Name)
                    and item.target.id == "_arch_dead_marker"
                ):
                    return True
        return False

    @staticmethod
    def get_annotation_type_name(node: ast.AST | None) -> ContentString:
        """Extract a human-readable type name from an AST annotation node."""
        if node is None:
            return ContentString(value="Unknown")
        if isinstance(node, ast.Name):
            return ContentString(value=node.id)
        if isinstance(node, ast.Subscript):
            return ContentString(
                value=f"{PythonASTUtils.get_annotation_type_name(node.value)}[{PythonASTUtils.get_annotation_type_name(node.slice)}]"
            )
        if isinstance(node, ast.Attribute):
            return ContentString(
                value=f"{PythonASTUtils.get_annotation_type_name(node.value)}.{node.attr}"
            )
        if isinstance(node, ast.Constant):
            return ContentString(value=str(node.value))
        return PythonASTUtils._get_complex_annotation_name(node)

    @staticmethod
    def _get_complex_annotation_name(node: ast.AST) -> ContentString:
        """Handle more complex AST nodes for type annotations."""
        if isinstance(node, ast.Tuple):
            return ContentString(
                value=f"tuple[{', '.join(str(PythonASTUtils.get_annotation_type_name(e)) for e in node.elts)}]"
            )
        if isinstance(node, ast.List):
            return ContentString(
                value=f"list[{', '.join(str(PythonASTUtils.get_annotation_type_name(e)) for e in node.elts)}]"
            )
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.BitOr):
            return ContentString(
                value=f"{PythonASTUtils.get_annotation_type_name(node.left)} | {PythonASTUtils.get_annotation_type_name(node.right)}"
            )
        return ContentString(value=ast.unparse(node))

    @staticmethod
    def elt_str(node: ast.AST) -> ContentString | None:
        """Convert an AST element (like in __all__) to a string."""
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            return ContentString(value=node.value)
        if isinstance(node, ast.Name):
            return ContentString(value=node.id)
        return None

    @staticmethod
    def is_type_checking_guard(node: ast.If) -> bool:
        """Check if an if-statement is a TYPE_CHECKING guard."""
        if isinstance(node.test, ast.Name) and node.test.id == "TYPE_CHECKING":
            return True
        if isinstance(node.test, ast.Attribute) and node.test.attr == "TYPE_CHECKING":
            return True
        return False
