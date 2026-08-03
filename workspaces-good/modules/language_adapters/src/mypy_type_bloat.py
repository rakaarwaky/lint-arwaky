"""
Mypy Type Bloat — TEST PROJECT ONLY.
Intentionally triggers all listed mypy error codes.
BUKAN KODE PRODUKSI.
"""

from typing import (
    Any,
    Literal,
    TypedDict,
    overload,
    Union,
    List,
)

# =============================================================================
# import: import nonexistent module
# =============================================================================


# =============================================================================
# import-untyped: import without stubs
# =============================================================================


# =============================================================================
# no-untyped-def: function with no annotations
# =============================================================================
def no_annotations_func(x, y):  # mypy: no-untyped-def
    return x + y


class UntypedMethods:
    def method_no_annotations(self, a, b):  # mypy: no-untyped-def
        return a * b

    def another_untyped(self, value):  # mypy: no-untyped-def
        return str(value)


# =============================================================================
# no-untyped-call: calling function with no annotations in typed context
# =============================================================================
def typed_caller() -> str:
    result = no_annotations_func(1, 2)  # mypy: no-untyped-call
    return str(result)


# =============================================================================
# no-any-explicit: explicitly typed `Any`
# =============================================================================
data: Any = {"key": "value"}  # mypy: no-any-explicit

def process_any(value: Any) -> Any:  # mypy: no-any-explicit
    return value


# =============================================================================
# literal-required: passing str where Literal expected
# =============================================================================
def expect_literal(mode: Literal["a", "b", "c"]) -> str:
    return f"Mode: {mode}"


def call_with_wrong_literal() -> None:
    user_mode = "a"  # type: str
    expect_literal(user_mode)  # mypy: literal-required


# =============================================================================
# typeddict-item: wrong key
# =============================================================================
class UserDict(TypedDict):
    name: str
    age: int


def create_user() -> UserDict:
    return {
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com",  # mypy: typeddict-unknown-key
    }


def update_user(user: UserDict) -> UserDict:
    user["name"] = 123  # mypy: typeddict-item (wrong type for key)
    return user


# =============================================================================
# has-type: cannot determine type of variable
# =============================================================================
def conditional_type(x: Union[int, str]) -> None:
    if isinstance(x, int):
        reveal_type(x)  # mypy: has-type — but this is actually fine...
    # Use a forward reference to trigger has-type issues
    self_ref: "ForwardRef" = "not a ref"


# =============================================================================
# valid-type: invalid type syntax
# =============================================================================
def invalid_type_hint() -> None:
    x: "List[int" = [1, 2, 3]  # mypy: valid-type (unterminated bracket)
    y: "Optional[int" = None  # mypy: valid-type


# =============================================================================
# overload-overlap: overlapping overloads
# =============================================================================
@overload
def process_value(x: int) -> int:  # type: ignore[overload-overlap]
    ...


@overload
def process_value(x: int) -> str:  # mypy: overload-overlap (same input, different output)
    ...


def process_value(x: int) -> int | str:
    if x > 10:
        return x
    return str(x)


# =============================================================================
# syntax: syntax error in type annotation
# =============================================================================
def bad_syntax_annotation(x: """invalid type syntax""") -> None:  # mypy: syntax
    pass


def another_syntax_error(
    x: list[int, str],  # mypy: syntax (list takes only one param)
    y: dict[int]  # mypy: syntax (dict takes two params)
) -> None:
    pass


# =============================================================================
# str-bytes-safe: implicit str<->bytes
# =============================================================================
def str_bytes_mixing() -> None:
    text: str = "hello"
    binary: bytes = b"world"

    # Mypy should catch str/bytes mixing
    mixed: str = text + binary  # mypy: str-bytes-safe

    if text == binary:  # mypy: str-bytes-safe
        print("match")


# =============================================================================
# Additional: Discovery mode / incomplete annotations
# =============================================================================
# Generates extra mypy hits by using Any in various positions
def returns_list_and_none() -> list[None]:
    return [None, None, None]


# =============================================================================
# Final: Type errors in a chain
# =============================================================================
class TypedContainer(TypedDict, total=False):
    id: int
    label: str
    tags: List[str]


def create_container() -> TypedContainer:
    return {
        "id": "not_an_int",  # mypy: typeddict-item (wrong type)
        "label": 42,  # mypy: typeddict-item
        "tags": "not_a_list",  # mypy: typeddict-item
    }


# Forward reference that's never resolved
class SelfReferencing:
    def get_child(self) -> "Child":  # mypy: valid-type (forward ref to nothing)
        return SelfReferencing()

    def compare(self, other: "SelfReferencing") -> bool:  # fine
        return id(self) == id(other)
