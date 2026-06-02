from __future__ import annotations

import re

from ..taxonomy import SymbolName, ResponseData, SymbolNameList, NamingError
from ..contract import INamingVariantPort


class PythonNamingVariantProvider(INamingVariantPort):
    """Implementation of identifier naming variants for Python."""

    def get_variant_dict(self, name: SymbolName) -> ResponseData | NamingError:
        """Produces common naming variants mapped by their type."""
        name_str = str(name)
        words = re.findall(
            r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+", name_str
        )
        words = [w.lower() for w in words]

        if not words:
            return ResponseData(
                value={
                    "snake_case": name_str,
                    "pascal_case": name_str,
                    "camel_case": name_str,
                    "screaming_snake": name_str.upper(),
                }
            )

        snake_case = "_".join(words)
        _first = str(words[0])
        _rest = [str(w).capitalize() for w in words[1:]]
        camel_case = _first + "".join(_rest)
        pascal_case = "".join(str(w).capitalize() for w in words)
        screaming_snake = snake_case.upper()

        res = {
            "snake_case": snake_case,
            "camel_case": camel_case,
            "pascal_case": pascal_case,
            "screaming_snake": screaming_snake,
        }
        return ResponseData(value=res)

    def build_variants(self, name: SymbolName) -> SymbolNameList | NamingError:
        """Produce common naming variants for a given base variable/function name."""
        name_str = str(name)
        variants_data = self.get_variant_dict(name=name)
        if isinstance(variants_data, NamingError):
            return variants_data

        variants_dict = variants_data.value
        sc = str(variants_dict.get("snake_case", name_str))
        ss = str(variants_dict.get("screaming_snake", name_str.upper()))
        cc = str(variants_dict.get("camel_case", name_str))
        pc = str(variants_dict.get("pascal_case", name_str))

        kebab_case = sc.replace("_", "-")
        variants_set = {name_str, sc, ss, cc, pc, kebab_case}
        return SymbolNameList(values=[SymbolName(value=v) for v in variants_set])
