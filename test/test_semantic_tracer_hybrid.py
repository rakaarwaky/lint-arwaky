import tempfile
from pathlib import Path
from auto_linter.taxonomy import SymbolName, FilePath, LineNumber, DirectoryPath
from auto_linter.infrastructure.naming_variant_provider import PythonNamingVariantProvider
from auto_linter.infrastructure.python_ast_tracer import PythonTracer
from auto_linter.infrastructure.javascript_call_tracer import JSTracer


def test_python_naming_variant_provider():
    provider = PythonNamingVariantProvider()
    
    # Test naming variant dict
    res = provider.get_variant_dict(SymbolName(value="my_custom_class"))
    assert not hasattr(res, "message")
    assert res.value["snake_case"] == "my_custom_class"
    assert res.value["camel_case"] == "myCustomClass"
    assert res.value["pascal_case"] == "MyCustomClass"
    assert res.value["screaming_snake"] == "MY_CUSTOM_CLASS"
    
    # Test build variants
    variants = provider.build_variants(SymbolName(value="my_custom_class"))
    assert not hasattr(variants, "message")
    vals = [str(v) for v in variants.values]
    assert "my_custom_class" in vals
    assert "myCustomClass" in vals
    assert "MyCustomClass" in vals
    assert "MY_CUSTOM_CLASS" in vals
    assert "my-custom-class" in vals


def test_python_scope_tracer(tmp_path):
    py_file = tmp_path / "test_scope.py"
    py_file.write_text("""
class MyBigClass:
    def method_one(self):
        pass

    async def method_two(self):
        x = 10
""")

    tracer = PythonTracer()
    
    # Line 3 is inside MyBigClass -> method_one
    scope = tracer.get_enclosing_scope(FilePath(value=str(py_file)), LineNumber(value=3))
    assert scope is not None
    assert str(scope) == "class MyBigClass -> def method_one"

    # Line 7 is inside MyBigClass -> method_two
    scope2 = tracer.get_enclosing_scope(FilePath(value=str(py_file)), LineNumber(value=7))
    assert scope2 is not None
    assert str(scope2) == "class MyBigClass -> def method_two"


def test_js_scope_tracer(tmp_path):
    js_file = tmp_path / "test_scope.js"
    js_file.write_text("""
class UserStore {
    constructor() {
        this.users = [];
    }
    addUser(user) {
        this.users.push(user);
    }
}
""")

    tracer = JSTracer()
    
    # Line 6 is inside class UserStore -> function addUser
    scope = tracer.get_enclosing_scope(FilePath(value=str(js_file)), LineNumber(value=6))
    assert scope is not None
    assert str(scope) == "class UserStore -> function addUser"


def test_call_chain_and_rename_py(tmp_path):
    dir_path = tmp_path / "src"
    dir_path.mkdir()
    
    file1 = dir_path / "caller.py"
    file1.write_text("""
from module import some_useful_function

def main():
    some_useful_function()
""")

    tracer = PythonTracer()
    
    # Test trace call chain
    calls = tracer.trace_call_chain(DirectoryPath(value=str(tmp_path)), SymbolName(value="some_useful_function"))
    assert len(calls.values) == 1
    assert "caller.py:5" in calls.values[0].value
    
    # Test project-wide rename
    count = tracer.project_wide_rename(
        DirectoryPath(value=str(tmp_path)),
        SymbolName(value="some_useful_function"),
        SymbolName(value="another_nice_func")
    )
    assert count.value == 1
    
    content = file1.read_text()
    assert "another_nice_func" in content
    assert "some_useful_function" not in content
