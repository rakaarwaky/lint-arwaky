"""naming_variant_analyzer — Capability for generating naming convention variants."""

from ..taxonomy import ResponseData, SymbolName, SymbolNameList

import re


from ..contract import INamingVariantProtocol


class NamingVariantAnalyzer(INamingVariantProtocol):
    """Business logic for transforming symbol names into various conventions."""

    def get_variant_dict(self, name: SymbolName) -> ResponseData:
        """
        Decomposes a name and returns a dictionary of common conventions.
        Pure logic, no infrastructure dependency.
        """
        n = str(name)
        # Split by caps, numbers or underscores
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
        """Returns a unique list of all possible naming variants including kebab-case."""
        n = str(name)
        d = self.get_variant_dict(name)
        variants = d.value

        kebab = variants["snake_case"].replace("_", "-")

        results = {
            n,
            variants["snake_case"],
            variants["camel_case"],
            variants["pascal_case"],
            variants["screaming_snake"],
            kebab,
        }
        return SymbolNameList(values=[SymbolName(value=s) for s in results])
