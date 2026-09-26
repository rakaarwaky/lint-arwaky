// PURPOSE: NameVariants, SymbolName — value objects for symbol naming and naming convention variants
use crate::list_wrapper_vo;
use crate::string_value_object;

string_value_object!(SymbolName);

list_wrapper_vo!(NameVariants, SymbolName);
