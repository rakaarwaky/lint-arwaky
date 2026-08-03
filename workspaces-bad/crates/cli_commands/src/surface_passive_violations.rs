// PURPOSE: Test AES0306 — surface passive with active logic, hierarchy violation, >15 functions

// Passive view component with >15 functions = AES0306
pub struct SurfacePassiveView;

fn helper_1() {}
fn helper_2() {}
fn helper_3() {}
fn helper_4() {}
fn helper_5() {}
fn helper_6() {}
fn helper_7() {}
fn helper_8() {}
fn helper_9() {}
fn helper_10() {}
fn helper_11() {}
fn helper_12() {}
fn helper_13() {}
fn helper_14() {}
fn helper_15() {}
fn helper_16() {}  // >15 functions = AES0306
fn helper_17() {}

pub struct SurfaceWithDomainLogic;
impl SurfaceWithDomainLogic {
    pub fn business_logic(&self, data: &[i32]) -> Vec<i32> {
        // AES0306: passive surface with active domain logic (deep nesting)
        let mut result = Vec::new();
        for &item in data {
            if item > 0 {
                if item % 2 == 0 {
                    if item > 10 {
                        result.push(item * 2);
                    } else {
                        result.push(item);
                    }
                } else {
                    result.push(item * 3);
                }
            } else {
                result.push(0);
            }
        }
        result
    }
}