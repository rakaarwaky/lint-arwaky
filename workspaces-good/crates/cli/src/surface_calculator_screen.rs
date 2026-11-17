// Surface fixture exercising the unified surface self-import rule: `surface`
// is now allowed to import other `surface` files (row 10 of the 11-scope
// AES201 Import Matrix).
use crate::surface_calculator_command;
use calculator_shared::contract_calculator_aggregate::ICalculatorAggregate;

pub fn show(calc: &dyn ICalculatorAggregate) {
    surface_calculator_command::run(calc);
}
