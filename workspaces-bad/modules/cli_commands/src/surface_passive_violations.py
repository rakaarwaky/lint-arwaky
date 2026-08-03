# PURPOSE: Test AES0306 — surface passive with active logic, hierarchy violation
# Passive view component with >15 functions (active domain logic)

class PassiveViewComponent:
    pass

def helper_1(): pass
def helper_2(): pass
def helper_3(): pass
def helper_4(): pass
def helper_5(): pass
def helper_6(): pass
def helper_7(): pass
def helper_8(): pass
def helper_9(): pass
def helper_10(): pass
def helper_11(): pass
def helper_12(): pass
def helper_13(): pass
def helper_14(): pass
def helper_15(): pass
def helper_16(): pass
def helper_17(): pass  # >15 functions = AES0306

class SurfaceWithDomainLogic:
    def business_logic(self, data):
        # AES0306: passive surface with active domain logic
        result = []
        for item in data:
            if item > 0:
                if item % 2 == 0:
                    if item > 10:
                        result.append(item * 2)
                    else:
                        result.append(item)
                else:
                    result.append(item * 3)
            else:
                result.append(0)
        return result