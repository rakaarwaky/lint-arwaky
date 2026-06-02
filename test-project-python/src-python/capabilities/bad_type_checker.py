"""
BAD TYPE CHECKER — TEST PROJECT
=================================
This file is INTENTIONALLY BROKEN to test mypy linting.
Contains 15+ categories of type violations across 10+ functions.
DO NOT FIX. This is a test asset.
"""

from typing import List, Dict, Optional, Union, TypedDict, Literal, Callable, Any


# ── Violation 1: Fungsi tanpa type annotations sama sekali ──
def no_annotations(a, b):
    return a + b


# ── Violation 2: Fungsi dengan type annotation salah (return int tapi return str) ──
def wrong_return_type(x: int, y: int) -> int:
    return f"{x} + {y} = {x + y}"  # ERROR: return str instead of int


# ── Violation 3: Mismatch assignment (declare str lalu assign int) ──
def mismatch_assignment() -> str:
    name: str = "hello"
    name = 42  # ERROR: assigning int to str variable
    return name  # ERROR: returning int as str


# ── Violation 4: Variable tidak didefinisikan ──
def undefined_variable() -> str:
    return undefined_var  # ERROR: name 'undefined_var' is not defined


# ── Violation 5: Argumen salah tipe di call function ──
def takes_only_str(x: str) -> str:
    return x.upper()


def calls_with_wrong_arg() -> str:
    return takes_only_str(12345)  # ERROR: passing int instead of str


# ── Violation 6: Mengakses attribute yang tidak ada di object ──
class SimpleContainer:
    def __init__(self) -> None:
        self.value: int = 10


def missing_attribute_access() -> int:
    obj = SimpleContainer()
    return obj.nonexistent_attr  # ERROR: 'SimpleContainer' has no attribute 'nonexistent_attr'


# ── Violation 7: Union type mismatch ──
def union_mismatch() -> Union[str, int]:
    return True  # ERROR: bool is not a member of Union[str, int]


# ── Violation 8: Return Any dari function yang dideclare return spesifik ──
def returns_any() -> Any:
    return 99


def declared_specific_but_got_any() -> str:
    x: str = returns_any()  # ERROR: assigning Any to str (implicit)
    return x


# ── Violation 9: None ke variable non-optional ──
def none_to_non_optional() -> str:
    x: str = None  # ERROR: None is not assignable to str (non-optional)
    return x


# ── Violation 10: Index error (index non-indexable type) ──
def index_non_indexable() -> str:
    x: int = 500
    return x[0]  # ERROR: int is not indexable


# ── Violation 11: Operator tidak didukung ──
def unsupported_operator() -> str:
    a: str = "hello"
    b: int = 10
    return a - b  # ERROR: unsupported operand type(s) for -: 'str' and 'int'


# ── Violation 12: Override dengan tipe return berbeda dari parent ──
class Parent:
    def get_value(self) -> int:
        return 42


class Child(Parent):
    def get_value(self) -> str:  # ERROR: return type mismatch with parent (str vs int)
        return "forty-two"


# ── Violation 13: TypedDict key tidak ada ──
class PersonData(TypedDict):
    name: str
    age: int


def typeddict_missing_key() -> str:
    p: PersonData = {"name": "Alice", "age": 30}
    return p["address"]  # ERROR: key 'address' not in TypedDict 'PersonData'


# ── Violation 14: Missing return statement ──
def missing_return() -> int:
    print("I forgot to return something")  # ERROR: missing return statement
    # no return


# ── Violation 15: Type comparison tidak valid ──
def invalid_type_comparison(x: object) -> bool:
    return x == 42  # OK (runtime comparison works)


def invalid_isinstance_check() -> str:
    value: int = 100
    if isinstance(value, int):
        return "int"
    elif isinstance(value, str):
        return "str"
    return "unknown"


def literal_type_mismatch() -> Literal["red", "green", "blue"]:
    return "yellow"  # ERROR: "yellow" is not a valid literal value


# ── Violation 16: Callable signature mismatch ──
def callable_mismatch() -> None:
    fn: Callable[[int], str] = lambda x: x  # ERROR: lambda returns int, not str


# ── Violation 17: Dict key type violation ──
def dict_key_type_violation() -> Dict[str, int]:
    d: Dict[str, int] = {}
    d[100] = 200  # ERROR: int key instead of str key
    return d


# ── Violation 18: List type mismatch ──
def list_type_mismatch() -> List[int]:
    items: List[int] = [1, 2, 3]
    items.append("four")  # ERROR: appending str to List[int]
    return items


# ── Violation 19: Optional handling wrong──
def optional_wrong_usage() -> str:
    value: Optional[str] = None
    return value.upper()  # ERROR: None has no attribute 'upper' (need None check)


# ── Violation 20: Double violation — salah annotation + missing return ──
def double_violation(x: int, y: str) -> List[str]:
    # ERROR 1: return type is List[str] but we return...
    # ERROR 2: ...nothing (missing return)
    pass


# ── Violation 21: Inconsistent return in function ──
def inconsistent_return(x: int) -> str:
    if x > 0:
        return "positive"
    elif x == 0:
        return 0  # ERROR: returning int instead of str
    # ERROR: missing return for else case


# ── Violation 22: Override parameter type berbeda dari parent ──
class BaseProcessor:
    def process(self, data: int) -> int:
        return data * 2


class ChildProcessor(BaseProcessor):
    def process(self, data: str) -> str:  # ERROR: parameter type int -> str, return int -> str
        return data + data


# ── Violation 23: Nested Any dan None propagation ──
def nested_broken() -> int:
    data: Optional[Dict[str, int]] = None
    # Chained violations:
    # 1. None check missing
    # 2. Access on None
    return data["missing_key"] + 100  # ERROR: multiple violations in one line


# ── Violation 24: Operator invalid antara str dan list ──
def operator_str_list() -> str:
    a: str = "hello"
    b: list = [1, 2, 3]
    return a + b  # ERROR: can only concatenate str (not "list") to str


# ── Violation 25: Variable didefinisikan di cabang tidak terpakai ──
def branch_undefined_var(flag: bool) -> str:
    if flag:
        result = "yes"
    # ERROR: if flag is False, 'result' is not defined
    return result
