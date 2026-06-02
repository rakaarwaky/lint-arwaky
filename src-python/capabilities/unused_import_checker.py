"""unused_import_checker — Capability for detecting unused imports."""

from ..taxonomy import FilePath, ImportNameList

from ..contract import ISourceParserPort, IUnusedImportProtocol


class UnusedImportRuleChecker(IUnusedImportProtocol):
    """Business logic for identifying imports that are not utilized in the code."""

    def __init__(self, parser: ISourceParserPort):
        self._parser = parser

    def find_unused_imports(self, path: FilePath) -> ImportNameList:
        """
        Orchestrates the detection of unused imports by comparing
        imported symbols against used and exported symbols.
        """
        symbols = self._parser.get_raw_symbols(path)

        # Keys aligned with ASTPythonParserAdapter ResponseData
        imported_aliases = symbols.value.get("aliases", {})
        defined_symbols = set(symbols.value.get("defined", []))
        exported_symbols = set(symbols.value.get("exported", []))
        used_symbols = set(symbols.value.get("used", []))

        # Helper to check presence regardless of str/SymbolName VO
        def _is_present(target: str, collection: set) -> bool:
            if target in collection:
                return True
            # Fallback for SymbolName VOs in the set
            for item in collection:
                if hasattr(item, "value") and item.value == target:
                    return True
            return False

        unused = []
        for alias, fullname in imported_aliases.items():
            # An import is unused if it's not referenced,
            # not in __all__ (exported), and not redefined locally.
            if not _is_present(alias, used_symbols) and not _is_present(
                alias, exported_symbols
            ):
                # Full name check for cases where internal logic uses the full path
                if not _is_present(fullname, defined_symbols):
                    unused.append(alias)

        return ImportNameList(values=unused)
