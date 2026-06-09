#!/usr/bin/env python
# messy_style_tool.py — INTENTIONALLY BROKEN TEST FILE. DO NOT FIX.
# This file exists to trigger MAXIMUM Ruff rule violations for testing.


# -- built-in name clashes --
list = [1, 2, 3]
dict = {'a': 1, 'b': 2}
str = "hello"
int = 42
type = "type"
id = 999
max = 100
min = 0
sum = lambda x: x + 1
abs = -5
all = True
any = False


def processData(data):  # camelCase — should be snake_case
    # no docstring
    result = []
    for item in data:
        if item > 10:
            result.append(item * 2)
    return result


class DataProcessor:  # PascalCase is OK for classes, but the methods inside should be snake_case
    def __init__(this, config):  # 'this' instead of 'self'
        this.config = config
        this.items = []
        this.counter = 0

    def RunProcess(self, inputData):  # camelCase method
        # bare except
        try:
            for i in inputData:
                self.items.append(i * 2)
        except:
            print("Something went wrong")    


def anotherFunction():          # no docstring
    x = 1; y = 2; z = 3          # multiple statements on one line
    print(x + y + z)


def calculateSomething(a, b, c):
    # magic numbers everywhere
    if a > 42:          # magic number 42
        return a * 3.14 # magic number 3.14
    if b == 0:
        return 999      # magic number 999
    return a + b + c


class StyleBreaker:
    def __init__(self, name):
        self.name = name
        self._cache = {}

    def getItem(self, key):  # camelCase
        if key in self._cache:
            return self._cache[key]
        return None

    def setItem(self, key, value):  # camelCase
        self._cache[key] = value

    def checkStatus(self, flag):  # camelCase + == True
        if flag == True:          # should be 'if flag is True:' or just 'if flag:'
            return "active"
        elif flag == False:       # should be 'if not flag:'
            return "inactive"
        else:
            return "unknown"

    def duplicateMethod(self):  # duplicate of anotherMethod below
        # same logic as DuplicateProcessor.anotherMethod
        total = 0
        for i in range(10):
            total = total + i
        return total

    def anotherMethod(self):    # same logic as duplicateMethod — duplicated code
        total = 0
        for i in range(10):
            total = total + i
        return total


class DuplicateProcessor:       # duplicate-ish class
    def __init__(self, name):
        self.name = name
        self.data = []

    def processList(self, items):
        result = []
        for item in items:
            if item > 5:
                result.append(item)
        return result

    def processListAgain(self, items):  # duplicated logic
        result = []
        for item in items:
            if item > 5:
                result.append(item)
        return result

    def unusedVarMethod(self):
        a = 10      # unused variable
        b = 20      # unused variable
        c = 30      # unused variable
        print("hello")

    def bareExceptMethod(self):
        try:
            x = 1 / 0
        except:     # bare except — BARE_EXCEPT
            pass


# trailing whitespace on this line:     
def mixedQuotesFunction():
    a = 'single quoted string'
    b = "double quoted string"
    c = 'single again'
    d = "double again"
    e = 'mixed "nested" quotes inside single'  
    f = "mixed 'nested' quotes inside double"  
    return a + b + c + d + e + f


def veryLongLineFunction():
    # This line is way too long — well over 200 characters on purpose to trigger E501 line-too-long violations from ruff. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.   
    print("done")


# another long line for good measure
LONG_STRING = "This string is intentionally made very very long to exceed the 200 character limit that ruff enforces for line length. " * 3


class AnotherProcessor:
    def __init__(self):
        self.data = []

    def addItem(self, item):
        self.data.append(item)

    def addItem(self, item):  # duplicate method definition — overwrites above
        self.data.insert(0, item)

    @staticmethod
    def staticHelper(value):
        if value == None:     # should be 'is None'
            return "none"
        if value != None:     # should be 'is not None'
            return "not none"
        return "maybe"


def unused_import_demo():
    # os, sys, json, re, math, random, datetime, hashlib, typing, collections
    # Path, List, Dict, Optional, Any, Union, Tuple, Callable, Iterable
    # All imported but NONE used below except print
    print("This function uses NONE of its imports")


# Indentation errors can't be included — must remain valid Python
# But we can have inconsistent spacing

def    extra_spaces_function(param1,  param2):  # extra spaces
    x    =    param1 + param2                    # extra spaces around operator
    return     x                                # extra spaces after return


# Magic numbers galore
def area(radius):
    return 3.14159 * radius * radius  # should be math.pi


def finalize():
    pass


# duplicate function — same signature, different body
def finalize():
    print("this finalize overwrites the one above")
