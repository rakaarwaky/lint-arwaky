// PURPOSE: Test AES024 — empty class in JS/TS

class EmptyProtocol {}

class AnotherEmpty extends EmptyProtocol {}

class RealClass {
    doSomething(): boolean {
        return true;
    }
}
