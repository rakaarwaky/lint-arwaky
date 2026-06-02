"""JSTracer - Naming variants for JavaScript/TypeScript symbols."""

from __future__ import annotations

import re
from typing import cast
from ..taxonomy import NameVariants, SymbolName
from ..contract import INamingProviderPort


class JavascriptNamingProvider(INamingProviderPort):
    """Implementation of naming variants for JavaScript/TypeScript symbols."""

    def get_variants(self, name: SymbolName) -> NameVariants:
        """Generate common naming variants (snake, camel, pascal, screaming, kebab)."""
        name_str = str(name)
        d = self._get_variant_dict(name_str)
        kebab = d["snake_case"].replace("_", "-")
        variants = {
            name_str,
            d["snake_case"],
            d["camel_case"],
            d["pascal_case"],
            d["screaming_snake"],
            kebab,
        }
        return NameVariants(values=list(variants))

    def _get_variant_dict(self, name: str) -> dict:
        """Produce common naming variants for a symbol name."""
        words = re.findall(r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+", name)
        words = cast(list[str], [w.lower() for w in words])
        if not words:
            return {
                "snake_case": name,
                "camel_case": name,
                "pascal_case": name,
                "screaming_snake": name,
            }
        snake_case = "_".join(words)
        first = str(words[0])
        rest = "".join(str(w).capitalize() for w in words[1:])
        return {
            "snake_case": snake_case,
            "camel_case": first + rest,
            "pascal_case": "".join(str(w).capitalize() for w in words),
            "screaming_snake": snake_case.upper(),
        }
