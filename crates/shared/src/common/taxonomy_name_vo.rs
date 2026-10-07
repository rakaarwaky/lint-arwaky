// PURPOSE: SymbolName — value object for symbol naming.
// Empty-name rejection: `string_value_object!` emits a `new` constructor
// that errors on empty input, keeping the contract surface type-safe.
use crate::string_value_object;

string_value_object!(SymbolName);
