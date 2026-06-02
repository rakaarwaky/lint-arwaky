import textwrap
from auto_linter.capabilities.dispatch_routing_parser import DispatchRoutingParser
from auto_linter.taxonomy import ContentString

def test_strip_docstrings():
    parser = DispatchRoutingParser()
    code = textwrap.dedent("""
        def foo():
            \"\"\"This is a docstring.\"\"\"
            # This is a comment
            x = 1 '''And this'''
            return x
    """)
    stripped = parser.strip_docstrings(ContentString(value=code))
    assert 'This is a docstring' not in stripped.value
    assert 'This is a comment' not in stripped.value
    assert 'And this' not in stripped.value
    assert 'def foo():' in stripped.value
    assert 'return x' in stripped.value

def test_extract_class_methods_simple():
    parser = DispatchRoutingParser()
    code = textwrap.dedent("""
        class MyCapability:
            def method_one(self):
                pass
            
            async def method_two(self, data):
                return None
        
        class OtherClass:
            def execute(self):
                pass
    """)
    result = parser.extract_class_methods(ContentString(value=code))
    
    assert "MyCapability" in result.definitions
    assert "method_one" in result.definitions["MyCapability"].methods
    assert "method_two" in result.definitions["MyCapability"].methods
    
    assert "OtherClass" in result.definitions
    assert "execute" in result.definitions["OtherClass"].methods

def test_extract_class_methods_nested_and_dedent():
    parser = DispatchRoutingParser()
    code = textwrap.dedent("""
        class Outer:
            def outer_method(self):
                if True:
                    def inner_func():
                        pass
                pass
        
            class Inner:
                def inner_method(self):
                    pass
        
        def standalone():
            pass
        
        class AfterStandalone:
            def hello(self):
                pass
    """)
    result = parser.extract_class_methods(ContentString(value=code))
    
    assert "Outer" in result.definitions
    assert "outer_method" in result.definitions["Outer"].methods
    # inner_func should NOT be in Outer.methods (heuristic indent <= 8)
    # outer_method is at indent 4, inner_func is at indent 8. 
    # Wait, if Outer is at indent 0, outer_method is at 4, inner_func is at 8.
    # indent <= 8 matches. I should change the heuristic or the test.
    assert "inner_func" not in result.definitions["Outer"].methods
    
    assert "Inner" in result.definitions
    assert "inner_method" in result.definitions["Inner"].methods
    
    assert "AfterStandalone" in result.definitions
    assert "hello" in result.definitions["AfterStandalone"].methods

def test_brace_tracking():
    parser = DispatchRoutingParser()
    # Testing brace tracking for JS-like objects or dicts inside methods
    code = textwrap.dedent("""
        class Handler:
            def handle(self):
                data = {
                    "key": "value",
                    "nested": {
                        "a": 1
                    }
                }
                return data
        
            def another(self):
                pass
    """)
    result = parser.extract_class_methods(ContentString(value=code))
    assert "Handler" in result.definitions
    assert "handle" in result.definitions["Handler"].methods
    assert "another" in result.definitions["Handler"].methods
