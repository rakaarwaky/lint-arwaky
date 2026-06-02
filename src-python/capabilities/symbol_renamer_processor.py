"""symbol_renamer_processor — Capability for project-wide symbol renaming."""

from ..taxonomy import Count, DirectoryPath, SymbolName, FileContentVO, FilePath

import re

from ..contract import IFileSystemPort, ISymbolRenamerProtocol


class SymbolRenamerProcessor(ISymbolRenamerProtocol):
    """Orchestrates renaming symbols across the entire codebase."""

    def __init__(self, fs_scanner: IFileSystemPort):
        self._fs = fs_scanner

    def rename_symbol(
        self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName
    ) -> Count:
        """
        Renames a symbol in the project, respecting comments and strings.
        """
        old = str(old_name)
        new = str(new_name)

        # Comprehensive regex for matching the symbol while ignoring strings/comments
        pattern = re.compile(
            rf"""
            (
                \"\"\"(?:\\.|[^\\])*?\"\"\" |
                \'\'\'(?:\\.|[^\\])*?\'\'\' |
                \"(?:\\.|[^\"\\])*\" |
                \'(?:\\.|[^\'\\])*\' |
                `(?:\\.|[^`\\])*` |
                \#[^\n]* |
                //[^\n]* |
                /\*(?:.|\n)*?\*/
            )
            |
            \b({re.escape(old)})\b
        """,
            re.VERBOSE | re.DOTALL,
        )

        def _replacer(match):
            if match.group(1) is not None:
                return match.group(1)
            return new

        modified_count = 0
        files = self._fs.walk(FilePath(value=str(root_dir)))

        for file_path in files:
            file_content = self._fs.read_text(file_path)
            source = file_content.value
            if not source or old not in source:
                continue

            new_source = pattern.sub(_replacer, source)
            if new_source != source:
                self._fs.write_text(file_path, FileContentVO(value=new_source))
                modified_count += 1

        return Count(value=modified_count)
