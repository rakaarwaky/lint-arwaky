import os
import pytest
from auto_linter.taxonomy import FilePath, SymbolName, PrimitiveTypeList
from auto_linter.infrastructure.ast_py_scanner import ASTPythonParserAdapter
from auto_linter.infrastructure.ast_js_scanner import ASTJSParserAdapter

def test_ast_python_scanner_bridge(tmp_path):
    # Setup test file
    py_file = tmp_path / "test_module.py"
    py_file.write_text("""
import math
from sys import exit as sys_exit

class MyClass:
    def __init__(self):
        self.val = 42
        if self.val > 0:
            pass

def my_func():
    return 10
""")

    adapter = ASTPythonParserAdapter()
    
    # Verify imports
    imports = adapter.extract_imports(FilePath(value=str(py_file)))
    assert not hasattr(imports, "message") # not an error
    modules = [str(i.module) for i in imports.values]
    assert "math" in modules
    assert "sys.exit" in modules

    # Verify raw symbols
    symbols = adapter.get_raw_symbols(FilePath(value=str(py_file)))
    assert not hasattr(symbols, "message")
    assert "MyClass" in symbols.value["defined"]
    assert "my_func" in symbols.value["defined"]

    # Verify control flow
    count = adapter.get_control_flow_count(FilePath(value=str(py_file)))
    assert count.value >= 1

    # Verify class definitions
    cdefs = adapter.get_class_definitions(FilePath(value=str(py_file)))
    classes = cdefs.value["classes"]
    assert len(classes) == 1
    assert classes[0]["name"] == "MyClass"

def test_ast_js_scanner_bridge(tmp_path):
    js_file = tmp_path / "test_module.js"
    js_file.write_text("""
import { foo, bar as myBar } from './module';
const express = require('express');

class Calculator {
    constructor() {
        this.value = 0;
    }
    add(x) {
        if (x > 0) {
            this.value += x;
        }
    }
}
""")

    adapter = ASTJSParserAdapter()
    
    # Verify imports
    imports = adapter.extract_imports(FilePath(value=str(js_file)))
    assert not hasattr(imports, "message")
    modules = [str(i.module) for i in imports.values]
    assert "module.foo" in modules
    assert "module.bar" in modules
    assert "express" in modules

    # Verify raw symbols
    symbols = adapter.get_raw_symbols(FilePath(value=str(js_file)))
    assert not hasattr(symbols, "message")
    assert "Calculator" in symbols.value["defined"]

    # Verify class definitions
    cdefs = adapter.get_class_definitions(FilePath(value=str(js_file)))
    classes = cdefs.value["classes"]
    assert len(classes) == 1
    assert classes[0]["name"] == "Calculator"
